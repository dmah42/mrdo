#ifndef ERROR_H_
#define ERROR_H_

#include <iostream>

void Error();

template <typename H, typename ...T>
void Error(const H& err, T&&... t) {
  std::cerr << err;
  Error(std::forward<T>(t)...);
}

#endif
