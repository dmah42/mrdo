#include "parser.h"

#include <iostream>
#include <map>

#include "ast.h"
#include "lexer.h"

namespace parser {
namespace {
ast::Expression* Identifier();
ast::Expression* If();
ast::Expression* Nested();
ast::Expression* Number();

const std::map<char, std::function<ast::Expression*()>> token_func = {
  {lexer::TOKEN_IDENT, Identifier},
  {lexer::TOKEN_NUMBER, Number},
  {lexer::TOKEN_IF, If},
  {'(', Nested}
};

const std::map<char, int> binary_op_precedence = {
  {'<', 10},
  {'+', 20},
  {'-', 20},
  {'*', 40}
};

int GetTokenPrecedence() {
  if (!isascii(lexer::current_token))
    return -1;

  std::map<char, int>::const_iterator prec_it =
      binary_op_precedence.find(lexer::current_token);
  if (prec_it == binary_op_precedence.end()) return -1;
  return prec_it->second;
}

ast::Expression* Error(const std::string& s) {
  std::cerr << "Error: " << s << "\n";
  return nullptr;
}

ast::Prototype* ErrorP(const std::string& s) {
  Error(s);
  return nullptr;
}

ast::Function* ErrorF(const std::string& s) {
  Error(s);
  return nullptr;
}

ast::Expression* Expression();

ast::Expression* Identifier() {
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
      ast::Expression* arg = std::move(parser::Expression());
      if (arg == nullptr) return nullptr;

      args.push_back(arg);

      if (lexer::current_token == ')') break;

      if (lexer::current_token != ',')
        return Error("Expected ')' or ',' in argument list.");
      lexer::GetNextToken();
    }
  }

  lexer::GetNextToken();

  return new ast::Call(name, args);
}

ast::Expression* Number() {
  ast::Expression* e(new ast::Number(lexer::number_value));
  lexer::GetNextToken();
  return e;
}

ast::Expression* Nested() {
  lexer::GetNextToken();
  ast::Expression* e = Expression();
  if (e == nullptr) return nullptr;

  if (lexer::current_token != ')') return Error("expected ')'");
  lexer::GetNextToken();
  return e;
}

ast::Expression* If() {
  lexer::GetNextToken();

  const ast::Expression* condition = Expression();
  if (!condition) return nullptr;

  const ast::Expression* _if = Expression();
  if (!_if) return nullptr;

  if (lexer::current_token != lexer::TOKEN_ELSE)
    return Error("Expected 'else'");
  lexer::GetNextToken();

  const ast::Expression* _else = Expression();
  if (!_else) return nullptr;

  return new ast::If(condition, _if, _else);
}

ast::Expression* Primary() {
  std::map<char, std::function<ast::Expression*()>>::const_iterator
      token_func_it = token_func.find(lexer::current_token);
  if (token_func_it == token_func.end())
    return Error("unknown token expecting expression");
  return token_func_it->second();
}

ast::Expression* BinaryOpRHS(
    int precedence, ast::Expression* lhs) {
  while (true) {
    int token_prec = GetTokenPrecedence();

    // This binop must bind at least as tightly as the current one or we are
    // done.
    if (token_prec < precedence) return lhs;

    int op = lexer::current_token;
    lexer::GetNextToken();

    ast::Expression* rhs = std::move(Primary());
    if (rhs == nullptr) return nullptr;

    // If the binop binds less tightly with RHS than the operator after RHS, let
    // the pending op take RHS as its LHS.
    int next_prec = GetTokenPrecedence();
    if (token_prec < next_prec) {
      rhs = BinaryOpRHS(token_prec + 1, std::move(rhs));
      if (rhs == nullptr) return nullptr;
    }

    // Merge
    lhs = new ast::Binary(op, lhs, rhs);
  }
}

ast::Expression* Expression() {
  ast::Expression* lhs = Primary();
  if (lhs == nullptr) return nullptr;
  return BinaryOpRHS(0, std::move(lhs));
}

ast::Prototype* Prototype() {
  if (lexer::current_token != lexer::TOKEN_IDENT)
    return ErrorP("Expected function name in prototype");

  std::string name = lexer::identifier_str;
  lexer::GetNextToken();

  if (lexer::current_token != '(')
    return ErrorP("Expected '(' in prototype");

  std::vector<std::string> args;
  // TODO: comma between args
  while (lexer::GetNextToken() == lexer::TOKEN_IDENT)
    args.push_back(lexer::identifier_str);

  if (lexer::current_token != ')')
    return ErrorP("Expected ')' in prototype");

  lexer::GetNextToken();
  return new ast::Prototype(name, args);
}
}  // end namespace

ast::Function* Function() {
  lexer::GetNextToken();
  ast::Prototype* proto = Prototype();
  if (proto == nullptr)
    return ErrorF("unable to parse prototype");

  ast::Expression* e = Expression();
  if (e == nullptr)
    return ErrorF("unable to parse body");

  return new ast::Function(proto, e);
}

ast::Function* TopLevel() {
  ast::Expression* e = Expression();
  if (e == nullptr) return nullptr;

  ast::Prototype* p = new ast::Prototype("", {});
  return new ast::Function(p, e);
}

ast::Prototype* Extern() {
  lexer::GetNextToken();
  return Prototype();
}

}  // end parser
