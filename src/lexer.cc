#include "lexer.h"

#include <cassert>
#include <iostream>
#include <map>
#include <string>

#include "engine.h"

namespace lexer {
namespace { char lastch = ' '; }

int current_token = TOKEN_EOF;
std::string ident_str;
std::string op_str;
std::string builtin_str;
double real_value = 0.0;

int line = 1, col = 1;

namespace {
// TODO: split token map to allow extra data (ie, binop precedence)
const std::map<std::string, Token> token_map = {
  { "do", TOKEN_DO },
  { "if", TOKEN_IF },
  { "elif", TOKEN_ELIF },
  { "else", TOKEN_ELSE },
  { "done", TOKEN_DONE },
  { "while", TOKEN_WHILE },
  { "not", TOKEN_UNOP },
};

const std::map<std::string, Token> builtin_map = {
  { "map", TOKEN_BUILTIN },
  { "fold", TOKEN_BUILTIN },
  { "filter", TOKEN_BUILTIN },
  { "zip", TOKEN_BUILTIN },
  { "read", TOKEN_BUILTIN },
  { "write", TOKEN_BUILTIN },
  { "length", TOKEN_BUILTIN }
};

// TODO: add precedence and remove from src/parser.cc
const std::map<std::string, std::pair<Token, int>> binop_map = {
  { "=",   { TOKEN_ASSIGN,   2 }},
  { "or",  { TOKEN_LOGIC,    5 }},
  { "xor", { TOKEN_LOGIC,    5 }},
  { "and", { TOKEN_LOGIC,    6 }},
  { "==",  { TOKEN_COMPARE,  9 }},
  { "!=",  { TOKEN_COMPARE,  9 }},
  { "<",   { TOKEN_COMPARE, 10 }},
  { ">",   { TOKEN_COMPARE, 10 }},
  { "<=",  { TOKEN_COMPARE, 10 }},
  { ">=",  { TOKEN_COMPARE, 10 }},
  { "+",   { TOKEN_ARITH,   20 }},
  { "-",   { TOKEN_ARITH,   20 }},
  { "*",   { TOKEN_ARITH,   40 }},
  { "/",   { TOKEN_ARITH,   40 }}
};

int GetCh() {
  int ch = engine::stream->get();
  if (ch == '\n') {
    line++;
    col = 1;
  } else {
    col++;
  }
  return ch;
}

int GetToken() {
  ident_str.clear();
  op_str.clear();
  builtin_str.clear();
  real_value = 0.0;

  while (isspace(lastch))
    lastch = GetCh();

  // ident
  if (isalpha(lastch)) {
    std::string s;
    do {
      s += lastch;
      lastch = GetCh();
    } while (isalnum(lastch) || lastch == '_' || lastch == '-');

    if (token_map.count(s))
      return token_map.at(s);
    if (builtin_map.count(s)) {
      builtin_str = s;
      return builtin_map.at(s);
    }
    if (binop_map.count(s)) {
      op_str = s;
      return binop_map.at(s).first;
    }
    ident_str = s;
    return TOKEN_IDENT;
  }

  // real
  if (isdigit(lastch)) {
    bool has_decimal = false;
    std::string s;
    do {
      s += lastch;
      lastch = GetCh();
      if (lastch == '.') {
        if (!has_decimal)
          has_decimal = true;
        else
          break;
      }
    } while (isdigit(lastch) || lastch == '.');

    // TODO: error check
    real_value = strtod(s.c_str(), 0);
    return TOKEN_REAL;
  }

  // comment
  if (lastch == '#') {
    do
      lastch = GetCh();
    while (lastch != EOF && lastch != '\n');
    if (lastch != EOF) {
      // eat the newline too
      lastch = GetCh();
      return GetToken();
    }
  }

  if (lastch == EOF) return TOKEN_EOF;

  // special chars
  switch (lastch) {
    case '[':
    case '(':
    case ']':
    case ')':
    case '|': {
      int ch = lastch;
      lastch = GetCh();
      return ch;
    }
  }

  // operators
  if (!isalpha(lastch)) {
    std::string s;
    while (lastch != EOF && !isalnum(lastch) && !isspace(lastch)) {
      s += lastch;
      lastch = GetCh();
    }

    if (binop_map.count(s)) {
      op_str = s;
      return binop_map.at(s).first;
    }
    assert(s.length() == 1);
    return s[0];
  }

  int ch = lastch;
  lastch = GetCh();
  return ch;
}
}  // end namespace

void Initialize() { NextToken(); }

int NextToken() { return current_token = GetToken(); }

bool BinOpPrecedence(int* precedence) {
  if (binop_map.count(op_str)) {
    *precedence = binop_map.at(op_str).second;
    return true;
  }
  return false;
}
}  // end namespace lexer
