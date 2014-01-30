#include "ast.h"

#include <map>
#include <vector>

#include <llvm/Analysis/Verifier.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>

#include "error.h"
#include "engine.h"

namespace ast {
namespace {
llvm::IRBuilder<> builder(llvm::getGlobalContext());

std::vector<std::map<std::string, llvm::AllocaInst*>> named_values;

llvm::AllocaInst* GetNamedValue(const std::string& name) {
  // NOTE: this iterates forwards suggesting that shadowing breaks things.
  for (const auto& m : named_values) {
    if (m.count(name)) {
      return m.at(name);
    }
  }
  return nullptr;
}

void SetNamedValue(const std::string& name, llvm::AllocaInst* alloca) {
  named_values.back().insert(std::make_pair(name, alloca));
}

void PushNamedValueScope() {
  named_values.push_back({});
}

void PopNamedValueScope() {
  named_values.pop_back();
}

llvm::AllocaInst* CreateEntryBlockAlloca(
    llvm::Function* function, const std::string& var) {
  llvm::IRBuilder<> tmp(
      &function->getEntryBlock(), function->getEntryBlock().begin());
  // TODO: collections
  return tmp.CreateAlloca(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()), 0, var.c_str());
}

llvm::Value* ToBool(llvm::Value* val) {
  return builder.CreateFCmpUNE(
      val,
      llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "booltmp");
}
}  // end namespace

llvm::Value* Real::Codegen() const {
  return llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(value_));
}

llvm::Value* Variable::Codegen() const {
  llvm::AllocaInst* val = GetNamedValue(name_);
  if (!val) {
    Error("Unknown variable name: ", name_);
    return nullptr;
  }
  return builder.CreateLoad(val, name_.c_str());
}

llvm::Value* BinaryOp::Codegen() const {
  // assign
  if (op_ == "=") {
    const Variable* lhs_expression = dynamic_cast<const Variable*>(lhs_);
    if (!lhs_expression) {
      Error("LHS of assignment must be a variable");
      return nullptr;
    }

    llvm::Value* v = rhs_->Codegen();
    if (!v) return nullptr;

    llvm::AllocaInst* var = GetNamedValue(lhs_expression->name());
    if (!var) {
      // create the variable
      llvm::Function* f = builder.GetInsertBlock()->getParent();

      var = CreateEntryBlockAlloca(f, lhs_expression->name());
      SetNamedValue(lhs_expression->name(), var);
    }

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
  else if (op_ == "<")
    return builder.CreateUIToFP(
        builder.CreateFCmpULT(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "<=")
    return builder.CreateUIToFP(
        builder.CreateFCmpULE(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == ">")
    return builder.CreateUIToFP(
        builder.CreateFCmpUGT(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == ">=")
    return builder.CreateUIToFP(
        builder.CreateFCmpUGE(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "==")
    return builder.CreateUIToFP(
        builder.CreateFCmpUEQ(l, r, "cmptmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  else if (op_ == "!=")
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
  else if (op_ == "xor")
    return builder.CreateUIToFP(
        builder.CreateXor(ToBool(l), ToBool(r), "xortmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");

  Error("unknown binary operator");
  return nullptr;
}

llvm::Value* UnaryOp::Codegen() const {
  llvm::Value* expr = expr_->Codegen();
  if (!expr) return nullptr;

  // TODO: check that expr_ is either 'real' or variable of type 'double'
  if (op_ == "not")
    return builder.CreateUIToFP(
        builder.CreateNot(ToBool(expr), "nottmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  Error("unknown unary operator");
  return nullptr;
}

llvm::Value* If::Codegen() const {
  llvm::Value* condition_value = condition_->Codegen();
  if (!condition_value) return nullptr;
  condition_value = ToBool(condition_value);

  llvm::Function* parent = builder.GetInsertBlock()->getParent();

  // create if and else blocks
  llvm::BasicBlock* if_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "if", parent);
  llvm::BasicBlock* else_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "else");
  llvm::BasicBlock* merge_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "merge");

  if (else_.empty())
    builder.CreateCondBr(condition_value, if_block, merge_block);
  else
    builder.CreateCondBr(condition_value, if_block, else_block);

  // emit if block
  builder.SetInsertPoint(if_block);

  PushNamedValueScope();
  for (const Expression* e : if_) {
    llvm::Value* value = e->Codegen();
    if (!value) return nullptr;
  }
  PopNamedValueScope();

  builder.CreateBr(merge_block);
  if_block = builder.GetInsertBlock();

  // emit else block
  if (!else_.empty()) {
    parent->getBasicBlockList().push_back(else_block);
    builder.SetInsertPoint(else_block);

    PushNamedValueScope();
    for (const Expression* e : else_) {
      llvm::Value* value = e->Codegen();
      if (!value) return nullptr;
    }
    PopNamedValueScope();

    builder.CreateBr(merge_block);
    else_block = builder.GetInsertBlock();
  }

  parent->getBasicBlockList().push_back(merge_block);
  builder.SetInsertPoint(merge_block);

  return llvm::Constant::getNullValue(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}

llvm::Function* Program::Codegen() const {
  PushNamedValueScope();
  // prototype
  // TODO: return array of doubles
  llvm::FunctionType* ft = llvm::FunctionType::get(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()),
      std::vector<llvm::Type*>(),
      false);
  llvm::Function* f = llvm::Function::Create(
      ft, llvm::Function::ExternalLinkage, "global", engine::module);

  if (f->getName() != "global") {
    f->eraseFromParent();
    Error("Failed to create function");
    return nullptr;
  }

  // function body
  llvm::BasicBlock* bb = llvm::BasicBlock::Create(
      llvm::getGlobalContext(), "entry", f);
  builder.SetInsertPoint(bb);

  llvm::Value* return_value = nullptr;
  for (const Expression* e : body_) {
    // TODO: find the return expressions.
    return_value = e->Codegen();
    if (!return_value) {
      f->eraseFromParent();
      return nullptr;
    }
  }

  // TODO: return should be a statement that codegens to this.
  builder.CreateRet(return_value);

  PopNamedValueScope();

  llvm::verifyFunction(*f);
  return f;
}
}  // end namespace ast
