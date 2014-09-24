#include "parser/nested.h"

#include <cassert>

#include "error.h"
#include "lexer.h"
#include "parser/expression.h"

namespace parser {
ast::Expression* Nested() {
  assert(lexer::current_token == '(');
  lexer::NextToken();
  ast::Expression* e = Expression();
  if (!e)
    return nullptr;

  if (lexer::current_token != ')') {
    Error(lexer::line,
          lexer::col,
          "Expected ')', got '",
          (char)lexer::current_token,
          "' [",
          lexer::current_token,
          "]");
    return nullptr;
  }
  lexer::NextToken();
  return e;
}
}  // end namespace parser
