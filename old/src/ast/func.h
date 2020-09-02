#ifndef _DO_AST_FUNC_H_
#define _DO_AST_FUNC_H_

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "ast/expression.h"
#include "debug_log.h"

namespace llvm {
class Function;
}

namespace ast {
class Variable;

class Func : public Expression {
 public:
  explicit Func(lexer::Position position,
                const std::vector<std::string>& args,
                const std::vector<const ast::Expression*>& body)
      : Expression(position), args_(args), body_(body), f_(nullptr) {
    std::stringstream str;
    str << uid_++;
    name_ = "func" + str.str();
    DebugLog(position, "Func: ", name_);
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
