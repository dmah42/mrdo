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
  f_ = nullptr;
  switch (args_.size()) {
    case 0:
      f_ = p.Codegen<double>();
      break;
    case 1:
      f_ = p.Codegen<double, double>();
      break;
    case 2:
      f_ = p.Codegen<double, double, double>();
      break;
    default:
      Error(position, "Unsupported number of args: ", args_.size());
      return nullptr;
  }
  if (!f_)
    return nullptr;

  llvm::BasicBlock* bb =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "entry", f_);
  llvm::BasicBlock* orig_bb = builder.GetInsertBlock();
  builder.SetInsertPoint(bb);

  PushNamedValueScope();
  CreateArgumentAllocas(f_);

  for (const Expression* e : body_) {
    llvm::Value* v = e->Codegen();
    if (!v) {
      f_->eraseFromParent();
      f_ = nullptr;
      PopNamedValueScope();
      return nullptr;
    }
  }
  PopNamedValueScope();

  builder.SetInsertPoint(orig_bb);

  llvm::verifyFunction(*f_);
  engine::Optimize(f_);

  llvm::AllocaInst* func_ai = builder.CreateAlloca(Type(), nullptr, "functmp");
  builder.CreateStore(f_, func_ai);
  return builder.CreateLoad(func_ai, "funcval");
}

llvm::Type* Func::Type() const {
  assert(f_);
  return f_->getType();
}

void Func::CreateArgumentAllocas(llvm::Function* f) const {
  llvm::Function::arg_iterator ai = f->arg_begin();
  for (const std::string& arg : args_) {
    // TODO: non-real argument types
    ast::Real* default_arg = new ast::Real(position, 0.0);
    llvm::Value* v = default_arg->Codegen();
    if (!v) {
      ErrorCont("Failed to create default for argument '",
                arg,
                "' for function '",
                name_,
                "'");
      return;
    }
    llvm::AllocaInst* alloca_v = CreateNamedVariable(f, arg, default_arg);
    builder.CreateStore(ai, alloca_v);
    ++ai;
  }
}
}  // end namespace ast
