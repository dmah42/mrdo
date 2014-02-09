#ifndef _DO_AST_H_
#define _DO_AST_H_

#include <string>

#include <llvm/IR/IRBuilder.h>

namespace llvm {
class AllocaInst;
class Value;
}

namespace ast {
extern llvm::IRBuilder<> builder;

llvm::AllocaInst* GetNamedValue(const std::string& name);
void SetNamedValue(const std::string& name, llvm::AllocaInst* alloca);

void PushNamedValueScope();
void PopNamedValueScope();

llvm::Value* ToBool(llvm::Value* val);
}  // end namespace ast

#endif
