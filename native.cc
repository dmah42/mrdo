#include "native.h"

#include <cstdio>
#include <vector>

#include "ast.h"

// native methods to expose to do
extern "C" {
double printd(double d) {
  return putchar((char)d);
}
}

namespace native {
void Initialize() {
  (new ast::Prototype("printd", {"c"}))->Codegen();
}
}  // end namespace native
