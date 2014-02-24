#include "ast/func.h"

#include <algorithm>

#include <llvm/Analysis/Verifier.h>
#include <llvm/IR/LLVMContext.h>

#include "ast.h"
#include "ast/prototype.h"
#include "ast/real.h"
#include "ast/variable.h"
#include "builtin.h"
#include "engine.h"
#include "error.h"

namespace ast {
// static
int Func::uid_ = 0;

llvm::Value* Func::Codegen() const {
  Prototype p(name_, args_);
  // TODO: non-real arguments
  llvm::Function* f = nullptr;
  switch (args_.size()) {
    case 0:
      f = p.Codegen<double>();
      break;
    case 1:
      f = p.Codegen<double, double>();
      break;
    case 2:
      f = p.Codegen<double, double, double>();
      break;
    default:
      Error(line, col, "Unsupported number of args: ", args_.size());
      return nullptr;
  }
  if (!f) return nullptr;

  llvm::BasicBlock* bb =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "entry", f);
  llvm::BasicBlock* orig_bb = builder.GetInsertBlock();
  builder.SetInsertPoint(bb);

  PushNamedValueScope();
  CreateArgumentAllocas(f);

  for (const Expression* e : body_) {
    llvm::Value* v = e->Codegen();
    if (!v) {
      f->eraseFromParent();
      PopNamedValueScope();
      return nullptr;
    }
  }
  PopNamedValueScope();

  builder.SetInsertPoint(orig_bb);

  llvm::verifyFunction(*f);
  engine::Optimize(f);

  llvm::AllocaInst* func_ai =
      builder.CreateAlloca(f->getType(), nullptr, "functmp");
  builder.CreateStore(f, func_ai);
  return builder.CreateLoad(func_ai, "funcval");
}

void Func::CreateArgumentAllocas(llvm::Function* f) const {
  llvm::Function::arg_iterator ai = f->arg_begin();
  for (const std::string& arg : args_) {
    // TODO: non-real argument types
    std::pair<llvm::AllocaInst*, llvm::Value*> alloca_v =
        CreateNamedVariable(f, arg, new ast::Real(0.0));
    builder.CreateStore(ai, alloca_v.first);
    ++ai;
  }
}
}  // end namespace ast
