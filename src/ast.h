#ifndef _DO_AST_H_
#define _DO_AST_H_

#include <iostream>
#include <string>
#include <vector>

namespace llvm {
class Function;
class Value;
}

namespace ast {
class Expression {
 public:
  virtual ~Expression() {}
  virtual llvm::Value* Codegen() const = 0;
  mutable int line_no;
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
  const Expression* lhs_, *rhs_;
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

class If : public Expression {
 public:
  // TODO: elif
  If(const Expression* condition, std::vector<const Expression*>& if_body,
     std::vector<const Expression*>& else_body)
      : condition_(condition), if_(if_body), else_(else_body) {}
  virtual llvm::Value* Codegen() const;

 private:
  const Expression* condition_;
  std::vector<const Expression*> if_;
  std::vector<const Expression*> else_;
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

#endif
