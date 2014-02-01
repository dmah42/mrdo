#include <iostream>
#include <fstream>
#include <string>

#include "engine.h"

int main(int argc, const char* argv[]) {
  std::string file = argc > 1 ? argv[1] : std::string();
  engine::Initialize(file);
  engine::Run();
  return 0;
}
