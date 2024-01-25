#pragma once
#include <mutex>
#include <unordered_map>

#include "poset.h"

template <bool is_solvable>
struct CacheNode {
  std::unique_ptr<CacheNode<is_solvable>> branchIsLess, branchIsNotLess;

  CacheNode() : branchIsLess(nullptr), branchIsNotLess(nullptr){};

  bool contains(const Poset &poset, const uint8_t index) const {
    if (0 == index) {
      return true;
    } else if (poset.comparisonTable[index - 1]) {
      return nullptr != branchIsLess && branchIsLess->contains(poset, index - 1) ||
             (is_solvable && nullptr != branchIsNotLess && branchIsNotLess->contains(poset, index - 1));
    } else {
      return (nullptr != branchIsNotLess && branchIsNotLess->contains(poset, index - 1)) ||
             (!is_solvable && nullptr != branchIsLess && branchIsLess->contains(poset, index - 1));
    }
  }

  void entries(std::unordered_set<Poset> &entries, Poset temp, const uint8_t index,
               std::unique_ptr<CacheNode<is_solvable>> &rootStruct,
               std::unique_ptr<CacheNode<is_solvable>> &topLevel) {
    if (0 == index) {
      if (nullptr != topLevel) {
        std::unique_ptr<CacheNode<is_solvable>> temp1 = move(topLevel);
        topLevel = nullptr;

        if (!rootStruct->contains(temp, temp.getComparisonTableSize())) {
          entries.insert(temp);
        }
        topLevel = move(temp1);
      }
    } else {
      if (nullptr != branchIsLess) {
        temp.comparisonTable[index - 1] = true;
        branchIsLess->entries(entries, temp, index - 1, rootStruct, branchIsLess);
      }
      if (nullptr != branchIsNotLess) {
        temp.comparisonTable[index - 1] = false;
        branchIsNotLess->entries(entries, temp, index - 1, rootStruct, branchIsNotLess);
      }
    }
  }
};

template <bool is_solvable>
class CacheTreeFixed {
 private:
  std::unique_ptr<CacheNode<is_solvable>> root;
  std::size_t _size;

 public:
  CacheTreeFixed() : root(std::make_unique<CacheNode<is_solvable>>()) {}

  inline void insert(const Poset &poset) {
    bool lastInsert = false;
    if constexpr (false) {
      lastInsert = root->insert(poset, poset.getComparisonTableSize());
    } else {
      CacheNode<is_solvable> *level = root.get();
      for (int i = poset.getComparisonTableSize() - 1; i >= 0; --i) {
        lastInsert = false;
        if (poset.comparisonTable[i]) {
          if (nullptr == level->branchIsLess) {
            level->branchIsLess = std::make_unique<CacheNode<is_solvable>>();
            lastInsert = true;
          }
          level = level->branchIsLess.get();
        } else {
          if (nullptr == level->branchIsNotLess) {
            level->branchIsNotLess = std::make_unique<CacheNode<is_solvable>>();
            lastInsert = true;
          }
          level = level->branchIsNotLess.get();
        }
      }
    }
    if (lastInsert) {
      ++_size;
    }
  }

  inline bool contains(const Poset &poset) const { return root->contains(poset, poset.getComparisonTableSize()); }

  inline std::unordered_set<Poset> entries(const uint8_t n, const uint8_t i) {
    std::unordered_set<Poset> entries;
    std::unique_ptr<CacheNode<is_solvable>> a = std::make_unique<CacheNode<is_solvable>>();
    root->entries(entries, Poset(n, i), n * n, root, a);
    return entries;
  }

  inline std::size_t size(const uint8_t n) const { return _size; }

  inline void reset() {
    _size = 0;
    root.reset();
  }
};

template <bool is_solvable>
class CacheTreeSingle {
 private:
  /**
   * 1. dimension: poset Size, 1 <= n <= MAX_N
   * 2. dimension: poset nthSmallest, 0 <= i <= MAX_N / 2
   * 3. dimension: remaining Comparisons, 0 <= c <= MAX_COMPARISONS
   */
  CacheTreeFixed<is_solvable> cache[MAX_N][MAX_N][MAX_COMPARISONS];
  std::shared_mutex mutex_cache[MAX_N][MAX_N][MAX_COMPARISONS];

 public:
  inline void insert(const Poset &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    const std::lock_guard<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()][remainingComparisons]);
    cache[poset.size()][poset.nth()][remainingComparisons].insert(poset);
  }

  inline bool check(const Poset &poset, const uint8_t remainingComparisons) {
    if (is_solvable) {
      for (int c = remainingComparisons; c >= 0; --c) {
        const std::shared_lock<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
        if (cache[poset.size()][poset.nth()][c].contains(poset)) {
          return true;
        }
      }
      return false;
    } else {
      for (int c = remainingComparisons; c < MAX_COMPARISONS; ++c) {
        const std::shared_lock<std::shared_mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
        if (cache[poset.size()][poset.nth()][c].contains(poset)) {
          return true;
        }
      }
      return false;
    }
  }

  inline std::size_t size() {
    std::size_t sum = 0;
    for (uint8_t n = 1; n < MAX_N; ++n) {
      for (uint8_t i = 0; i < MAX_N; ++i) {
        for (uint8_t c = 0; c < MAX_COMPARISONS; ++c) {
          const std::lock_guard<std::shared_mutex> lock(mutex_cache[n][i][c]);
          sum += cache[n][i][c].size(n);
        }
      }
    }
    return sum;
  }
};

class CacheTreeDual {
 private:
  /// TODO: doku aktualisieren
  /// @param cache_not_solvable enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
  ///                         werden kann; z.B. wenn cache_not_solvable[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
  ///                         um Poset zu lösen
  /// @param cache_solvable enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
  ///                         cache_solvable[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
  CacheTreeSingle<true> cache_solvable;
  CacheTreeSingle<false> cache_not_solvable;

 public:
  inline bool check_not_solvable(const Poset &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    return cache_not_solvable.check(poset, remainingComparisons);
  }

  inline bool check_solvable(const Poset &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    return cache_solvable.check(poset, remainingComparisons);
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