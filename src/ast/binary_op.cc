#include "ast/binary_op.h"

#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Module.h>

#include "ast.h"
#include "ast/call.h"
#include "ast/collection.h"
#include "ast/real.h"
#include "ast/variable.h"
#include "builtin.h"
#include "error.h"
#include "llvm_type.h"

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
  // TODO: test reassignment to same variable
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
    const Collection* rhs_coll = dynamic_cast<const Collection*>(rhs_);
    const Variable* rhs_v = dynamic_cast<const Variable*>(rhs_);
    const Real* rhs_r = dynamic_cast<const Real*>(rhs_);
    const BinaryOp* rhs_binop = dynamic_cast<const BinaryOp*>(rhs_);
    const Call* rhs_call = dynamic_cast<const Call*>(rhs_);
    llvm::Type* alloca_type = nullptr;
    if (rhs_coll) {
      alloca_type = TypeMap<builtin::Collection>::get();
    } else if (rhs_v) {
      alloca_type = v->getType();
    } else if (rhs_r || rhs_binop) {
      alloca_type = TypeMap<double>::get();
    } else if (rhs_call) {
      llvm::Function* func = engine::module->getFunction(rhs_call->name());
      if (!func) {
        Error(line, col, "Unknown function: ", rhs_call->name());
        return nullptr;
      }
      alloca_type = func->getReturnType();
    } else {
      Error(line, col, "Unknown rhs type: ");
      v->dump();
      return nullptr;
    }
    var = CreateEntryBlockAlloca(f, alloca_type, lhs_variable->name());
    SetNamedValue(lhs_variable->name(), var);
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
