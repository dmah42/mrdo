#include "ast/collection.h"

#include <llvm/IR/Constant.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/GlobalValue.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/LLVMContext.h>

#include "ast/real.h"
#include "error.h"

namespace ast {
llvm::Value* Collection::Codegen() const {
  std::vector<llvm::Constant*> init_values;
  for (const Expression* e : values_) {
    llvm::Value* val = e->Codegen();
    const Real* e_real = dynamic_cast<const Real*>(e);
    const Collection* e_coll = dynamic_cast<const Collection*>(e);
    if (e_real) {
      init_values.push_back(llvm::cast<llvm::Constant>(val));
    } else if (e_coll) {
      // TODO
      Error(line, col, "Unimplemented collection of collection.");
      return nullptr;
    } else {
      Error(line, col, "Unimplemented expression type ", typeid(e).name(),
            " in collection.");
      return nullptr;
    }
  }

  llvm::ArrayType* array_type = llvm::ArrayType::get(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()), init_values.size());

  return new llvm::GlobalVariable(
      *engine::module, array_type, true /*isConstant*/,
      llvm::GlobalValue::InternalLinkage,
      llvm::ConstantArray::get(array_type, init_values),
      is_sequence_ ? "seq" : "coll");
}
}  // end namespace ast
