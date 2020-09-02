#ifndef _DO_AST_EXPRESSION_H_
#define _DO_AST_EXPRESSION_H_

#include "lexer.h"

namespace llvm {
class Type;
class Value;
}

namespace ast {
class Expression {
 public:
  Expression(lexer::Position position) : position(position) {}
  virtual ~Expression() {}
  virtual llvm::Value* Codegen() const = 0;
  virtual llvm::Type* Type() const { return nullptr; }
  const lexer::Position position;
};
}  // end namespace ast

#endif
