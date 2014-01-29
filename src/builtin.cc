#include "builtin.h"

#include <algorithm>
#include <cstdio>
#include <functional>
#include <iostream>
#include <vector>

namespace builtin {
// TODO: support map and filter over collections of collections
// TODO: might be easier to allow collections of size 1 to be referenced as
// reals.
typedef std::function<double(double)> map_fn;
typedef std::function<bool(double)> filter_fn;

// TODO: input could be collection or sequence - different join behaviour on
// each when threaded
// TODO: input could be vector of vectors too
std::vector<double> do_map(map_fn fn, std::vector<double> input) {
  // TODO: thread
  std::vector<double> output;
  std::transform(input.begin(), input.end(), output.begin(), fn);
  return output;
}

// TODO: input could be collection or sequence - different join behaviour on
// each when threaded
std::vector<double> do_filter(filter_fn fn, std::vector<double> input) {
  // TODO: thread
  std::vector<double> output;
  std::copy_if(input.begin(), input.end(), output.begin(), fn);
  return output;
}

// TODO: how to read into collection vs seq?
std::vector<double> do_read() {
  // TODO: read array of arrays (potentially) from stdin, return collection
  return std::vector<double>();
}

void do_write(std::vector<double> input) {
  std::for_each(
      input.begin(), input.end(), [] (double v) { std::cout << v << "\n"; });
}
}  // end namespace builtin
