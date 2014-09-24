#include "ast/real.h"

#include <llvm/IR/Constants.h>
#include <llvm/IR/LLVMContext.h>

#include "llvm_type.h"

namespace ast {
llvm::Value* Real::Codegen() const {
  return llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(value_));
}

llvm::Type* Real::Type() const { return TypeMap<double>::get(); }
}  // end namespace ast
