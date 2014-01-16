#include "ast.h"

#include <iostream>
#include <map>

#include <llvm/Analysis/Verifier.h>
#include <llvm/DerivedTypes.h>
#include <llvm/IRBuilder.h>
#include <llvm/Function.h>
#include <llvm/LLVMContext.h>
#include <llvm/Module.h>
#include <llvm/PassManager.h>

#include "engine.h"

namespace ast {

llvm::IRBuilder<> builder(llvm::getGlobalContext());
std::map<std::string, llvm::Value*> named_values;

llvm::Value* ErrorV(const std::string& str) {
  std::cerr << "Error: " << str << "\n";
  return nullptr;
}

llvm::Value* Number::Codegen() const {
  return llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(value_));
}

llvm::Value* Variable::Codegen() const {
  if (named_values.count(name_) == 0) {
    std::cerr << "Error: Unknown variable name: " << name_ << "\n";
    return nullptr;
  }
  return named_values[name_];
}

llvm::Value* Binary::Codegen() const {
  llvm::Value* l = lhs_->Codegen();
  llvm::Value* r = rhs_->Codegen();
  if (!l || !r) return nullptr;

  switch (op_) {
    case '+':
      return builder.CreateFAdd(l, r, "addtmp");
    case '-':
      return builder.CreateFSub(l, r, "subtmp");
    case '*':
      return builder.CreateFMul(l, r, "multmp");
    case '<': {
      return builder.CreateUIToFP(
            builder.CreateFCmpULT(l, r, "cmptmp"),
            llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
    }
    default:
      return ErrorV("unknown binary operator");
  }
}

llvm::Value* Call::Codegen() const {
  llvm::Function* callee_func = engine::module->getFunction(callee_);
  if (!callee_func)
    return ErrorV("attempt to call unknown function");

  if (callee_func->arg_size() != args_.size())
    return ErrorV("argument length mismatch");

  std::vector<llvm::Value*> argv;
  for (const auto& arg : args_) {
    argv.push_back(arg->Codegen());
    if (!argv.back()) return nullptr;
  }

  return builder.CreateCall(callee_func, argv, "calltmp");
}

llvm::Value* If::Codegen() const {
  llvm::Value* condition_value = condition_->Codegen();
  if (!condition_value) return nullptr;

  // convert double to bool
  condition_value = builder.CreateFCmpONE(
      condition_value,
      llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "ifcond");

  llvm::Function* parent = builder.GetInsertBlock()->getParent();

  // Create blocks for 'if' and 'else'
  llvm::BasicBlock* if_block = llvm::BasicBlock::Create(llvm::getGlobalContext(), "if", parent);
  llvm::BasicBlock* else_block = llvm::BasicBlock::Create(llvm::getGlobalContext(), "else");
  llvm::BasicBlock* merge_block = llvm::BasicBlock::Create(llvm::getGlobalContext(), "ifcont");

  builder.CreateCondBr(condition_value, if_block, else_block);

  // emit if block
  builder.SetInsertPoint(if_block);

  llvm::Value* if_value = if_->Codegen();
  if (!if_value) return nullptr;

  builder.CreateBr(merge_block);
  if_block = builder.GetInsertBlock();

  // emit else block
  parent->getBasicBlockList().push_back(else_block);
  builder.SetInsertPoint(else_block);

  llvm::Value* else_value = else_->Codegen();
  if (!else_value) return nullptr;

  builder.CreateBr(merge_block);
  else_block  = builder.GetInsertBlock();

  // emit merge block
  parent->getBasicBlockList().push_back(merge_block);
  builder.SetInsertPoint(merge_block);
  llvm::PHINode* phi = builder.CreatePHI(llvm::Type::getDoubleTy(llvm::getGlobalContext()), 2, "iftmp");
  phi->addIncoming(if_value, if_block);
  phi->addIncoming(else_value, else_block);
  return phi;
}

llvm::Value* For::Codegen() const {
  llvm::Value* start_value = start_->Codegen();
  if (!start_value) return nullptr;

  llvm::Function* parent = builder.GetInsertBlock()->getParent();
  llvm::BasicBlock* preheader_block = builder.GetInsertBlock();
  llvm::BasicBlock* loop_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "loop", parent);

  builder.CreateBr(loop_block);

  builder.SetInsertPoint(loop_block);

  llvm::PHINode* variable = builder.CreatePHI(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()), 2, var_.c_str());
  variable->addIncoming(start_value, preheader_block);

  // Within the loop, the variable is defined equal to the PHI node. This allows
  // shadowing of existing variables.
  llvm::Value* old_value = named_values[var_];
  named_values[var_] = variable;

  if (!body_->Codegen()) return nullptr;

  // emit start value
  llvm::Value* step_value;
  if (step_) {
    step_value = step_->Codegen();
    if (!step_value) return nullptr;
  } else {
    // default step to 1.0
    step_value = llvm::ConstantFP::get(
        llvm::getGlobalContext(), llvm::APFloat(1.0));
  }

  llvm::Value* next_var = builder.CreateFAdd(variable, step_value, "nextvar");

  // compute end condition
  llvm::Value* end_cond = end_->Codegen();
  if (!end_cond) return nullptr;

  // convert to bool
  end_cond =
    builder.CreateFCmpONE(end_cond,
                          llvm::ConstantFP::get(llvm::getGlobalContext(),
                                                llvm::APFloat(0.0)),
                          "loopcond");

  // create the after loop
  llvm::BasicBlock* loop_end_block = builder.GetInsertBlock();
  llvm::BasicBlock* after_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "afterloop", parent);

  builder.CreateCondBr(end_cond, loop_block, after_block);
  builder.SetInsertPoint(after_block);

  variable->addIncoming(next_var, loop_end_block);

  // restore the shadowed variable
  if (old_value)
    named_values[var_] = old_value;
  else
    named_values.erase(var_);

  // for always returns 0.0
  return llvm::Constant::getNullValue(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}

llvm::Function* Prototype::Codegen() const {
  llvm::FunctionType* ft = llvm::FunctionType::get(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()),
      std::vector<llvm::Type*>(
          args_.size(), llvm::Type::getDoubleTy(llvm::getGlobalContext())),
      false);
  llvm::Function* f = llvm::Function::Create(
      ft, llvm::Function::ExternalLinkage, name_, engine::module);

  if (f->getName() != name_) {
    f->eraseFromParent();
    f = engine::module->getFunction(name_);

    if (!f->empty()) {
      ErrorV("redefinition of function");
      return nullptr;
    }

    if (f->arg_size() != args_.size()) {
      ErrorV("redefinition of function with different arg length");
      return nullptr;
    }
  }

  llvm::Function::arg_iterator ai = f->arg_begin();
  for (size_t i = 0; i != args_.size(); ++i) {
    ai->setName(args_[i]);
    named_values[args_[i]] = ai;
    ++ai;
  }
  return f;
}

llvm::Function* Function::Codegen() const {
  named_values.clear();

  llvm::Function* f = prototype_->Codegen();
  if (!f) return nullptr;

  llvm::BasicBlock* bb = llvm::BasicBlock::Create(
      llvm::getGlobalContext(), "entry", f);
  builder.SetInsertPoint(bb);

  llvm::Value* return_value = body_->Codegen();
  if (!return_value) {
    f->eraseFromParent();
    return nullptr;
  }

  builder.CreateRet(return_value);
  llvm::verifyFunction(*f);
  engine::fpm->run(*f);
  return f;
}

}
