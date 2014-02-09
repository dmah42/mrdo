#include "ast/while.h"

#include "ast.h"

namespace ast {
llvm::Value* While::Codegen() const {
  // loopstart:
  //   br condition, loop, outloop
  // loop:
  //   ...
  //   body
  //   ...
  //   br loopstart
  // afterloop:

  llvm::Function* parent = builder.GetInsertBlock()->getParent();

  llvm::BasicBlock* loopstart_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "loopstart", parent);
  llvm::BasicBlock* loop_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "loop", parent);
  llvm::BasicBlock* after_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "afterloop", parent);

  PushNamedValueScope();
  builder.CreateBr(loopstart_block);
  builder.SetInsertPoint(loopstart_block);

  llvm::Value* cond = condition_->Codegen();
  if (!cond) return nullptr;
  cond = ToBool(cond);
  builder.CreateCondBr(cond, loop_block, after_block);

  builder.SetInsertPoint(loop_block);

  for (const ast::Expression* e : body_)
    if (!e->Codegen()) return nullptr;

  builder.CreateBr(loopstart_block);
  builder.SetInsertPoint(after_block);

  // remove the latest scope from the stack
  PopNamedValueScope();

  // for always returns 0.0
  return llvm::Constant::getNullValue(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}
}
