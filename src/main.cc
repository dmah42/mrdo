#include <string>

#include "engine.h"

int main(int argc, const char* argv[]) {
  // TODO: pass stream 'stdin' or named from args.
  engine::Initialize();
  engine::Run();
  return 0;
}
