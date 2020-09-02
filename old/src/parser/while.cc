#include "parser/while.h"

#include <cassert>
#include <vector>

#include "ast/while.h"
#include "lexer.h"
#include "parser/expression.h"
#include "parser/statement.h"

namespace parser {
ast::Expression* While() {
  assert(lexer::current_token == lexer::TOKEN_WHILE);
  lexer::Position while_position = lexer::position;
  lexer::NextToken();

  const ast::Expression* cond = Expression();
  if (!cond)
    return nullptr;

  std::vector<const ast::Expression*> body;
  while (lexer::current_token != lexer::TOKEN_DONE) {
    const ast::Expression* state = Statement();
    if (!state)
      return nullptr;
    body.push_back(state);
  }
  lexer::NextToken();

  return new ast::While(while_position, cond, body);
}
}  // end namespace parser
