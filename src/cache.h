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
    const auto temp = cache.find(key);
    return temp != cache.end() && condition(temp->second);
  }

  inline void insert(const F &key, const G &newValue) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    cache[key] = newValue;
  }

  inline void insertIfCondition(const F &key, const G &newValue, const std::function<bool(const G)> &condition) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    const auto temp = cache.find(key);
    if (temp == cache.end()) {
      cache[key] = newValue;
    } else if (condition(temp->second)) {
      temp->second = newValue;
    }
  }

  inline size_t size() {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    return cache.size();
  }
};
