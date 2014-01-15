#include "lexer.h"

#include <iostream>
#include <map>
#include <string>

namespace lexer {

int current_token = TOKEN_EOF;
std::string identifier_str;
double number_value = 0.0;

namespace {
const std::map<std::string, lexer::Token> token_map = {
  {"func", TOKEN_FUNC},
  {"extern", TOKEN_EXTERN},
  {"if", TOKEN_IF},
  {"else", TOKEN_ELSE},
};

int GetToken() {
  static char lastch = ' ';

  while (isspace(lastch))
    lastch = getchar();

  if (isalpha(lastch)) {
    identifier_str = lastch;
    while (isalnum((lastch = getchar())))
      identifier_str += lastch;

    std::map<std::string, lexer::Token>::const_iterator tok_it =
        token_map.find(identifier_str);
    if (tok_it == token_map.end())
      return TOKEN_IDENT;
    return tok_it->second;
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
}  // end namespace

void Initialize() {
  GetNextToken();
}

int GetNextToken() {
  return current_token = GetToken();
}
}
