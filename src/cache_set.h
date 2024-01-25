#pragma once
#include <mutex>
#include <unordered_map>

#include "poset.h"

template <bool is_solvable>
class CacheSetSingle {
 private:
  std::unordered_map<Poset, uint8_t> cache[MAX_N][MAX_N];
  std::shared_mutex mutex[MAX_N][MAX_N];

 public:
  inline void insert(const Poset &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::shared_mutex> lock(mutex[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp == cache[poset.size()][poset.nth()].end()) {
      cache[poset.size()][poset.nth()][poset] = remainingComparisons;
    } else if ((is_solvable && remainingComparisons < temp->second) ||
               (!is_solvable && remainingComparisons > temp->second)) {
      temp->second = remainingComparisons;
    }
  }

  inline bool check(const Poset &poset, const uint8_t remainingComparisons, bool special = false) {
    const std::shared_lock<std::shared_mutex> lock(mutex[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    return temp != cache[poset.size()][poset.nth()].end() && ((is_solvable && remainingComparisons >= temp->second) ||
                                                              (!is_solvable && remainingComparisons <= temp->second));
  }

  inline std::size_t size() {
    size_t sum = 0;
    for (uint8_t n = 0; n < MAX_N; ++n) {
      for (uint8_t i = 0; i < MAX_N; ++i) {
        const std::shared_lock<std::shared_mutex> lock(mutex[n][i]);
        sum += cache[n][i].size();
      }
    }
    return sum;
  }
};

class CacheSetDual {
 private:
  /// TODO: doku aktualisieren
  /// @param cache_not_solvable enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
  ///                         werden kann; z.B. wenn cache_not_solvable[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
  ///                         um Poset zu lösen
  /// @param cache_solvable enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
  ///                         cache_solvable[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
  CacheSetSingle<true> cache_solvable;
  CacheSetSingle<false> cache_not_solvable;

 public:
  inline bool check_not_solvable(const Poset &poset, const uint8_t remainingComparisons, bool special = false) {
    assert(2 * poset.nth() < poset.size());
    return cache_not_solvable.check(poset, remainingComparisons, special);
  }

  inline bool check_solvable(const Poset &poset, const uint8_t remainingComparisons, bool special = false) {
    assert(2 * poset.nth() < poset.size());
    return cache_solvable.check(poset, remainingComparisons, special);
  }

  inline void insert_not_solvable(const Poset &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    cache_not_solvable.insert(poset, remainingComparisons);
  }

  inline void insert_solvable(const Poset &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    cache_solvable.insert(poset, remainingComparisons);
  }

  inline std::size_t size() { return cache_not_solvable.size() + cache_solvable.size(); }
};