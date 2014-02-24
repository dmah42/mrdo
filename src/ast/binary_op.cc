#include "ast/binary_op.h"

#include <llvm/IR/IRBuilder.h>

#include "ast.h"
#include "ast/variable.h"
#include "error.h"

namespace ast {
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
  // TODO: test reassignment to same variable
  const Variable* lhs_variable = dynamic_cast<const Variable*>(lhs_);
  if (!lhs_variable) {
    Error(line, col, "LHS of assignment must be a variable.");
    return nullptr;
  }

  llvm::Value* v = nullptr;

  llvm::AllocaInst* var = GetNamedValue(lhs_variable->name());
  if (!var) {
    // create the variable
    llvm::Function* f = builder.GetInsertBlock()->getParent();
    std::pair<llvm::AllocaInst*, llvm::Value*> var_v =
        CreateNamedVariable(f, lhs_variable->name(), rhs_);
    var = var_v.first;
    v = var_v.second;
    if (!var) {
      Error(line, col, "Failed to create variable ", lhs_variable->name());
      return nullptr;
    }
  } else {
    v = rhs_->Codegen();
    if (!v) return nullptr;
  }

  if (var->getAllocatedType()->getTypeID() != v->getType()->getTypeID()) {
    // TODO: better error message to catch type mismatch on reassignment.
    Error(line, col, "Attempting to store ", v->getType()->getTypeID(),
          " in variable of type ", var->getAllocatedType()->getTypeID(), ": ");
    v->getType()->dump();
    var->getAllocatedType()->getTypeID();
    return nullptr;
  }

  builder.CreateStore(v, var);
  return v;
}
}  // end namespace ast
