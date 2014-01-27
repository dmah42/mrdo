#ifndef _DO_ENGINE_H_
#define _DO_ENGINE_H_

namespace llvm { class Module; }

namespace dolib {
namespace engine {
extern llvm::Module* module;

void Initialize();
void Run();
}  // end namespace engine
}  // end namespace dolib

#endif
