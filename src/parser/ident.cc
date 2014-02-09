#include "parser/ident.h"

#include <cassert>
#include <string>

#include "ast/variable.h"
#include "lexer.h"

namespace parser {
ast::Expression* Ident() {
  assert(lexer::current_token == lexer::TOKEN_IDENT);
  std::string name = lexer::ident_str;
  ast::Expression* e(new ast::Variable(name));
  lexer::NextToken();
  return e;
}
}  // end namespace parser
