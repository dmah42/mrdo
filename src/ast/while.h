#ifndef _DO_AST_WHILE_H_
#define _DO_AST_WHILE_H_

#include <vector>

#include "ast/expression.h"

namespace ast {
class While : public ast::Expression {
 public:
  While(const Expression* condition, std::vector<const Expression*>& body)
      : condition_(condition), body_(body) {}
  llvm::Value* Codegen() const override;

 private:
  const Expression* condition_;
  std::vector<const Expression*> body_;
};
}

#endif
