#include "parser/do.h"

#include <cassert>
#include <string>
#include <vector>

#include "ast/call.h"
#include "error.h"
#include "lexer.h"
#include "parser/rvalue.h"

namespace parser {
ast::Expression* Do() {
  assert(lexer::current_token == lexer::TOKEN_DO);
  lexer::Position call_position = lexer::position;
  lexer::NextToken();
  if (lexer::current_token != '(') {
    Error(
        lexer::position, "Expected '(' after 'do', got ", lexer::current_token);
    return nullptr;
  }
  lexer::NextToken();
  if (lexer::current_token != lexer::TOKEN_BUILTIN) {
    Error(lexer::position,
          "Expected function name after '(', got ",
          lexer::current_token);
    return nullptr;
  }
  std::string builtin = lexer::builtin_str;
  lexer::NextToken();

  std::vector<const ast::Expression*> args;
  while (true) {
    if (lexer::current_token == ')')
      break;

    if (lexer::current_token != ',') {
      Error(lexer::position,
            "Expected ',' between args in do, got ",
            lexer::current_token);
      return nullptr;
    }
    lexer::NextToken();

    const ast::Expression* v = RValue();
    if (!v)
      return nullptr;
    args.push_back(v);
  }
  lexer::NextToken();
  return new ast::Call(call_position, builtin, args);
}
}  // end namespace parser
