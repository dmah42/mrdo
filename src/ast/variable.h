#ifndef _DO_AST_VARIABLE_H_
#define _DO_AST_VARIABLE_H_

#include "ast/expression.h"

#include <iostream>
#include <string>

#include "error.h"

namespace ast {
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
}  // end namespace ast

#endif
