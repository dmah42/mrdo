#include "debug_log.h"

void DebugLogCont() {
#ifdef DEBUG
  std::cerr << '\n';
#endif
}
