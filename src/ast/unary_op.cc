#include "ast/unary_op.h"

#include "ast.h"
#include "ast/real.h"
#include "ast/variable.h"
#include "error.h"
#include "llvm_type.h"

namespace ast {
llvm::Value* UnaryOp::Codegen() const {
  llvm::Value* expr = expr_->Codegen();
  if (!expr)
    return nullptr;

  if (op_ == "not") {
    const Real* expr_real = dynamic_cast<const Real*>(expr_);
    const Variable* expr_var = dynamic_cast<const Variable*>(expr_);
    // TODO: check var type is not collection.
    if (!expr_real && !expr_var) {
      Error(line, col, "Expected real or variable of type real after 'not'.");
      return nullptr;
    }
    return builder.CreateUIToFP(
        builder.CreateNot(ToBool(expr), "nottmp"), Type(), "booltmp");
  }
  Error(line, col, "Unknown unary operator: ", op_, ".");
  return nullptr;
}

llvm::Type* UnaryOp::Type() const { return TypeMap<double>::get(); }
}  // end namespace ast
