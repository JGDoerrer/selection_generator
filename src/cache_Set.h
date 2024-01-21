#pragma once
#include <mutex>
#include <unordered_map>

// #define PosetShadowing
// #define PosetShadowingRemove

// für n = 10, i = 4 und BottomTop:

// 6.742s (ohne):
// (cache_l: 2016256, cache_u: 288692, noSol: 12, bruteForce: 72555), cache = (59544 + 4809 = 64353)

// 10.882s (mit PosetShadowing):
// (cache_l: 1068601, cache_u: 209055, noSol: 7, bruteForce: 39551), cache = (35546 + 3980 = 39526)

// 13.232s (mit Beidem):
// (cache_l: 1068601, cache_u: 209055, noSol: 7, bruteForce: 39551), cache = (34344 + 3980 = 38324)

template <std::size_t maxN, std::size_t maxC>
class PosetCacheSet {
 private:
  std::unordered_map<Poset<maxN>, uint8_t> cache[globalMaxN][globalMaxN];
  std::mutex mutex_cache[globalMaxN][globalMaxN];

 public:
  inline void insert_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
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

  inline void insert_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
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

  inline bool check_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons, bool special = false) {
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp != cache[poset.size()][poset.nth()].end() && remainingComparisons <= temp->second) {
      return true;
    }
#ifdef PosetShadowing
    for (const auto &it : cache[poset.size()][poset.nth()]) {
      if (special) {
        if (remainingComparisons <= it.second && poset.subsetBruteForce(it.first)) {
          return true;
        }
      } else {
        if (remainingComparisons <= it.second && poset.subset_of(it.first)) {
          return true;
        }
      }
    }
#endif
    return false;
  }

  inline bool check_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons, bool special = false) {
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp != cache[poset.size()][poset.nth()].end() && remainingComparisons >= temp->second) {
      return true;
    }
#ifdef PosetShadowing
    for (const auto &it : cache[poset.size()][poset.nth()]) {
      if (special) {
        if (remainingComparisons >= it.second && it.first.subsetBruteForce(poset)) {
          return true;
        }
      } else {
        if (remainingComparisons >= it.second && it.first.subset_of(poset)) {
          return true;
        }
      }
    }
#endif
    return false;
  }

  inline void clean(const bool is_not_solvable) {}

  inline std::size_t size() {
    size_t sum = 0;
    for (uint8_t n = 0; n < globalMaxN; ++n) {
      for (uint8_t i = 0; i < globalMaxN; ++i) {
        const std::lock_guard<std::mutex> lock(mutex_cache[n][i]);
        sum += cache[n][i].size();
      }
    }
    return sum;
  }
};

template <std::size_t maxN, std::size_t maxC>
class CacheSet {
 private:
  /// TODO: doku aktualisieren
  /// @param cache_not_solvable enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
  ///                         werden kann; z.B. wenn cache_not_solvable[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
  ///                         um Poset zu lösen
  /// @param cache_solvable enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
  ///                         cache_solvable[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
  PosetCacheSet<maxN, maxC> cache_not_solvable, cache_solvable;

 public:
  inline bool check_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons, bool special = false) {
    assert(2 * poset.nth() < poset.size());
    return cache_not_solvable.check_not_solvable(poset, remainingComparisons, special);
  }

  inline bool check_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons, bool special = false) {
    assert(2 * poset.nth() < poset.size());
    return cache_solvable.check_solvable(poset, remainingComparisons, special);
  }

  inline void insert_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    cache_not_solvable.insert_not_solvable(poset, remainingComparisons);
  }

  inline void insert_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    cache_solvable.insert_solvable(poset, remainingComparisons);
  }

  inline void clean() {
    cache_not_solvable.clean(true);
    cache_solvable.clean(false);
  }

  inline std::size_t size() {
    // int n1 = 10;
    // int k1 = 4;

    // std::cout << std::endl;
    // std::vector<std::pair<float, std::pair<int, int>>> vec1;
    // for (int i = 0; i < n1; ++i) {
    //   for (int j = 0; j < n1; ++j) {
    //     int counter = 0;
    //     int counter2 = 0;
    //     for (auto [item, num] : cache_not_solvable.cache[n1][k1]) {
    //       if (item.is_less(i, j)) {
    //         ++counter;
    //       } else {
    //         ++counter2;
    //       }
    //     }
    //     for (auto [item, num] : cache_solvable.cache[n1][k1]) {
    //       if (item.is_less(i, j)) {
    //         ++counter;
    //       } else {
    //         ++counter2;
    //       }
    //     }
    //     vec1.push_back(
    //         {std::abs(float(std::max(counter, counter2)) / float(counter + counter2) - 0.5f), std::make_pair(i, j)});
    //     std::cout << 2 * (float(std::max(counter, counter2)) / float(counter + counter2) - 0.5f) << "\t";
    //   }
    //   std::cout << std::endl;
    // }
    // std::sort(vec1.begin(), vec1.end());
    // for (int i = 0; i < 17 && i < vec1.size(); ++i) {
    //   std::cout << "std::make_pair(" << vec1[i].second.first << ", " << vec1[i].second.second << "), ";
    //   // std::cout << i << ": " << vec1[i].second.first << ", " << vec1[i].second.second << " -> " << vec1[i].first
    //   <<
    //   // std::endl;
    // }

    return cache_not_solvable.size() + cache_solvable.size();
  }
};