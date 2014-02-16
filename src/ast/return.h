#ifndef _DO_AST_RETURN_H_
#define _DO_AST_RETURN_H_

#include "ast/expression.h"

#include <iostream>

namespace ast {
class Return : public Expression {
 public:
  explicit Return(const ast::Expression* e) : expression_(e) {
#ifdef DEBUG
    std::cerr << "Return.\n";
#endif
  }
  virtual llvm::Value* Codegen() const;

 private:
  const ast::Expression* expression_;
};
}  // end namespace ast

#endif

