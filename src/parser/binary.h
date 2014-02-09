#ifndef _DO_PARSER_BINARY_H_
#define _DO_PARSER_BINARY_H_

namespace ast { class Expression; }

namespace parser {
ast::Expression* Binary(int precedence, ast::Expression* lhs);
}  // end namespace parser

#endif
