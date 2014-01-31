#ifndef _DO_ENGINE_H_
#define _DO_ENGINE_H_

#include <iostream>

namespace llvm { class Module; }

namespace engine {
extern llvm::Module* module;
extern std::istream* file;

void Initialize(std::istream& f);
void Run();
}  // end namespace engine

#endif
