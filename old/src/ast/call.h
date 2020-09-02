#ifndef _DO_AST_CALL_H_
#define _DO_AST_CALL_H_

#include <iostream>
#include <string>
#include <vector>

#include "ast/expression.h"
#include "debug_log.h"

namespace ast {
class Call : public Expression {
 public:
  Call(lexer::Position position,
       const std::string& name,
       std::vector<const Expression*>& args)
      : Expression(position), name_(name), args_(args) {
    DebugLog(position, "Call: ", name);
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
