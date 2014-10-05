#include "parser/func.h"

#include <cassert>

#include "ast/func.h"
#include "error.h"
#include "lexer.h"
#include "parser/statement.h"

namespace parser {
ast::Expression* Func() {
  assert(lexer::current_token == lexer::TOKEN_FUNC);
  lexer::Position func_position = lexer::position;
  lexer::NextToken();
  if (lexer::current_token != '(') {
    Error(lexer::position, "Expected argument list after 'func'");
    return nullptr;
  }
  lexer::NextToken();

  std::vector<std::string> args;
  if (lexer::current_token != ')') {
    while (true) {
      if (lexer::current_token != lexer::TOKEN_IDENT) {
        Error(lexer::position,
              "Expected identifier as function argument, got ",
              lexer::current_token);
        return nullptr;
      }
      args.push_back(lexer::ident_str);
      lexer::NextToken();

      if (lexer::current_token == ')')
        break;

      if (lexer::current_token != ',') {
        Error(lexer::position,
              "Expected ',' between args in func, got ",
              lexer::current_token);
        return nullptr;
      }
      lexer::NextToken();
    }
  }
  lexer::NextToken();

  if (lexer::current_token != '{') {
    Error(lexer::position,
          "Expected '{' after arg list in func, got ",
          lexer::current_token);
    return nullptr;
  }
  lexer::NextToken();

  std::vector<const ast::Expression*> body;
  while (true) {
    if (lexer::current_token == '}')
      break;

    const ast::Expression* state = Statement();
    if (!state)
      return nullptr;
    body.push_back(state);
  }
  lexer::NextToken();

  return new ast::Func(func_position, args, body);
}
}
