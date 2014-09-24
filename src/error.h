#ifndef ERROR_H_
#define ERROR_H_

#include <iostream>

#include "engine.h"

void ErrorCont();

template <typename H, typename... T>
void ErrorCont(const H& err, T&&... t) {
  std::cerr << err;
  ErrorCont(std::forward<T>(t)...);
}

template <typename H, typename... T>
void Error(int line, int col, const H& err, T&&... t) {
  std::cerr << engine::filename << ":" << line << ":" << col
            << ": error: " << err;
  ErrorCont(std::forward<T>(t)...);
}

#endif
