#include "parser/return.h"

#include <cassert>

#include "ast/return.h"
#include "lexer.h"
#include "parser/expression.h"

namespace parser {
ast::Expression* Return() {
  assert(lexer::current_token == lexer::TOKEN_RETURN);
  lexer::NextToken();

  ast::Expression* e = Expression();
  if (e == nullptr) return nullptr;

  return new ast::Return(e);
}
}  // end namespace parser

