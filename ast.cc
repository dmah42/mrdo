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
namespace {
llvm::IRBuilder<> builder(llvm::getGlobalContext());

// A vector of maps means we can push and pop scope easily.
std::vector<std::map<std::string, llvm::AllocaInst*>> named_values;

llvm::AllocaInst* CreateEntryBlockAlloca(llvm::Function* function,
                                         const std::string& var) {
  llvm::IRBuilder<> tmp(&function->getEntryBlock(),
                        function->getEntryBlock().begin());
  return tmp.CreateAlloca(llvm::Type::getDoubleTy(llvm::getGlobalContext()), 0,
                          var.c_str());
}

llvm::Value* ToBool(llvm::Value* val) {
  return builder.CreateFCmpUNE(
      val,
      llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "booltmp");
}

llvm::Value* ErrorV(const std::string& str) {
  std::cerr << "Error: " << str << "\n";
  return nullptr;
}

void EnterScope() {
  named_values.push_back({});
}

void ExitScope() {
  // TODO: clean up memory.
  named_values.back().clear();
  named_values.pop_back();
}
}  // end namespace

void Initialize() {
  // Add global scope
  EnterScope();
}

llvm::Value* Number::Codegen() const {
  return llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(value_));
}

llvm::Value* Variable::Codegen() const {
  if (named_values.back().count(name_) == 0) {
    std::cerr << "Error: Unknown variable name: " << name_ << "\n";
    return nullptr;
  }
  return builder.CreateLoad(named_values.back()[name_], name_.c_str());
}

