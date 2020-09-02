#ifndef _DO_AST_REAL_H_
#define _DO_AST_REAL_H_

#include <iostream>

#include "ast/expression.h"
#include "debug_log.h"

namespace ast {
class Real : public Expression {
 public:
  Real(lexer::Position position, double value)
      : Expression(position), value_(value) {
    DebugLog(position, "Real: ", value_);
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

 private:
  double value_;
};
}  // end namespace ast

#endif
