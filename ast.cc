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
  if (named_values.count(name_) == 0)
    return ErrorV("Unknown variable name");
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
