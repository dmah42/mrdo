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
  const std::string& name() const { return name_; }
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
  If(const Expression* condition,
     std::vector<const Expression*>& _if,
     std::vector<const Expression*>& _else)
    : condition_(condition), if_(_if), else_(_else) {}
  virtual llvm::Value* Codegen() const;
 private:
  const Expression* condition_;
  std::vector<const Expression*> if_;
  std::vector<const Expression*> else_;
};

class For : public Expression {
 public:
  For(const std::string& var, const Expression* start, const Expression* end,
      const Expression* step, std::vector<const Expression*>& body)
      : var_(var), start_(start), end_(end), step_(step), body_(body) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string var_;
  const Expression *start_, *end_, *step_;
  std::vector<const Expression*> body_;
};

class Var : public Expression {
 public:
  Var(std::string& name, const Expression* init)
      : name_(name), init_(init) {}
  virtual llvm::Value* Codegen() const;
 private:
  std::string name_;
  const Expression* init_;
};

class Prototype {
 public:
  Prototype(const std::string& name, const std::vector<std::string>& args)
    : name_(name), args_(args) {}
  llvm::Function* Codegen() const;
  const std::vector<std::string>& args() const { return args_; }
 private:
  std::string name_;
  std::vector<std::string> args_;
};

class Function {
 public:
  Function(Prototype* prototype, std::vector<const Expression*> body)
    : prototype_(prototype), body_(body) {}
  llvm::Function* Codegen() const;
 private:
  void CreateArgumentAllocas(llvm::Function* f) const;

  Prototype* prototype_;
  std::vector<const Expression*> body_;
};
}  // end namespace ast

#endif
