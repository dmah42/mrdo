#ifndef _DO_AST_UNARY_OP_H_
#define _DO_AST_UNARY_OP_H_

#include <iostream>
#include <string>

#include "ast/expression.h"
#include "debug_log.h"

namespace ast {
class UnaryOp : public Expression {
 public:
  UnaryOp(lexer::Position position,
          const std::string& op,
          const Expression* expr)
      : Expression(position), op_(op), expr_(expr) {
    DebugLog(position, "UnaryOp: ", op_);
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

 private:
  const std::string op_;
  const Expression* expr_;
};
}  // end namespace ast

#endif
