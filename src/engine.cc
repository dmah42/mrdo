#include "engine.h"
#include "stopwatch.h"

#include <chrono>
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

#include "ast/program.h"
#include "builtin.h"
#include "lexer.h"
#include "parser/program.h"

namespace engine {
namespace {
llvm::ExecutionEngine* execution_engine = nullptr;
std::unique_ptr<llvm::FunctionPassManager> fpm;
std::ifstream input_file;
}  // namespace

llvm::Module* module = nullptr;
std::string filename;
bool optimize = false;

std::istream* stream = &std::cin;

void Initialize(const std::string& f, bool o) {
  filename = f;
  optimize = o;
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

  if (optimize) {
    fpm.reset(new llvm::FunctionPassManager(module));
    fpm->add(new llvm::DataLayout(*(execution_engine->getDataLayout())));
    fpm->add(llvm::createBasicAliasAnalysisPass());
    fpm->add(llvm::createCFGSimplificationPass());
    fpm->add(llvm::createGVNPass());
    fpm->add(llvm::createInstructionCombiningPass());
    fpm->add(llvm::createPromoteMemoryToRegisterPass());
    fpm->add(llvm::createReassociatePass());
    fpm->doInitialization();
  }

  if (engine::filename.empty()) {
    std::cerr << "do] ";
  }
  builtin::Initialize(execution_engine);
  lexer::Initialize();
}

void Optimize(llvm::Function* f) {
  if (optimize) {
    std::cerr << "Optimizing '" << f->getName().str() << "'\n";
    fpm->run(*f);
  }
}

bool Run(bool dump) {
  if (ast::Program* p = parser::Program()) {
    if (llvm::Function* lf = p->Codegen()) {
      Optimize(lf);

      // TODO: module optimizations - ConstantMerge.
      if (dump)
        module->dump();
      void* fptr = engine::execution_engine->getPointerToFunction(lf);
      void (*fp)() = (void (*)())(intptr_t)fptr;

      std::cerr << "Running... \n";

      Stopwatch sw;
      sw.Start();
      fp();
      sw.End();
      std::cerr << "... done in " << sw.Elapsed() << "\n";

      // TODO: write out to
      // raw_fd_ostream f(outpath...);
      // llvm::WriteBitcodeToFile(module, f);

      return true;
    } else {
      std::cerr << "Failed to codegen.\n";
      return false;
    }
  }
  std::cerr << "Failed to parse.\n";
  return false;
}
}  // end namespace engine
