#ifndef _DO_AST_IF_H_
#define _DO_AST_IF_H_

#include "ast/expression.h"

#include <vector>

namespace ast {
class If : public Expression {
 public:
  // TODO: elif
  If(lexer::Position position,
     const Expression* condition,
     std::vector<const Expression*>& if_body,
     std::vector<const Expression*>& else_body)
      : Expression(position),
        condition_(condition),
        if_(if_body),
        else_(else_body) {}
  llvm::Value* Codegen() const override;

 private:
  const Expression* condition_;
  std::vector<const Expression*> if_;
  std::vector<const Expression*> else_;
};
}  // end namespace ast

#endif
