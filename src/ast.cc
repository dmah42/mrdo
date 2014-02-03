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
  named_values.push_back({
  });
}

void PopNamedValueScope() { named_values.pop_back(); }

llvm::AllocaInst* CreateEntryBlockAlloca(llvm::Function* function,
                                         llvm::Type* type,
                                         const std::string& var) {
  llvm::IRBuilder<> tmp(&function->getEntryBlock(),
                        function->getEntryBlock().begin());
  return tmp.CreateAlloca(type, nullptr, var.c_str());
}

llvm::Value* ToBool(llvm::Value* val) {
  return builder.CreateFCmpUNE(
      val, llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "booltmp");
}
}  // end namespace

llvm::Value* Real::Codegen() const {
  return llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(value_));
}

llvm::Value* Collection::Codegen() const {
  std::vector<llvm::Constant*> init_values;
  for (const Expression* e : values_) {
    llvm::Value* val = e->Codegen();
    const Real* e_real = dynamic_cast<const Real*>(e);
    const Collection* e_coll = dynamic_cast<const Collection*>(e);
    if (e_real) {
      init_values.push_back(llvm::cast<llvm::Constant>(val));
    } else if (e_coll) {
      // TODO
      Error(line, col, "Unimplemented collection of collection.");
      return nullptr;
    } else {
      Error(line, col, "Unimplemented expression type ", typeid(e).name(),
            " in collection.");
      return nullptr;
    }
  }

  return llvm::ConstantArray::get(
      llvm::ArrayType::get(llvm::Type::getDoubleTy(llvm::getGlobalContext()),
                           init_values.size()),
      init_values);
}

llvm::Value* Variable::Codegen() const {
  llvm::AllocaInst* val = GetNamedValue(name_);
  if (!val) {
    Error(line, col, "Unknown variable name: ", name_);
    return nullptr;
  }
  return builder.CreateLoad(val, name_.c_str());
}

llvm::Value* BinaryOp::Codegen() const {
  if (op_ == "=") return HandleAssign();

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

  Error(line, col, "Unknown binary operator: ", op_, ".");
  return nullptr;
}

llvm::Value* BinaryOp::HandleAssign() const {
  const Variable* lhs_variable = dynamic_cast<const Variable*>(lhs_);
  if (!lhs_variable) {
    Error(line, col, "LHS of assignment must be a variable.");
    return nullptr;
  }

  llvm::Value* v = rhs_->Codegen();
  if (!v) return nullptr;

  llvm::AllocaInst* var = GetNamedValue(lhs_variable->name());
  if (!var) {
    // create the variable
    llvm::Function* f = builder.GetInsertBlock()->getParent();
    const Collection* rhs_c = dynamic_cast<const Collection*>(rhs_);
    const Variable* rhs_v = dynamic_cast<const Variable*>(rhs_);
    llvm::Type* alloca_type = llvm::Type::getDoubleTy(llvm::getGlobalContext());
    if (rhs_c) {
      alloca_type = llvm::ArrayType::get(
          llvm::Type::getDoubleTy(llvm::getGlobalContext()), rhs_c->size());
    } else if (rhs_v) {
      alloca_type = v->getType();
    }
    var = CreateEntryBlockAlloca(f, alloca_type, lhs_variable->name());
    SetNamedValue(lhs_variable->name(), var);
  }

  builder.CreateStore(v, var);
  return v;
}

llvm::Value* UnaryOp::Codegen() const {
  llvm::Value* expr = expr_->Codegen();
  if (!expr) return nullptr;

  if (op_ == "not") {
    const Real* expr_real = dynamic_cast<const Real*>(expr_);
    const Variable* expr_var = dynamic_cast<const Variable*>(expr_);
    // TODO: check var type is not collection.
    if (!expr_real && !expr_var) {
      Error(line, col, "Expected real or variable of type real after 'not'.");
      return nullptr;
    }
    return builder.CreateUIToFP(
        builder.CreateNot(ToBool(expr), "nottmp"),
        llvm::Type::getDoubleTy(llvm::getGlobalContext()), "booltmp");
  }
  Error(line, col, "Unknown unary operator: ", op_, ".");
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
  // No return type, no parameters.
  llvm::FunctionType* ft =
      llvm::FunctionType::get(llvm::Type::getVoidTy(llvm::getGlobalContext()),
                              std::vector<llvm::Type*>(), false);
  llvm::Function* f = llvm::Function::Create(
      ft, llvm::Function::ExternalLinkage, "global", engine::module);

  if (f->getName() != "global") {
    f->eraseFromParent();
    Error(lexer::line, lexer::col, "Failed to create function.");
    return nullptr;
  }

  // function body
  llvm::BasicBlock* bb =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "entry", f);
  builder.SetInsertPoint(bb);

  for (const Expression* e : body_) {
    // TODO: find the return expressions.
    llvm::Value* v = e->Codegen();
    if (!v) {
      f->eraseFromParent();
      return nullptr;
    }
  }

  builder.CreateRetVoid();

  PopNamedValueScope();

  llvm::verifyFunction(*f);
  return f;
}
}  // end namespace ast
