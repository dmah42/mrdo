#include <iostream>

#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/Function.h>
#include <llvm/Module.h>

#include "ast.h"
#include "engine.h"
#include "lexer.h"
#include "parser.h"

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

void HandleExtern() {
  if (ast::Prototype* p = parser::Extern()) {
    std::cerr << "Parsed extern\n";
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

int main() {
  std::cout << "> ";

  lexer::Initialize();
  engine::Initialize();

  while (true) {
    std::cout << "> ";
    switch (lexer::current_token) {
      case lexer::TOKEN_EOF:  return 0;
      case ';': lexer::GetNextToken(); break;  // ignore top-level semicolons
      case lexer::TOKEN_FUNC: HandleFunc(); break;
      case lexer::TOKEN_EXTERN: HandleExtern(); break;
      default: HandleTopLevel(); break;
    }
  }
  engine::module->dump();
  return 0;
}
