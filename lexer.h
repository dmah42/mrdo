#ifndef LEXER_H_
#define LEXER_H_

#include <string>

namespace lexer {

enum Token {
  TOKEN_EOF = -1,
  TOKEN_FUNC = -2,
  TOKEN_EXTERN = -3,
  TOKEN_IDENT = -4,
  TOKEN_NUMBER = -5,
  TOKEN_IF = -6,
  TOKEN_ELSE = -7,
  TOKEN_FOR = -8,
  TOKEN_DO = -9
};

extern int current_token;
extern std::string identifier_str;
extern double number_value;

void Initialize();
int GetNextToken();

}  // end lexer

#endif
