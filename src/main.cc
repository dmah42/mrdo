#include <string>

#include "engine.h"

int main(int argc, const char* argv[]) {
  dolib::engine::Initialize();
  dolib::engine::Run();
  return 0;
}
