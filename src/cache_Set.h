#pragma once
#include <mutex>
#include <unordered_map>

template <std::size_t maxN, std::size_t maxC>
class PosetCacheSet {
 private:
  std::unordered_map<Poset<maxN>, uint8_t> cache[globalMaxN][globalMaxN];
  std::shared_mutex mutex_cache[globalMaxN][globalMaxN];

 public:
  inline void insert_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp == cache[poset.size()][poset.nth()].end()) {
      cache[poset.size()][poset.nth()][poset] = remainingComparisons;
    } else if (remainingComparisons > temp->second) {
      temp->second = remainingComparisons;
    }
  }

  inline void insert_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    const std::lock_guard<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp == cache[poset.size()][poset.nth()].end()) {
      cache[poset.size()][poset.nth()][poset] = remainingComparisons;
    } else if (remainingComparisons < temp->second) {
      temp->second = remainingComparisons;
    }
  }

  inline bool check_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons, bool special = false) {
    const std::shared_lock<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp != cache[poset.size()][poset.nth()].end() && remainingComparisons <= temp->second) {
      return true;
    }
    return false;
  }

  inline bool check_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons, bool special = false) {
    const std::shared_lock<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()]);
    const auto temp = cache[poset.size()][poset.nth()].find(poset);
    if (temp != cache[poset.size()][poset.nth()].end() && remainingComparisons >= temp->second) {
      return true;
    }
    return false;
  }

  inline void clean(const bool is_not_solvable) {}

  inline std::size_t size() {
    size_t sum = 0;
    for (uint8_t n = 0; n < globalMaxN; ++n) {
      for (uint8_t i = 0; i < globalMaxN; ++i) {
        const std::shared_lock<std::shared_mutex> lock(mutex_cache[n][i]);
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
    return cache_not_solvable.size() + cache_solvable.size();
  }
};