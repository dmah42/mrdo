#include <string>

#include "engine.h"

int main(int argc, const char* argv[]) {
  // TODO: flags
  bool dbg = argc > 1 && std::string(argv[1]) == "debug";
  engine::Initialize(!dbg);
  engine::Run();
  engine::Dump();
  return 0;
}
