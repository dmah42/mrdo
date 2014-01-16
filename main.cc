#include <iostream>

#include "engine.h"
#include "lexer.h"

// utility methods to expose to hup
extern "C"
double putd(double d) {
  putchar((char)d);
  return 0;
}

int main() {
  std::cout << "> ";

  engine::Initialize();

  while (true) {
    std::cout << "> ";
    switch (lexer::current_token) {
      case lexer::TOKEN_EOF:  return 0;
      case ';': lexer::GetNextToken(); break;  // ignore top-level semicolons
      case lexer::TOKEN_FUNC: engine::HandleFunc(); break;
      case lexer::TOKEN_EXTERN: engine::HandleExtern(); break;
      default: engine::HandleTopLevel(); break;
    }
  }

  engine::Dump();
  return 0;
}
