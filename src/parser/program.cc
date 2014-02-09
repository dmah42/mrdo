#include "parser/program.h"

#include <iostream>
#include <vector>

#include "ast/program.h"
#include "engine.h"
#include "lexer.h"
#include "parser/statement.h"

namespace parser {
ast::Program* Program() {
  std::vector<const ast::Expression*> state_list;
  while (lexer::current_token != lexer::TOKEN_EOF) {
    if (engine::filename.empty()) {
      std::cerr << "do] ";
    }
    const ast::Expression* s = Statement();
    if (!s) return nullptr;
    state_list.push_back(s);
  }

  return new ast::Program(state_list);
}
}  // end namespace parser
