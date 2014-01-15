#include "lexer.h"

#include <iostream>
#include <string>

namespace lexer {

int current_token = TOKEN_EOF;
std::string identifier_str;
double number_value = 0.0;

const char func_str[] = "func";
const char extern_str[] = "extern";

int GetToken() {
  static char lastch = ' ';

  while (isspace(lastch))
    lastch = getchar();

  if (isalpha(lastch)) {
    identifier_str = lastch;
    while (isalnum((lastch = getchar())))
      identifier_str += lastch;

    if (identifier_str == func_str) return TOKEN_FUNC;
    if (identifier_str == extern_str) return TOKEN_EXTERN;
    return TOKEN_IDENT;
  }

  if (isdigit(lastch)) {
    std::string num_str;
    do {
      num_str += lastch;
      lastch = getchar();
    } while (isdigit(lastch) || lastch == '.');

    number_value = strtod(num_str.c_str(), 0);
    return TOKEN_NUMBER;
  }

  if (lastch == '#') {
    do lastch = getchar();
    while (lastch != EOF && lastch != '\n' && lastch != '\r');
    if (lastch != EOF)
      return GetToken();
  }

  if (lastch == EOF) return TOKEN_EOF;

  int ch = lastch;
  lastch = getchar();
  return ch;
}

void Initialize() {
  GetNextToken();
}

int GetNextToken() {
  return current_token = GetToken();
}

}
