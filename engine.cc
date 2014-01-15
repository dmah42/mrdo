#include "engine.h"

#include <iostream>

#include <llvm/Analysis/Passes.h>
#include <llvm/DataLayout.h>
#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/ExecutionEngine/JIT.h>
#include <llvm/Function.h>
#include <llvm/LLVMContext.h>
#include <llvm/Module.h>
#include <llvm/PassManager.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/Transforms/Scalar.h>

namespace engine {
llvm::ExecutionEngine* execution_engine = nullptr;
llvm::FunctionPassManager* fpm = nullptr;
llvm::Module* module = nullptr;

void Initialize() {
  llvm::InitializeNativeTarget();

  module = new llvm::Module("kaleidoscope jit", llvm::getGlobalContext());
  if (!module) {
    std::cerr << "Failed to create module\n";
    exit(1);
  }

  std::string err_str;
  execution_engine = llvm::EngineBuilder(module).setErrorStr(&err_str).create();
  if (!execution_engine) {
    std::cerr << "Failed to create execution engine: " << err_str << "\n";
    exit(1);
  }

  fpm = new llvm::FunctionPassManager(module);
  fpm->add(new llvm::DataLayout(*(execution_engine->getDataLayout())));
  fpm->add(llvm::createBasicAliasAnalysisPass());
  fpm->add(llvm::createInstructionCombiningPass());
  fpm->add(llvm::createReassociatePass());
  fpm->add(llvm::createGVNPass());
  fpm->add(llvm::createCFGSimplificationPass());
  fpm->doInitialization();
}
}
