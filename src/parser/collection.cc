#include "parser/collection.h"

#include <cassert>
#include <vector>

#include "ast/collection.h"
#include "error.h"
#include "lexer.h"
#include "parser/rvalue.h"

namespace parser {
ast::Expression* Collection() {
  assert(lexer::current_token == '[');
  lexer::NextToken();

  std::vector<const ast::Expression*> members;
  while (true) {
    const ast::Expression* v = RValue();
    if (!v) return nullptr;
    members.push_back(v);

    if (lexer::current_token == ']') break;
    if (lexer::current_token != ',') {
      Error(lexer::line, lexer::col,
            "Expected ',' between values in collection, got ",
            lexer::current_token);
      return nullptr;
    }
    lexer::NextToken();
  }
  ast::Expression* e(new ast::Collection(members));
  lexer::NextToken();
  return e;
}
}  // end namespace parser
