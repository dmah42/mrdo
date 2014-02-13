#ifndef _DO_AST_PROTOTYPE_H_
#define _DO_AST_PROTOTYPE_H_

#include <cassert>
#include <string>
#include <vector>

#include <llvm/IR/Function.h>

#include "engine.h"
#include "error.h"
#include "llvm_type.h"

namespace ast {
class Prototype {
 public:
  Prototype(const std::string& name, std::vector<std::string> args)
      : name_(name), args_(args) {
#ifdef DEBUG
    std::cerr << "Prototype: " << name_ << "\n";
#endif
  }

  template <typename R> llvm::Function* Codegen() const;
  template <typename R, typename A0> llvm::Function* Codegen() const;
  template <typename R, typename A0, typename A1>
  llvm::Function* Codegen() const;

 private:
  llvm::Function* CodegenImpl(llvm::Type* ret,
                              std::vector<llvm::Type*> args) const;

  std::string name_;
  std::vector<std::string> args_;
};

template <typename R> llvm::Function* Prototype::Codegen() const {
  assert(args_.empty());

  std::vector<llvm::Type*> arg_types { };
  llvm::Type* rt = TypeMap<R>::get();
  if (!rt) {
    ErrorCont("Failed to get type for return type for prototype ", name_);
    return nullptr;
  }
  return CodegenImpl(rt, arg_types);
}

template <typename R, typename A0> llvm::Function* Prototype::Codegen() const {
  assert(args_.size() == 1);

  llvm::Type* t = TypeMap<A0>::get();
  if (!t) {
    ErrorCont("Failed to get llvm type for arg in prototype ", name_);
    return nullptr;
  }
  std::vector<llvm::Type*> arg_types { t };
  llvm::Type* rt = TypeMap<R>::get();
  if (!rt) {
    ErrorCont("Failed to get type for return type for prototype ", name_);
    return nullptr;
  }
  return CodegenImpl(rt, arg_types);
}

template <typename R, typename A0, typename A1>
llvm::Function* Prototype::Codegen() const {
  assert(args_.size() == 2);

  llvm::Type* t0 = TypeMap<A0>::get();
  if (!t0) {
    ErrorCont("Failed to get llvm type for arg0 in prototype ", name_);
    return nullptr;
  }
  llvm::Type* t1 = TypeMap<A1>::get();
  if (!t1) {
    ErrorCont("Failed to get llvm type for arg1 in prototype ", name_);
    return nullptr;
  }
  std::vector<llvm::Type*> arg_types { t0, t1 };
  llvm::Type* rt = TypeMap<R>::get();
  if (!rt) {
    ErrorCont("Failed to get type for return type for prototype ", name_);
    return nullptr;
  }
  return CodegenImpl(rt, arg_types);
}
}  // end namespace ast

#endif
