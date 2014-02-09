#include "parser/expression.h"

#include "parser/binary.h"
#include "parser/unary.h"

namespace parser {
ast::Expression* Expression() {
  ast::Expression* lhs = Unary();
  if (lhs == nullptr) return nullptr;
  ast::Expression* e = Binary(0, lhs);
  // TODO: make ';' an option for multiline statements
  return e;
}
}  // end namespace parser
