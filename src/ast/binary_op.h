#ifndef _DO_AST_BINARY_OP_H_
#define _DO_AST_BINARY_OP_H_

#include "ast/expression.h"

#include <iostream>
#include <string>

namespace ast {
class BinaryOp : public Expression {
 public:
  BinaryOp(const std::string& op, const Expression* lhs, const Expression* rhs)
      : op_(op), lhs_(lhs), rhs_(rhs) {
#ifdef DEBUG
    std::cerr << "BinaryOp: " << op_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;

 private:
  llvm::Value* HandleAssign() const;

  const std::string op_;
  const Expression* lhs_, *rhs_;
};
}  // end namespace ast

#endif
