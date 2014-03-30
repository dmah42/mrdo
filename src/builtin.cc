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
#include "threadpool.h"

namespace builtin {
namespace {
ThreadPool* thread_pool = nullptr;

/*
// TODO: input could be collection or sequence - different join behaviour on
// each when threaded
// TODO: input could be vector of vectors too

// TODO
std::vector<double> Filter(FilterFn fn, std::vector<double> input) {
  // TODO: thread
  std::vector<double> output;
  std::copy_if(input.begin(), input.end(), output.begin(), fn);
  return output;
}
*/

Collection Filter(FilterFn fn, Collection input) {
  std::vector<std::future<double>> to_filter;
  for (size_t i = 0; i < input.length; ++i) {
    to_filter.push_back(thread_pool->enqueue([fn, &input, i] {
      return fn(input.values[i]);
    }));
  }

  std::vector<double> filtered;
  for (size_t i = 0; i < to_filter.size(); ++i) {
    if (fabs(to_filter[i].get()) > 0)
      filtered.push_back(input.values[i]);
  }

  double* output = new double[filtered.size()];
  for (size_t i = 0; i < filtered.size(); ++i)
    output[i] = filtered[i];

  return {output, filtered.size()};
}

Collection Map(MapFn fn, Collection input) {
  std::vector<std::future<double>> results;
  for (size_t i = 0; i < input.length; ++i) {
    results.push_back(thread_pool->enqueue([fn, &input, i] {
      return fn(input.values[i]);
    }));
  }

  double* output = new double[input.length];
  for (size_t i = 0; i < results.size(); ++i)
    output[i] = results[i].get();

  return {output, input.length};
}

double Fold(FoldFn fn, Collection input) {
  double cum = 0.0;
  for (size_t i = 0; i < input.length; ++i)
    cum = fn(cum, input.values[i]);
  return cum;
}

double Length(Collection input) {
  return input.length;
}

Collection Read() {
  // TODO: read array of arrays (potentially) from stdin, return collection
  std::vector<double> input;
  double v;
  while (std::cin >> v)
    input.push_back(v);
  std::cin.clear();
  double* ret = new double[input.size()];
#ifdef DEBUG
  std::cerr << "-- " << input.size() << "\n";
#endif
  for (size_t i = 0; i < input.size(); ++i) {
    ret[i] = input[i];
#ifdef DEBUG
    std::cerr << "r[" << i << "]: " << ret[i] << "\n";
#endif
  }
#ifdef DEBUG
  std::cerr << "--\n";
#endif
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
  thread_pool = new ThreadPool();

  // TODO: macro?
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

#ifdef __GNUC__
  __extension__
#endif
  execution_engine->addGlobalMapping(
      (new ast::Prototype("length", {"input"}))->Codegen<double, Collection>(),
      reinterpret_cast<void*>(&Length));

#ifdef __GNUC__
  __extension__
#endif
    execution_engine->addGlobalMapping(
        (new ast::Prototype("fold", {"fn", "input"}))->
            Codegen<double, FoldFn, Collection>(),
        reinterpret_cast<void*>(&Fold));

#ifdef __GNUC__
  __extension__
#endif
    execution_engine->addGlobalMapping(
        (new ast::Prototype("map", {"fn", "input"}))->
            Codegen<Collection, MapFn, Collection>(),
        reinterpret_cast<void*>(&Map));

#ifdef __GNUC__
  __extension__
#endif
    execution_engine->addGlobalMapping(
        (new ast::Prototype("filter", {"fn", "input"}))->
            Codegen<Collection, FilterFn, Collection>(),
        reinterpret_cast<void*>(&Filter));
}

}  // end namespace builtin
