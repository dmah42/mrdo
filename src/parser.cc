#include "parser.h"

#include <cassert>
#include <functional>
#include <map>
#include <vector>

#include "ast.h"
#include "error.h"
#include "lexer.h"

namespace parser {
namespace {
ast::Expression* Expression();
ast::Expression* Ident();
ast::Expression* Real();
ast::Expression* Nested();
ast::Expression* Statement();

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
      rhs = BinaryRHS(token_prec + 1, rhs);
      if (!rhs) return nullptr;
    }

    lhs = new ast::BinaryOp(op, lhs, rhs);
  }
}

ast::Expression* If() {
  assert(lexer::current_token == lexer::TOKEN_IF);
  lexer::NextToken();

  const ast::Expression* condition = Expression();
  if (!condition) return nullptr;

  std::vector<const ast::Expression*> if_body;
  while (lexer::current_token != lexer::TOKEN_ELIF &&
         lexer::current_token != lexer::TOKEN_ELSE &&
         lexer::current_token != lexer::TOKEN_DONE) {
    const ast::Expression* if_state = Statement();
    if (!if_state) return nullptr;
    if_body.push_back(if_state);
  }

  // TODO: elif

  std::vector<const ast::Expression*> else_body;
  if (lexer::current_token == lexer::TOKEN_ELSE) {
    lexer::NextToken();
    while (lexer::current_token != lexer::TOKEN_DONE) {
      const ast::Expression* else_state = Statement();
      if (!else_state) return nullptr;
      else_body.push_back(else_state);
    }
  }

  if (lexer::current_token != lexer::TOKEN_DONE) {
    Error("expected 'done' at end of 'if', got ", lexer::current_token);
    return nullptr;
  }
  lexer::NextToken();

  return new ast::If(condition, if_body, else_body);
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
