#include "ast/binary_op.h"

#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>

#include "ast.h"
#include "ast/collection.h"
#include "ast/variable.h"
#include "error.h"

namespace ast {
namespace {
llvm::AllocaInst* CreateEntryBlockAlloca(llvm::Function* function,
                                         llvm::Type* type,
                                         const std::string& var) {
  llvm::IRBuilder<> tmp(&function->getEntryBlock(),
                        function->getEntryBlock().begin());
  return tmp.CreateAlloca(type, nullptr, var.c_str());
}
}  // end namespace

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
      alloca_type = llvm::PointerType::getUnqual(llvm::ArrayType::get(
          llvm::Type::getDoubleTy(llvm::getGlobalContext()), rhs_c->size()));
    } else if (rhs_v) {
      alloca_type = v->getType();
    }
    var = CreateEntryBlockAlloca(f, alloca_type, lhs_variable->name());
    SetNamedValue(lhs_variable->name(), var);
  }

  builder.CreateStore(v, var);
  return v;
}
}  // end namespace ast
