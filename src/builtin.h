#ifndef _DO_BUILTIN_H_
#define _DO_BUILTIN_H_

namespace llvm { class ExecutionEngine; }

namespace builtin {
void Initialize(llvm::ExecutionEngine* execution_engine);
}  // end namespace builtin

#endif
