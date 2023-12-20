#pragma once
#include <mutex>
#include <unordered_map>

// #define PosetShadowing
// #define PosetShadowingRemove

// für n = 10, i = 4:
// 7.781s (ohne):
// (cache_l: 2059324, cache_u: 288777, noSol: 46, bruteForce: 73888), cache = (64427 + 5100 = 69527)
// 24.631s (mit PosetShadowing):
// (cache_l: 1257853, cache_u: 228493, noSol: 28, bruteForce: 46194), cache = (43340 + 4422 = 47762)
// 33.561s (mit Beidem):
// (cache_l: 1257853, cache_u: 228493, noSol: 28, bruteForce: 46194), cache = (42466 + 4422 = 46888)

template <typename F, typename G>
class Cache {
 private:
  std::unordered_map<F, G> cache;
  std::mutex mutex_cache;

 public:
  inline std::optional<G> get(const F &poset) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    const auto temp = cache.find(poset);
    if (temp == cache.end()) {
      return {};
    } else {
      return *temp;
    }
  }

  inline bool checkLower(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    const auto temp = cache.find(poset);
    if (temp != cache.end() && remainingComparisons <= temp->second) {
      return true;
    }
#ifdef PosetShadowing
    for (const auto &it : cache) {
      if (remainingComparisons <= it.second && poset.subset_of(it.first)) {
        return true;
      }
    }
#endif
    return false;
  }

  inline bool checkLower2(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    for (const auto &it : cache) {
      if (remainingComparisons <= it.second && poset.subset_of(it.first)) {
        return true;
      }
    }
    return false;
  }

  inline bool checkUpper(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    const auto temp = cache.find(poset);
    if (temp != cache.end() && remainingComparisons >= temp->second) {
      return true;
    }
#ifdef PosetShadowing
    for (const auto &it : cache) {
      if (remainingComparisons >= it.second && it.first.subset_of(poset)) {
        return true;
      }
    }
#endif
    return false;
  }

  inline bool checkUpper2(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    for (const auto &it : cache) {
      if (remainingComparisons >= it.second && it.first.subset_of(poset)) {
        return true;
      }
    }
    return false;
  }

  inline void insert(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    cache[poset] = remainingComparisons;
  }

  inline void insert_ifLower(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    const auto temp = cache.find(poset);
    if (temp == cache.end()) {
      cache[poset] = remainingComparisons;
    } else if (remainingComparisons > temp->second) {
      temp->second = remainingComparisons;
    }
#ifdef PosetShadowingRemove
    for (auto it = cache.begin(); it != cache.end();) {
      if (remainingComparisons > it->second && it->first.subset_of(poset) && it != temp) {
        it = cache.erase(it);
      } else {
        ++it;
      }
    }
#endif
  }

  inline void insert_ifUpper(const F &poset, const G remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    const auto temp = cache.find(poset);
    if (temp == cache.end()) {
      cache[poset] = remainingComparisons;
    } else if (remainingComparisons < temp->second) {
      temp->second = remainingComparisons;
    }
#ifdef PosetShadowingRemove
    for (auto it = cache.begin(); it != cache.end();) {
      if (remainingComparisons < it->second && poset.subset_of(it->first) && it != temp) {
        it = cache.erase(it);
      } else {
        ++it;
      }
    }
#endif
  }

  inline size_t size() {
    const std::lock_guard<std::mutex> lock(mutex_cache);
    return cache.size();
  }
};