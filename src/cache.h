#pragma once
#include <mutex>
#include <unordered_map>

#include "poset.h"

// immer in 4 bit Blöcke
// => 2^6 = 16 Einträge a 6 bit = 512 Bit = 8 Byte

// remove on insert

struct PosetStructRecursive {
  std::unique_ptr<PosetStructRecursive> branchIsLess, branchIsNotLess;

  PosetStructRecursive() : branchIsLess(nullptr), branchIsNotLess(nullptr){};

  template <std::size_t maxN>
  bool contains(const Poset<maxN> &poset, const uint8_t index, const bool is_not_solvable) const {
    if (0 == index) {
      return true;
    } else if (poset.comparisonTable[index - 1]) {
      return nullptr != branchIsLess && branchIsLess->contains(poset, index - 1, is_not_solvable) ||
             (!is_not_solvable && nullptr != branchIsNotLess &&
              branchIsNotLess->contains(poset, index - 1, is_not_solvable));
    } else {
      return (nullptr != branchIsNotLess && branchIsNotLess->contains(poset, index - 1, is_not_solvable)) ||
             (is_not_solvable && nullptr != branchIsLess && branchIsLess->contains(poset, index - 1, is_not_solvable));
    }
  }

  template <std::size_t maxN>
  bool insert(const Poset<maxN> &poset, const uint8_t index, const bool is_not_solvable, std::size_t &size) {
    if (poset.comparisonTable[index - 1]) {
      if (0 == index - 1) {
        if (nullptr == branchIsLess) {
          branchIsLess = std::make_unique<PosetStructRecursive>();
          ++size;
          return true;
        } else {
          return false;
        }
      } else {
        if (nullptr == branchIsLess) {
          branchIsLess = std::make_unique<PosetStructRecursive>();
          ++size;
        }
        return branchIsLess->insert(poset, index - 1, is_not_solvable, size);
      }
    } else {
      if (0 == index - 1) {
        if (nullptr == branchIsNotLess) {
          branchIsNotLess = std::make_unique<PosetStructRecursive>();
          ++size;
          return true;
        } else {
          return false;
        }
      } else {
        if (nullptr == branchIsNotLess) {
          branchIsNotLess = std::make_unique<PosetStructRecursive>();
          ++size;
        }
        return branchIsNotLess->insert(poset, index - 1, is_not_solvable, size);
      }
    }
  }

  template <std::size_t maxN>
  void entries(std::unordered_set<Poset<maxN>> &entries, std::size_t &_size, Poset<maxN> temp, const uint8_t index,
               std::unique_ptr<PosetStructRecursive> &rootStruct, std::unique_ptr<PosetStructRecursive> &topLevel,
               const bool is_not_solvable) {
    if (0 == index) {
      if (nullptr != topLevel) {
        std::unique_ptr<PosetStructRecursive> temp1 = move(topLevel);
        topLevel = nullptr;

        if (!rootStruct->contains(temp, temp.getComparisonTableSize(), is_not_solvable)) {
          entries.insert(temp);
        }
        topLevel = move(temp1);
      }
    } else {
      if (nullptr != branchIsLess) {
        temp.comparisonTable[index - 1] = true;
        branchIsLess->entries(entries, _size, temp, index - 1, rootStruct, branchIsLess, is_not_solvable);
      }
      if (nullptr != branchIsNotLess) {
        temp.comparisonTable[index - 1] = false;
        branchIsNotLess->entries(entries, _size, temp, index - 1, rootStruct, branchIsNotLess, is_not_solvable);
      }
    }
  }

  template <std::size_t maxN>
  bool clean(std::size_t &_size, Poset<maxN> temp, const uint8_t index,
             std::unique_ptr<PosetStructRecursive> &rootStruct, std::unique_ptr<PosetStructRecursive> &topLevel,
             const bool is_not_solvable) {
    if (0 == index) {
      if (nullptr != topLevel) {
        std::unique_ptr<PosetStructRecursive> temp1 = move(topLevel);
        topLevel = nullptr;

        if (rootStruct->contains(temp, temp.getComparisonTableSize(), is_not_solvable)) {
          --_size;
          return true;  // remove temp from rootStruct recursive
        } else {
          topLevel = move(temp1);
        }
      }
    } else {
      if (nullptr != branchIsLess) {
        temp.comparisonTable[index - 1] = true;
        if (branchIsLess->clean(_size, temp, index - 1, rootStruct, branchIsLess, is_not_solvable)) {
          branchIsLess = nullptr;
        }
      }
      if (nullptr != branchIsNotLess) {
        temp.comparisonTable[index - 1] = false;
        if (branchIsNotLess->clean(_size, temp, index - 1, rootStruct, branchIsNotLess, is_not_solvable)) {
          branchIsNotLess = nullptr;
        }
      }
      return nullptr == branchIsLess && nullptr == branchIsNotLess;
    }
    return false;
  }
};

template <std::size_t maxN>
class PosetSet {
 private:
  std::unique_ptr<PosetStructRecursive> root;
  std::size_t _size;

