#ifndef _DO_AST_REAL_H_
#define _DO_AST_REAL_H_

#include "ast/expression.h"

#include <iostream>

namespace ast {
class Real : public Expression {
 public:
  explicit Real(double value) : value_(value) {
#ifdef DEBUG
    std::cerr << "Real: " << value_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;

 private:
  double value_;
};
}  // end namespace ast

#endif
