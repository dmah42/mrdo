#ifndef _DO_AST_H_
#define _DO_AST_H_

#include <iostream>
#include <string>
#include <vector>

namespace llvm {
class Function;
class Value;
}

namespace dolib {
namespace ast {
class Expression {
 public:
  virtual ~Expression() {}
  virtual llvm::Value* Codegen() const = 0;
};

class Real : public Expression {
 public:
  explicit Real(double value) : value_(value) {
#ifdef DEBUG
    std::cerr << "Real: " << value_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;
 private:
  double value_;
};

class Variable : public Expression {
 public:
  explicit Variable(const std::string& name) : name_(name) {
#ifdef DEBUG
    std::cerr << "Variable: " << name_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;
  const std::string& name() const { return name_; }
 private:
  std::string name_;
};

class BinaryOp : public Expression {
 public:
  BinaryOp(const std::string& op, const Expression* lhs, const Expression* rhs)
      : op_(op), lhs_(lhs), rhs_(rhs) {
#ifdef DEBUG
    std::cerr << "BinaryOp: " << op_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;
 private:
  const std::string op_;
  const Expression *lhs_, *rhs_;
};

class UnaryOp : public Expression {
 public:
  UnaryOp(const std::string& op, const Expression* expr)
      : op_(op), expr_(expr) {
#ifdef DEBUG
    std::cerr << "UnaryOp: " << op_ << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;
 private:
  const std::string op_;
  const Expression* expr_;
};

class Program {
 public:
  explicit Program(const std::vector<const ast::Expression*> body)
      : body_(body) {}
  llvm::Function* Codegen() const;
 private:
  std::vector<const ast::Expression*> body_;
};
}  // end namespace ast
}  // end namespace dolib

#endif
