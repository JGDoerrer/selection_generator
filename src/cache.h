#pragma once
#include <mutex>
#include <unordered_map>

template <typename F, typename G>
class Cache {
 private:
  std::unordered_map<F, G> cache;
  std::mutex mutex_cache;

 public:
  inline bool checkCondition(const F &key, const std::function<bool(const G)> &condition) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    return cache.find(key) != cache.end() && condition(cache.at(key));
  }

  inline void insertIfCondition(const F &key, const G &newValue, const std::function<bool(const G)> &condition) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    if (cache.find(key) == cache.end() || condition(cache.at(key))) {
      cache[key] = newValue;
    }
  }

  inline size_t size() {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    return cache.size();
  }
};
