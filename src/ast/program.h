#ifndef _DO_AST_PROGRAM_H_
#define _DO_AST_PROGRAM_H_

#include <vector>

namespace llvm {
class Function;
}

namespace ast {
class Expression;

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
