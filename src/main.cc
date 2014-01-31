#include <iostream>
#include <fstream>
#include <string>

#include "engine.h"

int main(int argc, const char* argv[]) {
  std::ifstream fs;
  if (argc > 1)
    fs.open(argv[1], std::ios::in);

  std::istream& f(fs.is_open() ? fs : std::cin);
  engine::Initialize(f);
  engine::Run();
  return 0;
}
