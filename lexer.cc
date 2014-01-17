#include "lexer.h"

#include <iostream>
#include <map>
#include <string>

namespace lexer {

int current_token = TOKEN_EOF;
std::string identifier_str;
std::string op_str;
double number_value = 0.0;

namespace {
const std::map<std::string, lexer::Token> token_map = {
  {"func", TOKEN_FUNC},
  {"if", TOKEN_IF},
  {"else", TOKEN_ELSE},
  {"for", TOKEN_FOR},
  {"do", TOKEN_DO},
  {"done", TOKEN_DONE},
  {"+", TOKEN_BINOP},
  {"-", TOKEN_BINOP},
  {"*", TOKEN_BINOP},
  {"/", TOKEN_BINOP},
  {"=", TOKEN_BINOP},
  {"eq", TOKEN_BINOP},
  {"lt", TOKEN_BINOP},
  {"gt", TOKEN_BINOP},
  {"le", TOKEN_BINOP},
  {"ge", TOKEN_BINOP},
  {"ne", TOKEN_BINOP},
  {"and", TOKEN_BINOP},
  {"or", TOKEN_BINOP},
  {"not", TOKEN_UNOP},
  {"var", TOKEN_VAR}
};

int GetToken() {
  static char lastch = ' ';

  identifier_str.clear();
  op_str.clear();

  while (isspace(lastch))
    lastch = getchar();

  // identifier or op
  if (isalpha(lastch)) {
    std::string str;
    do {
      str += lastch;
      lastch = getchar();
    } while (isalnum(lastch));

    if (token_map.count(str) == 0) {
      identifier_str = str;
      return TOKEN_IDENT;
    }
    Token tok = token_map.at(str);
    if (tok == TOKEN_BINOP || tok == TOKEN_UNOP)
      op_str = str;
    return tok;
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

  std::string maybe_op(1, lastch);
  if (token_map.count(maybe_op) != 0) {
    Token tok = token_map.at(maybe_op);
    if (tok == TOKEN_BINOP || tok == TOKEN_UNOP)
      op_str = maybe_op;
    lastch = getchar();
    return tok;
  }

  switch (lastch) {
    case '#':
      do lastch = getchar();
      while (lastch != EOF && lastch != '\n' && lastch != '\r');
      if (lastch != EOF)
        return GetToken();
      break;

    case EOF:
      return TOKEN_EOF;
  }

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
