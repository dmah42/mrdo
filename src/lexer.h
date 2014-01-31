#ifndef _DO_LEXER_H_
#define _DO_LEXER_H_

#include <string>

namespace lexer {
enum Token {
  TOKEN_EOF = -1,
  TOKEN_IDENT = -2,
  TOKEN_REAL = -3,
  TOKEN_DO = -4,
  TOKEN_UNOP = -5,
  TOKEN_IF = -5,
  TOKEN_ELIF = -6,
  TOKEN_ELSE = -7,
  TOKEN_WHILE = -8,
  TOKEN_DONE = -9,
  TOKEN_BUILTIN = -10,
  TOKEN_ASSIGN = -11,
  TOKEN_LOGIC = -12,
  TOKEN_ARITH = -13,
  TOKEN_COMPARE = -14
};

extern int current_token;
extern std::string ident_str;
extern std::string op_str;
extern double real_value;

extern int line, col;

void Initialize();
int NextToken();
bool BinOpPrecedence(int* precedence);
}  // end namespace lexer

#endif  // _DO_LEXER_H_
