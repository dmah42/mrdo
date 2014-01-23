#include "parser.h"

#include <cassert>
#include <iostream>
#include <map>

#include "ast.h"
#include "error.h"
#include "lexer.h"

namespace parser {
namespace {
ast::Expression* Identifier();
ast::Expression* If();
ast::Expression* For();
ast::Expression* Nested();
ast::Expression* Number();
ast::Expression* Var();

const std::map<char, std::function<ast::Expression*()>> token_func = {
  {lexer::TOKEN_IDENT, Identifier},
  {lexer::TOKEN_NUMBER, Number},
  {lexer::TOKEN_IF, If},
  {lexer::TOKEN_FOR, For},
  {lexer::TOKEN_VAR, Var},
  {'(', Nested}
};

// TODO: keep this in sync with lexer somehow...
const std::map<std::string, int> binary_op_precedence = {
  {"=", 2},
  {"or", 5},
  {"and", 6},
  {"eq", 9},
  {"ne", 9},
  {"lt", 10},
  {"gt", 10},
  {"le", 10},
  {"ge", 10},
  {"+", 20},
  {"-", 20},
  {"*", 40},
  {"/", 40}
};

// TODO: better error checking if this is called with a binop that isn't in the
// precedence map.
int GetTokenPrecedence() {
  if (lexer::current_token != lexer::TOKEN_BINOP)
    return -1;

  std::map<std::string, int>::const_iterator prec_it =
      binary_op_precedence.find(lexer::op_str);
  if (prec_it == binary_op_precedence.end()) return -1;
  return prec_it->second;
}

ast::Expression* Expression();

bool ExpressionList(std::vector<const ast::Expression*>* list) {
  assert(list);
  while (lexer::current_token != lexer::TOKEN_DONE) {
    const ast::Expression* e = Expression();
    if (!e) return nullptr;
    list->push_back(e);

    if (lexer::current_token != ';') {
      Error("expected ';' after expression. got ", lexer::current_token);
      return false;
    }
    lexer::GetNextToken();
  }
  lexer::GetNextToken();
  return true;
}

ast::Expression* Identifier() {
  assert(lexer::current_token == lexer::TOKEN_IDENT);
  std::string name = lexer::identifier_str;
  lexer::GetNextToken();

  // variable
  if (lexer::current_token != '(')
    return new ast::Variable(name);

  // call
  lexer::GetNextToken();
  std::vector<ast::Expression*> args;
  if (lexer::current_token != ')') {
    while (true) {
      ast::Expression* arg = Expression();
      if (arg == nullptr) return nullptr;

      args.push_back(arg);

      if (lexer::current_token == ')') break;

      if (lexer::current_token != ',') {
        Error("Expected ')' or ',' in argument list.");
        return nullptr;
      }
      lexer::GetNextToken();
    }
  }

  lexer::GetNextToken();

  return new ast::Call(name, args);
}

ast::Expression* Number() {
  assert(lexer::current_token == lexer::TOKEN_NUMBER);
  ast::Expression* e(new ast::Number(lexer::number_value));
  lexer::GetNextToken();
  return e;
}

ast::Expression* Nested() {
  lexer::GetNextToken();
  ast::Expression* e = Expression();
  if (e == nullptr) return nullptr;

  if (lexer::current_token != ')') {
    Error("expected ')', got ", lexer::current_token);
    return nullptr;
  }
  lexer::GetNextToken();
  return e;
}

ast::Expression* If() {
  assert(lexer::current_token == lexer::TOKEN_IF);
  lexer::GetNextToken();

  const ast::Expression* condition = Expression();
  if (!condition) return nullptr;

  if (lexer::current_token != lexer::TOKEN_DO) {
    Error("expected 'do', got ", lexer::current_token);
    return nullptr;
  }
  lexer::GetNextToken();

  std::vector<const ast::Expression*> _if;

  // TODO: figure out how to use ExpressionList with this.
  while (lexer::current_token != lexer::TOKEN_DONE &&
         lexer::current_token != lexer::TOKEN_ELSE) {
    const ast::Expression* if_expr = Expression();
    if (!if_expr) return nullptr;

    _if.push_back(if_expr);

    if (lexer::current_token != ';') {
      Error("expected ';' after expression, got ", lexer::current_token);
      return nullptr;
    }
    lexer::GetNextToken();
  }

  std::vector<const ast::Expression*> _else;
  if (lexer::current_token == lexer::TOKEN_ELSE) {
    lexer::GetNextToken();
    if (!ExpressionList(&_else)) return nullptr;
  } else {
    assert(lexer::current_token == lexer::TOKEN_DONE);
    lexer::GetNextToken();
  }

  return new ast::If(condition, _if, _else);
}

ast::Expression* For() {
  assert(lexer::current_token == lexer::TOKEN_FOR);
  lexer::GetNextToken();

  if (lexer::current_token != lexer::TOKEN_IDENT) {
    Error("Expected identifier after for, got ", lexer::current_token);
    return nullptr;
  }

  std::string name = lexer::identifier_str;
  lexer::GetNextToken();

  if (lexer::current_token != lexer::TOKEN_BINOP ||
      lexer::op_str != "=") {
    Error("expected '=' after for, got ", lexer::current_token);
    return nullptr;
  }
  lexer::GetNextToken();

  const ast::Expression* start = Expression();
  if (!start) return nullptr;

  if (lexer::current_token != ',') {
    Error("expected ',' after for start, got ", lexer::current_token);
    return nullptr;
  }
  lexer::GetNextToken();

  const ast::Expression* end = Expression();
  if (!end) return nullptr;

  const ast::Expression* step = nullptr;
  if (lexer::current_token == ',') {
    lexer::GetNextToken();
    step = Expression();
    if (!step) return nullptr;
  }

  if (lexer::current_token != lexer::TOKEN_DO) {
    Error("expected 'do' after step");
    return nullptr;
  }
  lexer::GetNextToken();

  std::vector<const ast::Expression*> body;
  if (!ExpressionList(&body)) return nullptr;

  return new ast::For(name, start, end, step, body);
}

ast::Expression* Var() {
  lexer::GetNextToken();

  if (lexer::current_token != lexer::TOKEN_IDENT) {
    Error("expected identifier after var");
    return nullptr;
  }

  std::string name = lexer::identifier_str;
  lexer::GetNextToken();

  // read optional initializer
  // TODO: make this required - copy or 'read'
  ast::Expression* init = nullptr;
  if (lexer::current_token == lexer::TOKEN_BINOP) {
    if (lexer::op_str != "=") {
      Error("expected '=' after variable name");
      return nullptr;
    }
    lexer::GetNextToken();

    init = Expression();
    if (!init) return nullptr;
  }

  return new ast::Var(name, init);
}

ast::Expression* Primary() {
  if (token_func.count(lexer::current_token) == 0) {
    Error("unknown token ", lexer::current_token, " expecting expression");
    return nullptr;
  }
  return token_func.at(lexer::current_token)();
}

ast::Expression* Unary() {
  if (lexer::current_token != lexer::TOKEN_UNOP)
    return Primary();
  std::string op = lexer::op_str;
  lexer::GetNextToken();

  ast::Expression* operand = Unary();
  if (!operand) return nullptr;
  return new ast::Unary(op, operand);
}

ast::Expression* BinaryOpRHS(
    int precedence, ast::Expression* lhs) {
  while (true) {
    int token_prec = GetTokenPrecedence();

    // This binop must bind at least as tightly as the current one or we are
    // done.
    if (token_prec < precedence) return lhs;

    if (lexer::current_token != lexer::TOKEN_BINOP) {
      Error("unknown operator ", lexer::current_token);
      return nullptr;
    }
    std::string op = lexer::op_str;
    lexer::GetNextToken();

    ast::Expression* rhs = Unary();
    if (rhs == nullptr) return nullptr;

    // If the binop binds less tightly with RHS than the operator after RHS, let
    // the pending op take RHS as its LHS.
    int next_prec = GetTokenPrecedence();
    if (token_prec < next_prec) {
      rhs = BinaryOpRHS(token_prec + 1, rhs);
      if (rhs == nullptr) return nullptr;
    }

    // Merge
    lhs = new ast::Binary(op, lhs, rhs);
  }
}

ast::Expression* Expression() {
  ast::Expression* lhs = Unary();
  if (lhs == nullptr) return nullptr;
  return BinaryOpRHS(0, lhs);
}

ast::Prototype* Prototype() {
  if (lexer::current_token != lexer::TOKEN_IDENT) {
    Error("Expected function name in prototype");
    return nullptr;
  }

  std::string name = lexer::identifier_str;
  lexer::GetNextToken();

  if (lexer::current_token != '(') {
    Error("Expected '(' in prototype");
    return nullptr;
  }

  std::vector<std::string> args;
  // TODO: comma between args
  while (lexer::GetNextToken() == lexer::TOKEN_IDENT)
    args.push_back(lexer::identifier_str);

  if (lexer::current_token != ')') {
    Error("Expected ')' in prototype");
    return nullptr;
  }

  lexer::GetNextToken();
  return new ast::Prototype(name, args);
}

}  // end namespace

ast::Function* Function() {
  lexer::GetNextToken();
  ast::Prototype* proto = Prototype();
  if (proto == nullptr) return nullptr;

  std::vector<const ast::Expression*> body;
  if (!ExpressionList(&body)) return nullptr;

  return new ast::Function(proto, body);
}

ast::Function* TopLevel() {
  std::vector<const ast::Expression*> expr_list;
  if (!ExpressionList(&expr_list)) return nullptr;

  ast::Prototype* p = new ast::Prototype("", {});
  return new ast::Function(p, expr_list);
  //const ast::Expression* expr = Expression();
  //if (!expr) return nullptr;

  //ast::Prototype* p = new ast::Prototype("", {});
  //return new ast::Function(p, {expr});
}

ast::Prototype* Native() {
  lexer::GetNextToken();
  return Prototype();
}

}  // end parser
