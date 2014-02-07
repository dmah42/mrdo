#ifndef _DO_AST_UNARY_OP_H_
#define _DO_AST_UNARY_OP_H_

#include "ast/expression.h"

#include <iostream>
#include <string>

namespace ast {
class UnaryOp : public Expression {
 public:
  UnaryOp(const std::string& op, const Expression* expr)
      : op_(op), expr_(expr) {
#ifdef DEBUG
    std::cerr << "UnaryOp: " << op_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;

 private:
  const std::string op_;
  const Expression* expr_;
};
}  // end namespace ast

#endif
