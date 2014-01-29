#ifndef _DO_ENGINE_H_
#define _DO_ENGINE_H_

namespace llvm { class Module; }

namespace engine {
extern llvm::Module* module;

void Initialize();
void Run();
}  // end namespace engine

#endif
