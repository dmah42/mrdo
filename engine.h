#ifndef _ENGINE_H_
#define _ENGINE_H_

namespace llvm {
class ExecutionEngine;
class FunctionPassManager;
class Module;
}

namespace engine {
extern llvm::ExecutionEngine* execution_engine;
extern llvm::FunctionPassManager* fpm;
extern llvm::Module* module;

void Initialize(bool opt);
void Run();
void Dump();

void HandleFunc();
void HandleNative();
void HandleTopLevel();
}  // end namespace engine

#endif
