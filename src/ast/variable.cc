#include "ast/variable.h"

#include "ast.h"

namespace ast {
llvm::Value* Variable::Codegen() const {
  llvm::AllocaInst* val = GetNamedValue(name_);
  if (!val) {
    Error(line, col, "Unknown variable name: ", name_);
    return nullptr;
  }
  return builder.CreateLoad(val, name_.c_str());
}
}  // end namespace ast
