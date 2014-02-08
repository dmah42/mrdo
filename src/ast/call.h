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
  virtual llvm::Value* Codegen() const;

 private:
  std::string name_;
  std::vector<const Expression*> args_;
};
}  // end namespace ast

#endif