 public:
  PosetSet() : root(std::make_unique<PosetStructRecursive>()) {}

  inline void insert(const Poset<maxN> &poset, const bool is_not_solvable) {
    if constexpr (true) {
      root->insert(poset, poset.getComparisonTableSize() + 1, is_not_solvable, _size);
    } else {
      // works with poset in Large mode
      PosetStructRecursive *level = root.get();
      bool lastInsert = false;
      for (int i = poset.getComparisonTableSize() - 1; i >= 0; --i) {
        lastInsert = false;
        if (poset.comparisonTable[i]) {
          if (nullptr == level->branchIsLess) {
            level->branchIsLess = std::make_unique<PosetStructRecursive>();
            lastInsert = true;
          }
          level = level->branchIsLess.get();
        } else {
          if (nullptr == level->branchIsNotLess) {
            level->branchIsNotLess = std::make_unique<PosetStructRecursive>();
            lastInsert = true;
          }
          level = level->branchIsNotLess.get();
        }
      }
      if (lastInsert) {
        ++_size;
      }
    }
  }

  inline bool contains(const Poset<maxN> &poset, const bool is_not_solvable) const {
    return root->contains(poset, poset.getComparisonTableSize() + 1, is_not_solvable);
  }

  inline void clean(const uint8_t n, const uint8_t i, const bool is_not_solvable) {
    const auto a = std::make_unique<PosetStructRecursive>();
    // root->clean(_size, Poset<maxN>(n, i), n * (n - 1), root, a, is_not_solvable);
  }

  inline std::unordered_set<Poset<maxN>> entries(const uint8_t n, const uint8_t i, const bool is_not_solvable) {
    std::unordered_set<Poset<maxN>> entries;
    const auto a = std::make_unique<PosetStructRecursive>();
    // root->entries(entries, _size, Poset<maxN>(n, i), n * n, root, a, is_not_solvable);
    return entries;
  }

  inline std::size_t size(const uint8_t n) const { return _size; }
};

template <std::size_t maxN, std::size_t maxC>
class PosetCache {
 private:
  /**
   * 1. dimension: poset Size, 1 <= n <= maxN
   * 2. dimension: poset nthSmallest, 0 <= i <= maxN / 2
   * 3. dimension: remaining Comparisons, 0 <= c <= maxC
   */
  PosetSet<maxN> cache[maxN][maxN][maxC];
  std::mutex mutex_cache[maxN][maxN][maxC];

 public:
  inline void insert(const Poset<maxN> &poset, const uint8_t remainingComparisons, const bool is_not_solvable) {
    assert(2 * poset.nth() < poset.size());
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][remainingComparisons]);
    cache[poset.size()][poset.nth()][remainingComparisons].insert(poset, is_not_solvable);
  }

  inline bool check_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    for (int c = remainingComparisons; c < maxC; ++c) {
      const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
      if (cache[poset.size()][poset.nth()][c].contains(poset, true)) {
        return true;
      }
    }
    return false;
  }

  inline bool check_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    for (int c = remainingComparisons; c >= 0; --c) {
      const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
      if (cache[poset.size()][poset.nth()][c].contains(poset, false)) {
        return true;
      }
    }
    return false;
  }

  inline void clean(const bool is_not_solvable) {
    for (uint8_t n = 1; n < maxN; ++n) {
      for (uint8_t i = 0; i < maxN; ++i) {
        for (uint8_t c = 0; c < maxC; ++c) {
          const std::lock_guard<std::mutex> lock(mutex_cache[n][i][c]);
          cache[n][i][c].clean<maxN>(n, i, is_not_solvable);
        }
      }
    }
  }

  inline std::size_t size() {
    std::size_t sum = 0;
    for (uint8_t n = 1; n < maxN; ++n) {
      for (uint8_t i = 0; i < maxN; ++i) {
        for (uint8_t c = 0; c < maxC; ++c) {
          const std::lock_guard<std::mutex> lock(mutex_cache[n][i][c]);
          sum += cache[n][i][c].size(n);
        }
      }
    }
    return sum;
  }
};

template <std::size_t maxN, std::size_t maxC>
class CacheTree {
 private:
  /// TODO: doku aktualisieren
  /// @param cache_not_solvable enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
  ///                         werden kann; z.B. wenn cache_not_solvable[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
  ///                         um Poset zu lösen
  /// @param cache_solvable enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
  ///                         cache_solvable[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
  PosetCache<maxN, maxC> cache_not_solvable, cache_solvable;

 public:
  inline bool check_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    return cache_not_solvable.check_not_solvable(poset, remainingComparisons);
  }

  inline bool check_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    return cache_solvable.check_solvable(poset, remainingComparisons);
  }

  inline void insert_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    cache_not_solvable.insert(poset, remainingComparisons, true);
  }

  inline void insert_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    cache_solvable.insert(poset, remainingComparisons, false);
  }

  inline void clean() {
    cache_not_solvable.clean(true);
    cache_solvable.clean(false);
  }

  inline std::size_t size() { return cache_not_solvable.size() + cache_solvable.size(); }
};