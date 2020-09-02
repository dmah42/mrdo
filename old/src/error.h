#ifndef ERROR_H_
#define ERROR_H_

#include <iostream>

#include "engine.h"
#include "lexer.h"

void ErrorCont();

template <typename H, typename... T>
void ErrorCont(const H& err, T&&... t) {
  std::cerr << err;
  ErrorCont(std::forward<T>(t)...);
}

template <typename H, typename... T>
void Error(lexer::Position position, const H& err, T&&... t) {
  std::cerr << engine::filename << ":" << position.line << ":" << position.col
            << ": error: " << err;
  ErrorCont(std::forward<T>(t)...);
}

#endif
