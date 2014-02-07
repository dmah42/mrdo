#ifndef _DO_AST_EXPRESSION_H_
#define _DO_AST_EXPRESSION_H_

#include "lexer.h"

namespace llvm { class Value; }

namespace ast {
class Expression {
 public:
  Expression() : line(lexer::line), col(lexer::col) {}
  virtual ~Expression() {}
  virtual llvm::Value* Codegen() const = 0;
  const int line, col;
};
}  // end namespace ast

#endif
