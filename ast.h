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
  Binary(char op, Expression* lhs, Expression* rhs)
    : op_(op), lhs_(std::move(lhs)), rhs_(std::move(rhs)) {}
  virtual llvm::Value* Codegen() const;
 private:
  char op_;
  Expression* lhs_;
  Expression* rhs_;
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
