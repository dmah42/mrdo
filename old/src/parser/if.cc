#include "parser/if.h"

#include <cassert>
#include <vector>

#include "ast/if.h"
#include "error.h"
#include "lexer.h"
#include "parser/expression.h"
#include "parser/statement.h"

namespace parser {
ast::Expression* If() {
  assert(lexer::current_token == lexer::TOKEN_IF);
  lexer::Position if_position = lexer::position;
  lexer::NextToken();

  const ast::Expression* condition = Expression();
  if (!condition)
    return nullptr;

  std::vector<const ast::Expression*> if_body;
  while (lexer::current_token != lexer::TOKEN_ELIF &&
         lexer::current_token != lexer::TOKEN_ELSE &&
         lexer::current_token != lexer::TOKEN_DONE) {
    const ast::Expression* if_state = Statement();
    if (!if_state)
      return nullptr;
    if_body.push_back(if_state);
  }

  // TODO: elif

  std::vector<const ast::Expression*> else_body;
  if (lexer::current_token == lexer::TOKEN_ELSE) {
    lexer::NextToken();
    while (lexer::current_token != lexer::TOKEN_DONE) {
      const ast::Expression* else_state = Statement();
      if (!else_state)
        return nullptr;
      else_body.push_back(else_state);
    }
  }

  if (lexer::current_token != lexer::TOKEN_DONE) {
    Error(lexer::position,
          "expected 'done' at end of 'if', got '",
          (char)lexer::current_token,
          "' [",
          lexer::current_token,
          "]");
    return nullptr;
  }
  lexer::NextToken();

  return new ast::If(if_position, condition, if_body, else_body);
}
}  // end namespace parser
