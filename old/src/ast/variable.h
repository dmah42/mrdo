#ifndef _DO_AST_VARIABLE_H_
#define _DO_AST_VARIABLE_H_

#include <iostream>
#include <string>

#include "ast/expression.h"
#include "debug_log.h"
#include "error.h"

namespace ast {
class Variable : public Expression {
 public:
  Variable(lexer::Position position, const std::string& name)
      : Expression(position), name_(name), v_(nullptr) {
    DebugLog(position, "Variable: ", name_);
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

  const std::string& name() const { return name_; }

 private:
  std::string name_;

  mutable llvm::Value* v_;
};
}  // end namespace ast

#endif
