#ifndef LEXER_H_
#define LEXER_H_

#include <string>

namespace lexer {

enum Token {
  TOKEN_EOF = -1,
  TOKEN_FUNC = -2,
  TOKEN_IDENT = -3,
  TOKEN_NUMBER = -4,
  TOKEN_IF = -5,
  TOKEN_ELSE = -6,
  TOKEN_FOR = -7,
  TOKEN_DO = -8,
  TOKEN_DONE = -9,
  TOKEN_BINOP = -10,
  TOKEN_UNOP = -11,
  TOKEN_VAR = -12  // TODO: break into coll/seq/dict
};

extern int current_token;
extern std::string identifier_str;
extern std::string op_str;
extern double number_value;

void Initialize();
int GetNextToken();

}  // end lexer

#endif
