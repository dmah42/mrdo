#include "threadpool.h"

#include <iostream>

ThreadPool::ThreadPool()
    : stop_(false) {
  size_t thread_count = std::thread::hardware_concurrency();
  std::cerr << "Initializing thread pool with " << thread_count << " threads\n";
  for (size_t i = 0; i < thread_count; ++i) {
    workers_.emplace_back(
        [this] {
          for(;;) {
            std::unique_lock<std::mutex> lock(this->queue_mutex_);
            while (!this->stop_ && this->tasks_.empty())
              this->condition_.wait(lock);
            if (this->stop_ && this->tasks_.empty())
              return;

            std::function<void()> task(this->tasks_.front());
            this->tasks_.pop();
            lock.unlock();

            task();
          }
        }
    );
  }
}

ThreadPool::~ThreadPool() {
  {
    std::unique_lock<std::mutex> lock(queue_mutex_);
    stop_ = true;
  }
  condition_.notify_all();
  for (auto& worker : workers_) {
    worker.join();
  }
}
