#include "ast/binary_op.h"

#include <llvm/IR/IRBuilder.h>

#include "ast.h"
#include "ast/variable.h"
#include "error.h"
#include "llvm_type.h"

namespace ast {
llvm::Value* BinaryOp::Codegen() const {
  if (op_ == "=")
    return HandleAssign();

  llvm::Value* l = lhs_->Codegen();
  llvm::Value* r = rhs_->Codegen();
  if (!l || !r)
    return nullptr;

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
        builder.CreateFCmpULT(l, r, "cmptmp"), Type(), "booltmp");
  else if (op_ == "<=")
    return builder.CreateUIToFP(
        builder.CreateFCmpULE(l, r, "cmptmp"), Type(), "booltmp");
  else if (op_ == ">")
    return builder.CreateUIToFP(
        builder.CreateFCmpUGT(l, r, "cmptmp"), Type(), "booltmp");
  else if (op_ == ">=")
    return builder.CreateUIToFP(
        builder.CreateFCmpUGE(l, r, "cmptmp"), Type(), "booltmp");
  else if (op_ == "==")
    return builder.CreateUIToFP(
        builder.CreateFCmpUEQ(l, r, "cmptmp"), Type(), "booltmp");
  else if (op_ == "!=")
    return builder.CreateUIToFP(
        builder.CreateFCmpUNE(l, r, "cmptmp"), Type(), "booltmp");
  else if (op_ == "or")
    return builder.CreateUIToFP(
        builder.CreateOr(ToBool(l), ToBool(r), "ortmp"), Type(), "booltmp");
  else if (op_ == "and")
    return builder.CreateUIToFP(
        builder.CreateAnd(ToBool(l), ToBool(r), "andtmp"), Type(), "booltmp");
  else if (op_ == "xor")
    return builder.CreateUIToFP(
        builder.CreateXor(ToBool(l), ToBool(r), "xortmp"), Type(), "booltmp");

  Error(position, "Unknown binary operator: ", op_, ".");
  return nullptr;
}

llvm::Type* BinaryOp::Type() const { return TypeMap<double>::get(); }

llvm::Value* BinaryOp::HandleAssign() const {
  // TODO: test reassignment to same variable
  const Variable* lhs_variable = dynamic_cast<const Variable*>(lhs_);
  if (!lhs_variable) {
    Error(position, "LHS of assignment must be a variable.");
    return nullptr;
  }

  llvm::Value* v = rhs_->Codegen();
  if (!v)
    return nullptr;

  llvm::AllocaInst* var = GetNamedValue(lhs_variable->name());
  if (!var) {
    // create the variable
    llvm::Function* f = builder.GetInsertBlock()->getParent();
    var = CreateNamedVariable(f, lhs_variable->name(), rhs_);
    if (!var) {
      Error(position, "Failed to create variable ", lhs_variable->name());
      return nullptr;
    }
  }

  if (var->getAllocatedType()->getTypeID() != v->getType()->getTypeID()) {
    // TODO: better error message to catch type mismatch on reassignment.
    Error(position,
          "Attempting to store ",
          v->getType()->getTypeID(),
          " in variable of type ",
          var->getAllocatedType()->getTypeID(),
          ": ");
    v->getType()->dump();
    var->getAllocatedType()->getTypeID();
    return nullptr;
  }

  builder.CreateStore(v, var);
  return v;
}
}  // end namespace ast
