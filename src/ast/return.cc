#include "ast/return.h"

#include "ast.h"

namespace ast {
llvm::Value* Return::Codegen() const {
  llvm::Value* v = expression_->Codegen();
  if (!v) return nullptr;
  return builder.CreateRet(v);
}
}  // end namespace ast
