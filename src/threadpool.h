#ifndef _DO_THREADPOOL_H_
#define _DO_THREADPOOL_H_

#include <assert.h>
#include <stdlib.h>

#include <future>
#include <queue>
#include <vector>

class ThreadPool {
 public:
  ThreadPool();
  ~ThreadPool();

  template <class F, class... Args>
  auto enqueue(F&& f, Args&&... args)
      -> std::future<typename std::result_of<F(Args...)>::type>;

 private:
  std::vector<std::thread> workers_;
  std::queue<std::function<void()>> tasks_;

  std::mutex queue_mutex_;
  std::condition_variable condition_;
  bool stop_;
};

template <class F, class... Args>
auto ThreadPool::enqueue(F&& f, Args&&... args)
    -> std::future<typename std::result_of<F(Args...)>::type> {
  typedef typename std::result_of<F(Args...)>::type return_type;

  assert(!stop_);

  auto task = std::make_shared<std::packaged_task<return_type()>>(
      std::bind(std::forward<F>(f), std::forward<Args>(args)...));

  std::future<return_type> res = task->get_future();
  {
    std::unique_lock<std::mutex> lock(queue_mutex_);
    tasks_.push([task]() { (*task)(); });
  }
  condition_.notify_one();
  return res;
}

#endif  // _DO_THREADPOOL_H_
