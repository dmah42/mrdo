#ifndef ENGINE_H_
#define ENGINE_H_

namespace llvm {
class Function;
class Module;
}

namespace engine {
extern llvm::Module* module;

void Initialize();
void Run();
void Optimize(llvm::Function* f);
void Dump();

void HandleFunc();
void HandleNative();
void HandleTopLevel();
}  // end namespace engine

#endif
