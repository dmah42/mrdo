#ifndef _DO_AST_VARIABLE_H_
#define _DO_AST_VARIABLE_H_

#include "ast/expression.h"

#include <iostream>
#include <string>

#include "error.h"

namespace ast {
class Variable : public Expression {
 public:
  explicit Variable(const std::string& name) : name_(name), v_(nullptr) {
#ifdef DEBUG
    std::cerr << "Variable: " << name_ << "\n";
#endif
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

  const std::string& name() const { return name_; }

 private:
  std::string name_;

  mutable llvm::Value* v_;
};
}  // end namespace ast

#endif
