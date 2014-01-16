#ifndef AST_H_
#define AST_H_

#include <memory>
#include <string>
#include <vector>

namespace llvm {
class Function;
class Module;
class Value;
}

namespace ast {

void Initialize();

class Expression {
 public:
  virtual ~Expression() {}
  virtual llvm::Value* Codegen() const = 0;
};

class Number : public Expression {
 public:
  Number(double value) : value_(value) {}
  virtual llvm::Value* Codegen() const;
 private:
  double value_;
};

class Variable : public Expression {
 public:
  Variable(const std::string& name) : name_(name) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string name_;
};

class Binary : public Expression {
 public:
  Binary(const std::string& op, Expression* lhs, Expression* rhs)
    : op_(op), lhs_(lhs), rhs_(rhs) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string op_;
  Expression* lhs_;
  Expression* rhs_;
};

class Unary : public Expression {
 public:
  Unary(const std::string& op, Expression* expr)
    : op_(op), expression_(expr) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string op_;
  Expression* expression_;
};

class Call : public Expression {
 public:
  Call(const std::string& callee, const std::vector<Expression*>& args)
    : callee_(callee), args_(args) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string callee_;
  std::vector<Expression*> args_;
};

class If : public Expression {
 public:
  If(const Expression* condition, const Expression* _if,
     const Expression* _else)
    : condition_(condition), if_(_if), else_(_else) {}
  virtual llvm::Value* Codegen() const;
 private:
  const Expression *condition_, *if_, *else_;
};

class For : public Expression {
 public:
   // TODO: for i = 1..n, 2 {
  For(const std::string& var, const Expression* start, const Expression* end,
      const Expression* step, const Expression* body)
      : var_(var), start_(start), end_(end), step_(step), body_(body) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string var_;
  const Expression *start_, *end_, *step_, *body_;
};

class Prototype {
 public:
  Prototype(const std::string& name, const std::vector<std::string>& args)
    : name_(name), args_(args) {}
  llvm::Function* Codegen() const;
 private:
  std::string name_;
  std::vector<std::string> args_;
};

class Function {
 public:
  Function(Prototype* prototype,
           Expression* body)
    : prototype_(std::move(prototype)), body_(std::move(body)) {}
  llvm::Function* Codegen() const;
 private:
  Prototype* prototype_;
  Expression* body_;
};

}  // end ast

#endif
