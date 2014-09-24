#include "ast/if.h"

#include <llvm/IR/Function.h>

#include "ast.h"

namespace ast {
llvm::Value* If::Codegen() const {
  llvm::Value* condition_value = condition_->Codegen();
  if (!condition_value)
    return nullptr;
  condition_value = ToBool(condition_value);

  llvm::Function* parent = builder.GetInsertBlock()->getParent();

  // create if and else blocks
  llvm::BasicBlock* if_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "if", parent);
  llvm::BasicBlock* else_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "else");
  llvm::BasicBlock* merge_block =
      llvm::BasicBlock::Create(llvm::getGlobalContext(), "merge");

  if (else_.empty())
    builder.CreateCondBr(condition_value, if_block, merge_block);
  else
    builder.CreateCondBr(condition_value, if_block, else_block);

  // emit if block
  builder.SetInsertPoint(if_block);

  PushNamedValueScope();
  for (const Expression* e : if_) {
    llvm::Value* value = e->Codegen();
    if (!value)
      return nullptr;
  }
  PopNamedValueScope();

  builder.CreateBr(merge_block);
  if_block = builder.GetInsertBlock();

  // emit else block
  if (!else_.empty()) {
    parent->getBasicBlockList().push_back(else_block);
    builder.SetInsertPoint(else_block);

    PushNamedValueScope();
    for (const Expression* e : else_) {
      llvm::Value* value = e->Codegen();
      if (!value)
        return nullptr;
    }
    PopNamedValueScope();

    builder.CreateBr(merge_block);
    else_block = builder.GetInsertBlock();
  }

  parent->getBasicBlockList().push_back(merge_block);
  builder.SetInsertPoint(merge_block);

  return llvm::Constant::getNullValue(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}
}  // end namespace ast
