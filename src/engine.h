#ifndef _DO_ENGINE_H_
#define _DO_ENGINE_H_

#include <string>

namespace llvm {
  class Function;
  class Module;
}

namespace engine {
extern llvm::Module* module;
extern std::string filename;
extern std::istream* stream;

void Initialize(const std::string& f);
void Optimize(llvm::Function* f);
bool Run(bool dump);
}  // end namespace engine

#endif
