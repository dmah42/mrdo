#ifndef _DO_BUILTIN_H_
#define _DO_BUILTIN_H_

#include <cstdlib>

namespace llvm { class ExecutionEngine; }

namespace builtin {
// TODO: support map and filter over collections of collections
typedef double (*MapFn)(double);
typedef bool (*FilterFn)(double);
typedef double (*FoldFn)(double, double);

struct Collection {
  double* values;
  size_t length;
};

void Initialize(llvm::ExecutionEngine* execution_engine);
}  // end namespace builtin

#endif
