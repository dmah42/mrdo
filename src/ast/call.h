#ifndef _DO_AST_CALL_H_
#define _DO_AST_CALL_H_

#include <iostream>
#include <string>
#include <vector>

#include "ast/expression.h"

namespace ast {
class Call : public Expression {
 public:
  Call(const std::string& name, std::vector<const Expression*>& args)
      : name_(name), args_(args) {
#ifdef DEBUG
    std::cerr << "Call: " << name_ << "\n";
#endif
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

  const std::string& name() const { return name_; }

 private:
  std::string name_;
  std::vector<const Expression*> args_;
};
}  // end namespace ast

#endif
