#include "llvm_type.h"

#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Type.h>

template <> llvm::Type* TypeMap<void>::get() {
  return llvm::Type::getVoidTy(llvm::getGlobalContext());
}

template <> llvm::Type* TypeMap<double>::get() {
  return llvm::Type::getDoubleTy(llvm::getGlobalContext());
}

template <> llvm::Type* TypeMap<double*>::get() {
  return llvm::PointerType::getUnqual(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()));
}

template <> llvm::Type* TypeMap<size_t>::get() {
  return llvm::Type::getInt64Ty(llvm::getGlobalContext());
}
