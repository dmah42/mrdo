#include "parser/unary.h"

#include <string>

#include "ast/unary_op.h"
#include "lexer.h"
#include "parser/rvalue.h"

namespace parser {
ast::Expression* Unary() {
  if (lexer::current_token != lexer::TOKEN_UNOP)
    return RValue();

  std::string op = lexer::op_str;
  lexer::NextToken();

  ast::Expression* operand = Unary();
  if (!operand)
    return nullptr;
  return new ast::UnaryOp(op, operand);
}
}  // end namespace parser
