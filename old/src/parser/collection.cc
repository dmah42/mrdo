#include "parser/collection.h"

#include <cassert>
#include <vector>

#include "ast/collection.h"
#include "error.h"
#include "lexer.h"
#include "parser/rvalue.h"

namespace parser {
ast::Expression* Collection() {
  assert(lexer::current_token == '[' || lexer::current_token == '|');
  bool is_sequence = lexer::current_token == '|';
  // cache the current lexer position to pass to the AST.
  lexer::Position collection_position = lexer::position;
  lexer::NextToken();

  const char end_token = is_sequence ? '|' : ']';
  std::vector<const ast::Expression*> members;
  while (true) {
    const ast::Expression* v = RValue();
    if (!v)
      return nullptr;
    members.push_back(v);

    if (lexer::current_token == end_token)
      break;
    if (lexer::current_token != ',') {
      Error(lexer::position,
            "Expected ',' between values in collection, got ",
            lexer::current_token);
      return nullptr;
    }
    lexer::NextToken();
  }
  ast::Expression* e(
      new ast::Collection(collection_position, is_sequence, members));
  lexer::NextToken();
  return e;
}
}  // end namespace parser
