#include "builtin.h"

#include <algorithm>
#include <cstdio>
#include <functional>
#include <iostream>
#include <vector>

#include <llvm/ExecutionEngine/ExecutionEngine.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/GlobalValue.h>

#include "ast/prototype.h"

namespace builtin {
namespace {
// TODO: support map and filter over collections of collections
typedef double (*map_fn)(double);
typedef bool (*filter_fn)(double);

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
*/

Collection Read() {
  // TODO: read array of arrays (potentially) from stdin, return collection
  std::vector<double> input;
  while (std::cin && std::cin.peek() != EOF) {
    double v;
    std::cin >> v;
    if (std::cin.get() == EOF) break;
    input.push_back(v);
  }
  double* ret = new double[input.size()];
  std::cerr << "-- " << input.size() << "\n";
  for (size_t i = 0; i < input.size(); ++i) {
    ret[i] = input[i];
    std::cerr << "r[" << i << "]: " << ret[i] << "\n";
  }
  std::cerr << "--\n";
  return {ret, input.size()};
}

void Write(Collection input) {
  std::cout << "[ ";
  for (size_t i = 0; i < input.length; ++i) {
    std::cout << input.values[i];
    if (i != input.length - 1) std::cout << ", ";
  }
  std::cout << " ]\n";
}
}  // end namespace

void Initialize(llvm::ExecutionEngine* execution_engine) {
#ifdef __GNUC__
  __extension__
#endif
  execution_engine->addGlobalMapping(
      (new ast::Prototype("write", {"input"}))->Codegen<void, Collection>(),
      reinterpret_cast<void*>(&Write));

#ifdef __GNUC__
  __extension__
#endif
  execution_engine->addGlobalMapping(
      (new ast::Prototype("read", {}))->Codegen<Collection>(),
      reinterpret_cast<void*>(&Read));
}

}  // end namespace builtin