llvm::Value* Binary::Codegen() const {
  if (op_ == "=") {
    // Require LHS to be identifier
    Variable* lhs_expression = dynamic_cast<Variable*>(lhs_);
    if (!lhs_expression) return ErrorV("destination of '=' must be a variable");

    llvm::Value* v = rhs_->Codegen();
    if (!v) return nullptr;

    llvm::Value* var = named_values.back()[lhs_expression->name()];
    if (!var) return ErrorV("unknown variable name");

    builder.CreateStore(v, var);
    return v;
  }

  llvm::Value* l = lhs_->Codegen();
  llvm::Value* r = rhs_->Codegen();
  if (!l || !r) return nullptr;

  if (op_ == "+")
    return builder.CreateFAdd(l, r, "addtmp");
  else if (op_ == "-")
    return builder.CreateFSub(l, r, "subtmp");
  else if (op_ == "*")
    return builder.CreateFMul(l, r, "multmp");
  else if (op_ == "/")
    return builder.CreateFDiv(l, r, "divtmp");
  else if (op_ == "lt")
    return builder.CreateUIToFP(
        builder.CreateFCmpULT(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "le")
    return builder.CreateUIToFP(
        builder.CreateFCmpULE(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "gt")
    return builder.CreateUIToFP(
        builder.CreateFCmpUGT(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "ge")
    return builder.CreateUIToFP(
        builder.CreateFCmpUGE(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "eq")
    return builder.CreateUIToFP(
        builder.CreateFCmpUEQ(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "ne")
    return builder.CreateUIToFP(
        builder.CreateFCmpUNE(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "or")
    return builder.CreateUIToFP(
        builder.CreateOr(ToBool(l), ToBool(r), "ortmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "and")
    return builder.CreateUIToFP(
        builder.CreateAnd(ToBool(l), ToBool(r), "andtmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else
    return ErrorV("unknown binary operator");
}

llvm::Value* Unary::Codegen() const {
  llvm::Value* expr = expression_->Codegen();
  if (!expr) return nullptr;

  if (op_ == "not")
    return builder.CreateUIToFP(
        builder.CreateNot(ToBool(expr), "nottmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else
    return ErrorV("unknown unary operator");
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

  if (else_.empty())
    builder.CreateCondBr(condition_value, if_block, merge_block);
  else
    builder.CreateCondBr(condition_value, if_block, else_block);

  // emit if block
  builder.SetInsertPoint(if_block);

  EnterScope();
  for (const ast::Expression* e : if_) {
    llvm::Value* value = e->Codegen();
    if (!value) return nullptr;
  }
  ExitScope();

  builder.CreateBr(merge_block);
  if_block = builder.GetInsertBlock();

  if (!else_.empty()) {
    parent->getBasicBlockList().push_back(else_block);
    // emit else block
    builder.SetInsertPoint(else_block);

    EnterScope();
    for (const ast::Expression* e : else_) {
      llvm::Value* value = e->Codegen();
      if (!value) return nullptr;
    }
    ExitScope();

    builder.CreateBr(merge_block);
    else_block = builder.GetInsertBlock();
  }

  parent->getBasicBlockList().push_back(merge_block);
  builder.SetInsertPoint(merge_block);

  // if always returns 0.0
  return llvm::Constant::getNullValue(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}

llvm::Value* For::Codegen() const {
  // var = alloca real
  // ...
  // start = startexpr
  // store start -> var
  // goto loop
  //loop:
  // ...
  // body
  // ...
  //loopend:
  //  step = stepexpr
  //  endcond = endexpr
  //
  //  curvar = load var
  //  nextvar = curvar + step
  //  store nextvar -> var
  //  br endcond, loop, endloop
  //outloop:

  llvm::Function* parent = builder.GetInsertBlock()->getParent();

  llvm::AllocaInst* alloca = CreateEntryBlockAlloca(parent, var_);

  llvm::Value* start_value = start_->Codegen();
  if (!start_value) return nullptr;

  builder.CreateStore(start_value, alloca);

  llvm::BasicBlock* loop_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "loop", parent);

  builder.CreateBr(loop_block);

  builder.SetInsertPoint(loop_block);

  // Start a new scope for the loop.
  EnterScope();

  // Within the loop, the variable is defined equal to the PHI node. This allows
  // shadowing of existing variables.
  named_values.back().insert(std::make_pair(var_, alloca));

  for (const ast::Expression* e : body_)
    if (!e->Codegen()) return nullptr;

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

  // compute end condition
  llvm::Value* end_cond = end_->Codegen();
  if (!end_cond) return nullptr;

  // reload, increment, restore alloca.
  llvm::Value* current_var = builder.CreateLoad(alloca, var_.c_str());
  llvm::Value* next_var =
      builder.CreateFAdd(current_var, step_value, "nextvar");
  builder.CreateStore(next_var, alloca);

  // convert to bool
  end_cond =
    builder.CreateFCmpONE(end_cond,
                          llvm::ConstantFP::get(llvm::getGlobalContext(),
                                                llvm::APFloat(0.0)),
                          "loopcond");

  // create the after loop
  llvm::BasicBlock* after_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "afterloop", parent);

  builder.CreateCondBr(end_cond, loop_block, after_block);
  builder.SetInsertPoint(after_block);

  // remove the latest scope from the stack
  ExitScope();

  // for always returns 0.0
  return llvm::Constant::getNullValue(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}

llvm::Value* Var::Codegen() const {
  llvm::Function* f = builder.GetInsertBlock()->getParent();

  // register var and emit initializer
  llvm::Value* init_value = nullptr;
  if (init_) {
    init_value = init_->Codegen();
    if (!init_value) return nullptr;
  } else {
    init_value = llvm::ConstantFP::get(
        llvm::getGlobalContext(), llvm::APFloat(0.0));
  }

  llvm::AllocaInst* alloca = CreateEntryBlockAlloca(f, name_);
  builder.CreateStore(init_value, alloca);

  // TODO: warn on shadowing
  if (named_values.back().count(name_) != 0)
    return ErrorV("variable of that name already exists");
  
  named_values.back().insert(std::make_pair(name_, alloca));

  return init_value; 
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
    ++ai;
  }
  return f;
}

llvm::Function* Function::Codegen() const {
  llvm::Function* f = prototype_->Codegen();
  if (!f) return nullptr;

  llvm::BasicBlock* bb = llvm::BasicBlock::Create(
      llvm::getGlobalContext(), "entry", f);
  builder.SetInsertPoint(bb);

  EnterScope();
  CreateArgumentAllocas(f);

  // return value is value of last expression in function.
  llvm::Value* return_value = nullptr;

  for (const Expression* e : body_) { 
    return_value = e->Codegen();
    if (!return_value) {
      f->eraseFromParent();
      ExitScope();
      return nullptr;
    }
  }

  // TODO: return value for filter, no return for map
  builder.CreateRet(return_value);

  ExitScope();

  llvm::verifyFunction(*f);
  engine::fpm->run(*f);
  return f;
}

void Function::CreateArgumentAllocas(llvm::Function* f) const {
  llvm::Function::arg_iterator ai = f->arg_begin();
  for (const std::string& arg : prototype_->args()) {
    llvm::AllocaInst* alloca = CreateEntryBlockAlloca(f, arg);
    builder.CreateStore(ai, alloca);
    named_values.back().insert(std::make_pair(arg, alloca));
    ++ai;
  }
}
}  // end namespace ast
