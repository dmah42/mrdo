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

#include "ast.h"
#include "lexer.h"
#include "native.h"
#include "parser.h"

namespace engine {
namespace {
const char prompt[] = "do] ";
}

llvm::ExecutionEngine* execution_engine = nullptr;
llvm::FunctionPassManager* fpm = nullptr;
llvm::Module* module = nullptr;

void Initialize(bool opt) {
  llvm::InitializeNativeTarget();

  module = new llvm::Module("do jit", llvm::getGlobalContext());
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
  if (opt) {
    fpm->add(new llvm::DataLayout(*(execution_engine->getDataLayout())));
    fpm->add(llvm::createBasicAliasAnalysisPass());
    fpm->add(llvm::createCFGSimplificationPass());
    fpm->add(llvm::createGVNPass());
    fpm->add(llvm::createInstructionCombiningPass());
    fpm->add(llvm::createPromoteMemoryToRegisterPass());
    fpm->add(llvm::createReassociatePass());
  }
  fpm->doInitialization();

  native::Initialize();

 // std::cout << prompt << std::flush;

  lexer::Initialize();
}

void Run() {
  // TODO: if everything is top-level, this should be much simpler.
  while (lexer::current_token != lexer::TOKEN_EOF) {
//    std::cout << prompt << std::flush;
    switch (lexer::current_token) {
      case ';':  // ignore top-level semicolons
        lexer::GetNextToken();
        break;
      case lexer::TOKEN_FUNC:
        engine::HandleFunc();
        break;
      default:
        engine::HandleTopLevel();
        break;
    }
  }
}

void Dump() {
  module->dump();
}

void HandleFunc() {
  if (ast::Function* f = parser::Function()) {
    if (llvm::Function* lf = f->Codegen()) {
      std::cerr << "Function:\n";
      lf->dump();
    }
    return;
  }
  lexer::GetNextToken();
}

// TODO: remove requirement for top level expression to end in ';'
void HandleTopLevel() {
  // TODO: remove requirement for file to end in done;
  if (ast::Function* f = parser::TopLevel()) {
    if (llvm::Function* lf = f->Codegen()) {
      std::cerr << "Top-level expression:\n";
      lf->dump();

      // JIT
      void* fptr = engine::execution_engine->getPointerToFunction(lf);

      double (*fp)() = (double (*)())(intptr_t)fptr;
      std::cerr << "Evaluated to " << fp() << "\n";
    }
    return;
  }
  lexer::GetNextToken();
}
}  // end namespace engine
