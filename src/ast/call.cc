#include "ast/call.h"

#include <llvm/IR/Function.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/Module.h>

#include "ast.h"
#include "ast/collection.h"
#include "ast/func.h"
#include "ast/real.h"
#include "ast/variable.h"
#include "builtin.h"
#include "engine.h"
#include "error.h"
#include "llvm_type.h"

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
    const Call* arg_call = dynamic_cast<const Call*>(arg);
    const Func* arg_func = dynamic_cast<const Func*>(arg);

    llvm::Value* v = arg->Codegen();
    if (!v)
      return nullptr;
    if (arg_collection || arg_variable || arg_real || arg_call || arg_func) {
      argv.push_back(v);
    } else {
      Error(line, col, "unknown type for arg.");
      return nullptr;
    }
  }

  if (func->arg_size() != argv.size()) {
    Error(line,
          col,
          "expected ",
          func->arg_size(),
          " arguments to ",
          name_,
          ", got ",
          argv.size());
    return nullptr;
  }

  switch (func->getReturnType()->getTypeID()) {
    case llvm::Type::VoidTyID:
      return builder.CreateCall(func, argv, "");
    case llvm::Type::StructTyID:
    case llvm::Type::DoubleTyID:
      return builder.CreateCall(func, argv, "calltmp");
    default:
      Error(line, col, "Unknown return type: ");
      func->getReturnType()->dump();
      return nullptr;
  }
}

llvm::Type* Call::Type() const {
  llvm::Function* func = engine::module->getFunction(name_);
  if (!func) {
    ErrorCont("Unknown function: ", name_);
    return nullptr;
  }
  return func->getReturnType();
}
}  // end namespace ast
