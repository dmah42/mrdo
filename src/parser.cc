#include "parser.h"

#include <cassert>
#include <functional>
#include <map>
#include <vector>

#include "ast.h"
#include "error.h"
#include "lexer.h"

namespace dolib {
namespace parser {
namespace {
ast::Expression* Expression();
ast::Expression* Ident();
ast::Expression* Real();
ast::Expression* Nested();

// TODO: keep this in sync with lexer. maybe add a second map from op to
// binary_op_details
const std::map<std::string, int> binary_op_precedence = {
  {"=", 2},
  {"or", 5},
  {"xor", 5},
  {"and", 6},
  {"==", 9},
  {"!=", 9},
  {"<", 10},
  {">", 10},
  {"<=", 10},
  {">=", 10},
  {"+", 20},
  {"-", 20},
  {"*", 40},
  {"/", 40}
};

bool IsValidBinop() {
  return lexer::current_token == lexer::TOKEN_LOGIC ||
    lexer::current_token == lexer::TOKEN_ARITH ||
    lexer::current_token == lexer::TOKEN_COMPARE ||
    lexer::current_token == lexer::TOKEN_ASSIGN;
}

int GetTokenPrecedence() {
  if (!IsValidBinop()) return -1;
  return binary_op_precedence.count(lexer::op_str) ?
      binary_op_precedence.at(lexer::op_str) : -1;
}

ast::Expression* Ident() {
  assert(lexer::current_token == lexer::TOKEN_IDENT);
  std::string name = lexer::ident_str;
  ast::Expression* e(new ast::Variable(name));
  lexer::NextToken();
  return e;
}

ast::Expression* Real() {
  assert(lexer::current_token == lexer::TOKEN_REAL);
  ast::Expression* e(new ast::Real(lexer::real_value));
  lexer::NextToken();
  return e;
}

ast::Expression* Nested() {
  assert(lexer::current_token == '(');
  lexer::NextToken();
  ast::Expression* e = Expression();
  if (!e) return nullptr;

  if (lexer::current_token != ')') {
    Error("Expected ')', got ", lexer::current_token);
    return nullptr;
  }
  lexer::NextToken();
  return e;
}

ast::Expression* RValue() {
  switch (lexer::current_token) {
    case lexer::TOKEN_IDENT:
      return Ident();

    case lexer::TOKEN_REAL:
      return Real();

    case '(':
      return Nested();

    default:
      Error("Expected identifier or real, got ", lexer::current_token);
      return nullptr;
  };
}

ast::Expression* Unary() {
  if (lexer::current_token != lexer::TOKEN_UNOP)
    return RValue();

  std::string op = lexer::op_str;
  lexer::NextToken();

  ast::Expression* operand = Unary();
  if (!operand) return nullptr;
  return new ast::UnaryOp(op, operand);
}

ast::Expression* BinaryRHS(
    int precedence, ast::Expression* lhs) {
  while (true) {
    int token_prec = GetTokenPrecedence();
    if (token_prec < precedence) return lhs;

    if (lexer::current_token != lexer::TOKEN_LOGIC &&
        lexer::current_token != lexer::TOKEN_ARITH &&
        lexer::current_token != lexer::TOKEN_COMPARE &&
        lexer::current_token != lexer::TOKEN_ASSIGN) {
      Error("Unknown operator ", lexer::current_token);
      return nullptr;
    }

    std::string op = lexer::op_str;
    lexer::NextToken();

    ast::Expression* rhs = Unary();
    if (!rhs) return nullptr;

    int next_prec = GetTokenPrecedence();
    if (token_prec < next_prec) {
      rhs = BinaryRHS(token_prec + 1, rhs);
      if (!rhs) return nullptr;
    }

    lhs = new ast::BinaryOp(op, lhs, rhs);
  }
}

ast::Expression* If() {
  // TODO: implement
  return nullptr;
}

ast::Expression* While() {
  // TODO: implement
  return nullptr;
}

ast::Expression* Expression() {
  ast::Expression* lhs = Unary();
  if (lhs == nullptr) return nullptr;
  ast::Expression* e = BinaryRHS(0, lhs);
  // TODO: make ';' an option for multiline statements
  return e;
}

ast::Expression* Statement() {
  switch (lexer::current_token) {
    case lexer::TOKEN_IF:
      return If();

    case lexer::TOKEN_WHILE:
      return While();

    default:
      return Expression();
  };
}
}  // end namespace

ast::Program* Program() {
  std::vector<const ast::Expression*> state_list;
  while (lexer::current_token != lexer::TOKEN_EOF) {
    const ast::Expression* s = Statement();
    if (!s) return nullptr;
    state_list.push_back(s);
  }

  return new ast::Program(state_list);
}

}  // end namespace parser
}  // end namespace dolib
