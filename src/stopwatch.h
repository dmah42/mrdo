#ifndef _DO_STOPWATCH_H_
#define _DO_STOPWATCH_H_

#include <chrono>
#include <string>

class Stopwatch {
 public:
  void Start() { start_ = std::chrono::system_clock::now(); }

  void End() { end_ = std::chrono::system_clock::now(); }

  std::string Elapsed() const;

 private:
  std::chrono::time_point<std::chrono::system_clock> start_;
  std::chrono::time_point<std::chrono::system_clock> end_;
};

#endif  // _DO_STOPWATCH_H_
