#ifndef _DO_AST_BINARY_OP_H_
#define _DO_AST_BINARY_OP_H_

#include <iostream>
#include <string>

#include "ast/expression.h"
#include "debug_log.h"

namespace ast {
class BinaryOp : public Expression {
 public:
  BinaryOp(lexer::Position position,
           const std::string& op,
           const Expression* lhs,
           const Expression* rhs)
      : Expression(position), op_(op), lhs_(lhs), rhs_(rhs) {
    DebugLog(position, "BinaryOp: ", op_);
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

 private:
  llvm::Value* HandleAssign() const;

  const std::string op_;
  const Expression* lhs_, *rhs_;
};
}  // end namespace ast

#endif
