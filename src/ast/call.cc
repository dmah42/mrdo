#include "ast/call.h"

#include <llvm/IR/Function.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/Module.h>

#include "ast.h"
#include "engine.h"
#include "error.h"

namespace ast {
llvm::Value* Call::Codegen() const {
  llvm::Function* func = engine::module->getFunction(name_);
  if (!func) {
    Error(line, col, "attempt to call unknown function: ", name_);
    return nullptr;
  }

  std::vector<llvm::Value*> argv;
  for (const auto& arg : args_) {
    // TODO: check types against expectation
    const Variable* arg_variable = dynamic_cast<const Variable*>(arg);
    const Collection* arg_collection = dynamic_cast<const Collection*>(arg);
    const Real* arg_real = dynamic_cast<const Real*>(arg);

    llvm::Value* v = arg->Codegen();
    if (!v) return nullptr;
    if (arg_collection || (arg_variable && v->getType()->isPointerTy())) {
      llvm::Value* gep_v =
          builder.CreateConstInBoundsGEP2_32(v, 0, 0, "collptr");
      if (!gep_v) {
        ErrorCont("failed to create GEP for collection in function call");
        return nullptr;
      }
      argv.push_back(gep_v);

      uint64_t num_elements = 0;
      if (arg_collection) {
        llvm::GlobalVariable* gv = dynamic_cast<llvm::GlobalVariable*>(v);
        if (!gv) {
          ErrorCont("expected global variable for collection");
          return nullptr;
        }
        num_elements = gv->getInitializer()->getType()->getArrayNumElements();
      } else {
        llvm::Type* array_type = v->getType()->getPointerElementType();
        if (!array_type) {
          ErrorCont("Expected variable of collection to be pointer type");
          return nullptr;
        }
        num_elements = array_type->getArrayNumElements();
      }
      llvm::Value* array_size_v = llvm::ConstantInt::get(
          llvm::Type::getInt64Ty(llvm::getGlobalContext()), num_elements);
      if (!array_size_v) {
        ErrorCont("failed to get collection size for function call");
        return nullptr;
      }

      argv.push_back(array_size_v);
    } else if (arg_real || arg_variable) {
      argv.push_back(v);
    } else {
      ErrorCont("unknown type for arg");
      return nullptr;
    }
  }

  if (func->arg_size() != argv.size()) {
    Error(line, col, "expected ", func->arg_size(), " arguments, got ",
          argv.size());
    return nullptr;
  }

  return builder.CreateCall(
      func, argv,
      (func->getReturnType()->getTypeID() == llvm::Type::VoidTyID) ? ""
                                                                   : "calltmp");
}
}  // end namespace ast
