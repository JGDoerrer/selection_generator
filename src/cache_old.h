#pragma once
#include <mutex>
#include <unordered_map>

#define PosetShadowing
#define PosetShadowingRemove

// für n = 10, i = 4 und BottomTop:

// 6.742s (ohne):
// (cache_l: 2016256, cache_u: 288692, noSol: 12, bruteForce: 72555), cache = (59544 + 4809 = 64353)

// 10.882s (mit PosetShadowing):
// (cache_l: 1068601, cache_u: 209055, noSol: 7, bruteForce: 39551), cache = (35546 + 3980 = 39526)

// 13.232s (mit Beidem):
// (cache_l: 1068601, cache_u: 209055, noSol: 7, bruteForce: 39551), cache = (34344 + 3980 = 38324)

template <size_t maxN>
class Cache {
 private:
  std::unordered_map<Poset<maxN>, uint8_t> cache[globalMaxN][globalMaxN];
  std::mutex mutex_cache[globalMaxN][globalMaxN];

 public:
  inline bool checkLower(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp != cache[poset.size()][poset.nth()].end() && remainingComparisons <= temp->second) {
      return true;
    }
#ifdef PosetShadowing
    for (const auto &it : cache[poset.size()][poset.nth()]) {
      if (remainingComparisons <= it.second && poset.subset_of(it.first)) {
        return true;
      }
    }
#endif
    return false;
  }

  inline bool checkUpper(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp != cache[poset.size()][poset.nth()].end() && remainingComparisons >= temp->second) {
      return true;
    }
#ifdef PosetShadowing
    for (const auto &it : cache[poset.size()][poset.nth()]) {
      if (remainingComparisons >= it.second && it.first.subset_of(poset)) {
        return true;
      }
    }
#endif
    return false;
  }

  inline void insert_ifLower(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp == cache[poset.size()][poset.nth()].end()) {
      cache[poset.size()][poset.nth()][poset] = remainingComparisons;
    } else if (remainingComparisons > temp->second) {
      temp->second = remainingComparisons;
    }
#ifdef PosetShadowingRemove
    for (auto it = cache[poset.size()][poset.nth()].begin(); it != cache[poset.size()][poset.nth()].end();) {
      if (remainingComparisons > it->second && it->first.subset_of(poset) && it != temp) {
        it = cache[poset.size()][poset.nth()].erase(it);
      } else {
        ++it;
      }
    }
#endif
  }

  inline void insert_ifUpper(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp == cache[poset.size()][poset.nth()].end()) {
      cache[poset.size()][poset.nth()][poset] = remainingComparisons;
    } else if (remainingComparisons < temp->second) {
      temp->second = remainingComparisons;
    }
#ifdef PosetShadowingRemove
    for (auto it = cache[poset.size()][poset.nth()].begin(); it != cache[poset.size()][poset.nth()].end();) {
      if (remainingComparisons < it->second && poset.subset_of(it->first) && it != temp) {
        it = cache[poset.size()][poset.nth()].erase(it);
      } else {
        ++it;
      }
    }
#endif
  }

  inline size_t size() {
    size_t sum = 0;
    for (uint8_t n = 0; n < globalMaxN; ++n) {
      for (uint8_t i = 0; i < globalMaxN; ++i) {
        const std::lock_guard<std::mutex> lock(mutex_cache[n][i]);
        sum += cache[n][i].size();
      }
    }
    return sum;
  }

  void clean() {}
};