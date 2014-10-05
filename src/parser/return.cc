#include "parser/return.h"

#include <cassert>

#include "ast/return.h"
#include "lexer.h"
#include "parser/expression.h"

namespace parser {
ast::Expression* Return() {
  assert(lexer::current_token == lexer::TOKEN_RETURN);
  lexer::Position return_position = lexer::position;
  lexer::NextToken();

  ast::Expression* e = Expression();
  if (e == nullptr)
    return nullptr;

  return new ast::Return(return_position, e);
}
}  // end namespace parser
