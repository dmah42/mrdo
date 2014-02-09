#include "parser/statement.h"

#include "lexer.h"
#include "parser/expression.h"
#include "parser/if.h"
#include "parser/while.h"

namespace parser {
ast::Expression* Statement() {
  switch (lexer::current_token) {
    case lexer::TOKEN_IF:
      return If();

    case lexer::TOKEN_WHILE:
      return While();

    default:
      return Expression();
  };
}
}  // end namespace parser
