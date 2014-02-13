#include "ast/prototype.h"

#include <llvm/IR/Module.h>

namespace ast {
llvm::Function* Prototype::CodegenImpl(llvm::Type* ret,
                                       std::vector<llvm::Type*> args) const {
  llvm::FunctionType* ft = llvm::FunctionType::get(ret, args, false);
  llvm::Function* f = llvm::Function::Create(
      ft, llvm::Function::ExternalLinkage, name_, engine::module);

  if (f->getName() != name_) {
    f->eraseFromParent();
    f = engine::module->getFunction(name_);

    if (!f->empty()) {
      ErrorCont("redefinition of function ", name_);
      return nullptr;
    }

    if (f->arg_size() != args_.size()) {
      ErrorCont("redefinition of function ", name_,
                " with mismatch arg length");
      return nullptr;
    }
  }

  llvm::Function::arg_iterator ai = f->arg_begin();
  for (size_t i = 0; i < args_.size(); ++i, ++ai) {
    const llvm::Type::TypeID ai_type_id = ai->getType()->getTypeID();
    const llvm::Type::TypeID arg_type_id = args[i]->getTypeID();
    if (ai_type_id != arg_type_id) {
      ErrorCont("redefinition of function ", name_,
                " with mismatched types for arg ", i, ": ");
      ai->getType()->dump();
      args[i]->dump();
    }
    ai->setName(args_[i]);
  }
  return f;
}
}  // end namespace ast
