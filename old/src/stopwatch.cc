#include "stopwatch.h"

std::string Stopwatch::Elapsed() const {
  const std::chrono::duration<double> elapsed = end_ - start_;

  if (elapsed.count() > 1) {
    return std::to_string(elapsed.count()) + " s";
  }
  if (elapsed.count() > 0.001) {
    return std::to_string(elapsed.count() * 1000) + " ms";
  }
  if (elapsed.count() > 0.000001) {
    return std::to_string(elapsed.count() * 1000000) + " us";
  }
  return std::to_string(elapsed.count() * 1000000000) + " ns";
}
