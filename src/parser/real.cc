#include "parser/real.h"

#include <cassert>

#include "ast/real.h"
#include "lexer.h"

namespace parser {
ast::Expression* Real() {
  assert(lexer::current_token == lexer::TOKEN_REAL);
  ast::Expression* e(new ast::Real(lexer::real_value));
  lexer::NextToken();
  return e;
}
}  // end namespace parser
