#include "ast/collection.h"

#include <llvm/IR/Constant.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/GlobalValue.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/LLVMContext.h>

#include "ast.h"
#include "ast/collection.h"
#include "ast/real.h"
#include "ast/variable.h"
#include "builtin.h"
#include "error.h"
#include "llvm_type.h"

namespace ast {
llvm::Value* Collection::Codegen() const {
  std::vector<llvm::Constant*> init_values;
  for (const Expression* e : values_) {
    llvm::Value* val = e->Codegen();
    const Real* e_real = dynamic_cast<const Real*>(e);
    const Collection* e_coll = dynamic_cast<const Collection*>(e);
    const Variable* e_var = dynamic_cast<const Variable*>(e);
    // TODO: variable should not be cast to a constant, it should be loaded and
    // the collection should be non-const.
    if (e_real || e_var) {
      init_values.push_back(llvm::cast<llvm::Constant>(val));
    } else if (e_coll) {
      // TODO
      Error(line, col, "Unimplemented collection of collection.");
      return nullptr;
    } else {
      Error(line,
            col,
            "Unimplemented expression type ",
            typeid(e).name(),
            " in collection.");
      return nullptr;
    }
  }

  llvm::ArrayType* array_type = llvm::ArrayType::get(
      llvm::Type::getDoubleTy(llvm::getGlobalContext()), init_values.size());

  llvm::GlobalVariable* gv = new llvm::GlobalVariable(
      *engine::module,
      array_type,
      true /*isConstant*/,
      llvm::GlobalValue::InternalLinkage,
      llvm::ConstantArray::get(array_type, init_values),
      is_sequence_ ? "seq" : "coll");
  if (!gv) {
    Error(line, col, "failed to create global variable for collection");
    return nullptr;
  }

  // TODO: make these internal errors.
  llvm::Value* gep_v = builder.CreateConstInBoundsGEP2_32(gv, 0, 0, "collptr");
  if (!gep_v) {
    Error(line, col, "failed to get pointer to global variable");
    return nullptr;
  }

  llvm::Value* array_size_v = llvm::ConstantInt::get(
      llvm::Type::getInt64Ty(llvm::getGlobalContext()), init_values.size());
  if (!array_size_v) {
    Error(line, col, "failed to get collection size");
    return nullptr;
  }

  llvm::AllocaInst* struct_ai =
      builder.CreateAlloca(Type(), nullptr, "colltmp");
  llvm::Value* struct_v = builder.CreateLoad(struct_ai, "collval");
  struct_v = builder.CreateInsertValue(struct_v, gep_v, {0}, "collval");
  struct_v = builder.CreateInsertValue(struct_v, array_size_v, {1}, "collval");
  return struct_v;
}

llvm::Type* Collection::Type() const {
  return TypeMap<builtin::Collection>::get();
}
}  // end namespace ast
