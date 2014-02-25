#ifndef _DO_AST_FUNC_H_
#define _DO_AST_FUNC_H_

#include "ast/expression.h"

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace llvm { class Function; }

namespace ast {
class Variable;

class Func : public Expression {
 public:
  explicit Func(const std::vector<std::string>& args,
                const std::vector<const ast::Expression*>& body)
      : args_(args), body_(body), f_(nullptr) {
    std::stringstream str;
    str << uid_++;
    name_ = "func" + str.str();
#ifdef DEBUG
    std::cerr << "Func: " << name_ << "\n";
#endif
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

  size_t num_args() const { return args_.size(); }

 private:
  void CreateArgumentAllocas(llvm::Function* f) const;

  static int uid_;
  std::string name_;
  std::vector<std::string> args_;
  std::vector<const ast::Expression*> body_;

  mutable llvm::Function* f_;
};
}  // end namespace ast

#endif
