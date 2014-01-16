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
#include "parser.h"

namespace engine {
namespace {
const char prompt[] = "hup> ";
}

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

  std::cout << prompt << std::flush;

  lexer::Initialize();
}

void Run() {
  while (true) {
    std::cout << prompt << std::flush;
    switch (lexer::current_token) {
      case lexer::TOKEN_EOF:
        return;
      case ';':  // ignore top-level semicolons
        lexer::GetNextToken();
        break;
      case lexer::TOKEN_FUNC:
        engine::HandleFunc();
        break;
      case lexer::TOKEN_NATIVE:
        engine::HandleNative();
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
    std::cerr << "Parsed function\n";
    if (llvm::Function* lf = f->Codegen()) {
      std::cerr << "Read function:\n";
      lf->dump();
    }
    return;
  }
  lexer::GetNextToken();
}

void HandleNative() {
  if (ast::Prototype* p = parser::Native()) {
    std::cerr << "Parsed native\n";
    if (llvm::Function* lf = p->Codegen()) {
      std::cerr << "Read prototype:\n";
      lf->dump();
    }
    return;
  }
  lexer::GetNextToken();
}

void HandleTopLevel() {
  if (ast::Function* f = parser::TopLevel()) {
    std::cerr << "Parsed top-level expression\n";
    if (llvm::Function* lf = f->Codegen()) {
      std::cerr << "Read top-level expression:\n";
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

}
