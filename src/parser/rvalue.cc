#include "parser/rvalue.h"

#include "error.h"
#include "lexer.h"
#include "parser/ident.h"
#include "parser/real.h"
#include "parser/do.h"
#include "parser/nested.h"
#include "parser/collection.h"

namespace parser {
ast::Expression* RValue() {
  switch (lexer::current_token) {
    case lexer::TOKEN_IDENT:
      return Ident();

    case lexer::TOKEN_REAL:
      return Real();

    case lexer::TOKEN_DO:
      return Do();

    case '(':
      return Nested();

    case '[':
    case '<':
      return Collection();

    default:
      Error(lexer::line, lexer::col, "Expected identifier or real, got '",
            (char) lexer::current_token, "' [", lexer::current_token, "]");
      return nullptr;
  };
}
}  // end namespace parser
