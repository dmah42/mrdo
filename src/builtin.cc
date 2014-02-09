#include "builtin.h"

#include <algorithm>
#include <cstdio>
#include <functional>
#include <iostream>
#include <vector>

#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/GlobalValue.h>

#include "ast.h"

namespace builtin {
namespace {
// TODO: support map and filter over collections of collections
// TODO: might be easier to allow collections of size 1 to be referenced as
// reals.
typedef std::function<double(double)> map_fn;
typedef std::function<bool(double)> filter_fn;
/*
// TODO: input could be collection or sequence - different join behaviour on
// each when threaded
// TODO: input could be vector of vectors too
std::vector<double> Map(map_fn fn, std::vector<double> input) {
  // TODO: thread
  std::vector<double> output;
  std::transform(input.begin(), input.end(), output.begin(), fn);
  return output;
}

// TODO: input could be collection or sequence - different join behaviour on
// each when threaded
std::vector<double> Filter(filter_fn fn, std::vector<double> input) {
  // TODO: thread
  std::vector<double> output;
  std::copy_if(input.begin(), input.end(), output.begin(), fn);
  return output;
}

std::vector<double> Read() {
  // TODO: read array of arrays (potentially) from stdin, return collection
  std::vector<double> input;
  while (std::cin && std::cin.peek() != EOF) {
    double v;
    std::cin >> v;
    input.push_back(v);
  }
  return input;
}
*/

void Write(double* input, size_t input_len) {
  std::cout << "[ ";
  for (size_t i = 0; i < input_len; ++i) {
    std::cout << input[i];
    if (i != input_len - 1)
      std::cout << ", ";
  }
  std::cout << " ]\n";
}
}  // end namespace

void Initialize(llvm::ExecutionEngine* execution_engine) {
#ifdef __GNUC__
  __extension__
#endif
  execution_engine->addGlobalMapping(
      (new ast::Prototype("write", {"input", "input_len"}))->
          Codegen<void, double*, size_t>(),
      reinterpret_cast<void*>(&Write));
}

}  // end namespace builtin
