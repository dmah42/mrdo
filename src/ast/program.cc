#include "ast/program.h"

#include <llvm/Analysis/Verifier.h>
#include <llvm/IR/Function.h>

#include "ast.h"
#include "ast/expression.h"
#include "engine.h"
#include "error.h"
#include "lexer.h"

namespace ast {
llvm::Function* Program::Codegen() const {
  PushNamedValueScope();
  // No return type, no parameters.
  llvm::FunctionType* ft =
      llvm::FunctionType::get(llvm::Type::getVoidTy(llvm::getGlobalContext()),
                              std::vector<llvm::Type*>(), false);
  llvm::Function* f = llvm::Function::Create(
      ft, llvm::Function::ExternalLinkage, "global", engine::module);

  if (f->getName() != "global") {
    f->eraseFromParent();
    Error(lexer::line, lexer::col, "Failed to create function.");
    return nullptr;
  }

  // function body
  llvm::BasicBlock* bb =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "entry", f);
  builder.SetInsertPoint(bb);

  for (const Expression* e : body_) {
    // TODO: find the return expressions.
    llvm::Value* v = e->Codegen();
    if (!v) {
      f->eraseFromParent();
      return nullptr;
    }
  }

  builder.CreateRetVoid();

  PopNamedValueScope();

  llvm::verifyFunction(*f);
  return f;
}
}  // end namespace ast
