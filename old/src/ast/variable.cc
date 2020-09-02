#include "ast/variable.h"

#include "ast.h"

namespace ast {
llvm::Value* Variable::Codegen() const {
  llvm::AllocaInst* val = GetNamedValue(name_);
  if (!val) {
    Error(position, "Unknown variable name: ", name_);
    return nullptr;
  }
  v_ = builder.CreateLoad(val, name_.c_str());
  return v_;
}

llvm::Type* Variable::Type() const {
  assert(v_);
  return v_->getType();
}
}  // end namespace ast
