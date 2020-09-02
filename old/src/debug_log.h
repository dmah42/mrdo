#ifndef DEBUG_LOG_H_
#define DEBUG_LOG_H_

#include <iostream>

#include "engine.h"
#include "lexer.h"

void DebugLogCont();

template <typename H, typename... T>
void DebugLogCont(const H& msg, T&&... t) {
#ifdef DEBUG
  std::cerr << msg;
#endif
  DebugLogCont(std::forward<T>(t)...);
}

template <typename H, typename... T>
void DebugLog(lexer::Position position, const H& msg, T&&... t) {
#ifdef DEBUG
  std::cerr << engine::filename << ":" << position.line << ":" << position.col
            << ": " << msg;
#endif
  DebugLogCont(std::forward<T>(t)...);
}

#endif

