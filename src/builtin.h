#ifndef _DO_BUILTIN_H_
#define _DO_BUILTIN_H_

#include <cstdlib>

namespace llvm { class ExecutionEngine; }

namespace builtin {
struct Collection {
  double* values;
  size_t length;
};

void Initialize(llvm::ExecutionEngine* execution_engine);
}  // end namespace builtin

#endif
