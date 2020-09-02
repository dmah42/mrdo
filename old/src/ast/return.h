#ifndef _DO_AST_RETURN_H_
#define _DO_AST_RETURN_H_

#include <iostream>

#include "ast/expression.h"
#include "debug_log.h"


namespace ast {
class Return : public Expression {
 public:
  Return(lexer::Position position, const ast::Expression* e)
      : Expression(position), expression_(e) {
    DebugLog(position, "Return");
  }
  llvm::Value* Codegen() const override;

 private:
  const ast::Expression* expression_;
};
}  // end namespace ast

#endif
