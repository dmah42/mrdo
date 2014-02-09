#include "parser/binary.h"

#include <string>

#include "ast/binary_op.h"
#include "lexer.h"
#include "parser/unary.h"

namespace parser {
ast::Expression* Binary(int precedence, ast::Expression* lhs) {
  while (true) {
    int token_prec;
    bool valid_binop = lexer::BinOpPrecedence(&token_prec);
    if (!valid_binop || token_prec < precedence) return lhs;

    std::string op = lexer::op_str;
    lexer::NextToken();

    ast::Expression* rhs = Unary();
    if (!rhs) return nullptr;

    int next_prec;
    valid_binop = lexer::BinOpPrecedence(&next_prec);
    if (valid_binop && token_prec < next_prec) {
      rhs = Binary(token_prec + 1, rhs);
      if (!rhs) return nullptr;
    }

    lhs = new ast::BinaryOp(op, lhs, rhs);
  }
}
}  // end namespace parser
