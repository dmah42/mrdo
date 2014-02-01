#include "engine.h"

#include <iostream>
#include <fstream>

#include <llvm/Analysis/Passes.h>
#include <llvm/Bitcode/ReaderWriter.h>
#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/ExecutionEngine/JIT.h>
#include <llvm/IR/DataLayout.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>
#include <llvm/PassManager.h>
#include <llvm/Support/TargetSelect.h>
#include <llvm/Transforms/Scalar.h>

#include "ast.h"
#include "lexer.h"
#include "parser.h"

namespace engine {
namespace {
llvm::ExecutionEngine* execution_engine = nullptr;
llvm::FunctionPassManager* fpm = nullptr;
std::ifstream input_file;
}
llvm::Module* module = nullptr;
std::string filename;
std::istream* stream = &std::cin;

void Initialize(const std::string& f) {
  filename = f;
  if (!f.empty()) {
    input_file.open(f, std::ios::in);
    if (!input_file.is_open()) {
      std::cerr << "Failed to open file: '" << f << "'.\n";
      exit(1);
    }
    stream = &input_file;
  }

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

#ifndef DEBUG
  fpm = new llvm::FunctionPassManager(module);
  fpm->add(new llvm::DataLayout(*(execution_engine->getDataLayout())));
  fpm->add(llvm::createBasicAliasAnalysisPass());
  fpm->add(llvm::createCFGSimplificationPass());
  fpm->add(llvm::createGVNPass());
  fpm->add(llvm::createInstructionCombiningPass());
  fpm->add(llvm::createPromoteMemoryToRegisterPass());
  fpm->add(llvm::createReassociatePass());
  fpm->doInitialization();
#endif

  if (engine::filename.empty()) {
    std::cerr << "do] ";
  }
  lexer::Initialize();
}

void Run() {
  if (ast::Program* p = parser::Program()) {
    if (llvm::Function* lf = p->Codegen()) {

      if (fpm) {
        //    lf->dump();
        //    std::cerr << "Optimizing...\n";
        fpm->run(*lf);
        //     lf->dump();
      }

      void* fptr = engine::execution_engine->getPointerToFunction(lf);
      double(*fp)() = (double(*)())(intptr_t) fptr;

      std::cerr << "Evaluates to: " << fp() << "\n";
    } else {
      std::cerr << "Failed to codegen.\n";
    }
  } else {
    std::cerr << "Failed to parse.\n";
  }
  // TODO: write out to 
  // raw_fd_ostream f(outpath...);
  //llvm::WriteBitcodeToFile(module, f);
  // TODO: add flag to dump module
  //module->dump();
}
}  // end namespace engine
