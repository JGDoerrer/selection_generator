#pragma once
#include <mutex>
#include <unordered_map>

#include "poset.h"

// für n = 10, i = 4 und BottomTop:

// 4.842s:
// (cache_l: 1068601, cache_u: 209055, noSol: 7, bruteForce: 39551), cache = (51512 + 3983 = 55495)

struct PosetStruct {
  std::unique_ptr<PosetStruct> branchIsLess, branchIsNotLess;

  PosetStruct() : branchIsLess(nullptr), branchIsNotLess(nullptr){};

  template <size_t maxN>
  bool containsLower(const Poset<maxN> &poset, const uint8_t index) const {
    if (0 == index) {
      return true;
    } else if (poset.comparisonTable[index - 1]) {
      return nullptr != branchIsLess && branchIsLess->containsLower(poset, index - 1);
    } else {
      return (nullptr != branchIsNotLess && branchIsNotLess->containsLower(poset, index - 1)) ||
             (nullptr != branchIsLess && branchIsLess->containsLower(poset, index - 1));
    }
  }

  template <size_t maxN>
  bool containsUpper(const Poset<maxN> &poset, const uint8_t index) const {
    if (0 == index) {
      return true;
    } else if (poset.comparisonTable[index - 1]) {
      return (nullptr != branchIsLess && branchIsLess->containsUpper(poset, index - 1)) ||
             (nullptr != branchIsNotLess && branchIsNotLess->containsUpper(poset, index - 1));
    } else {
      return nullptr != branchIsNotLess && branchIsNotLess->containsUpper(poset, index - 1);
    }
  }

  template <size_t maxN>
  bool entries(std::vector<Poset<maxN>> &entries, size_t &_size, Poset<maxN> temp, const uint8_t index,
               std::unique_ptr<PosetStruct> &rootStruct, std::unique_ptr<PosetStruct> &topLevel, const bool isLower) {
    if (0 == index) {
      if (nullptr != topLevel) {
        std::unique_ptr<PosetStruct> temp1 = move(topLevel);
        topLevel = nullptr;

        if ((isLower) ? rootStruct->containsLower(temp, temp.size() * temp.size())
                      : rootStruct->containsUpper(temp, temp.size() * temp.size())) {
          --_size;
          // return true; // remove temp from rootStruct recursive
        } else {
          topLevel = move(temp1);
          entries.push_back(temp);
        }
      }
    } else {
      if (nullptr != branchIsLess) {
        temp.comparisonTable[index - 1] = true;
        if (branchIsLess->entries(entries, _size, temp, index - 1, rootStruct, branchIsLess, isLower)) {
          branchIsLess = nullptr;
        }
      }
      if (nullptr != branchIsNotLess) {
        temp.comparisonTable[index - 1] = false;
        if (branchIsNotLess->entries(entries, _size, temp, index - 1, rootStruct, branchIsNotLess, isLower)) {
          branchIsNotLess = nullptr;
        }
      }
      // return nullptr == branchIsLess && nullptr == branchIsNotLess;
    }
    return false;
  }
};

class unordered_set2 {
 private:
  std::unique_ptr<PosetStruct> root;
  size_t _size;

 public:
  unordered_set2() : root(std::make_unique<PosetStruct>()) {}

  template <size_t maxN>
  inline void insert(const Poset<maxN> &poset) {
    PosetStruct *level = root.get();
    for (int i = poset.size() * poset.size() - 1; i >= 0; --i) {
      if (poset.comparisonTable[i]) {
        if (nullptr == level->branchIsLess) {
          level->branchIsLess = std::make_unique<PosetStruct>();
        }
        level = level->branchIsLess.get();
      } else {
        if (nullptr == level->branchIsNotLess) {
          level->branchIsNotLess = std::make_unique<PosetStruct>();
        }
        level = level->branchIsNotLess.get();
      }
    }
    ++_size;
  }

  template <size_t maxN>
  inline bool containsLower(const Poset<maxN> &poset) const {
    return root->containsLower(poset, poset.size() * poset.size());
  }

  template <size_t maxN>
  inline bool containsUpper(const Poset<maxN> &poset) const {
    return root->containsUpper(poset, poset.size() * poset.size());
  }

  template <size_t maxN>
  inline void clean(const uint8_t n, const uint8_t i, const bool isLower) {
    std::vector<Poset<maxN>> entries;
    auto temp1 = std::make_unique<PosetStruct>();
    root->entries(entries, _size, Poset<maxN>(n, i), n * n, root, temp1, isLower);
    assert(entries.size() == _size);
  }

  inline size_t size(const uint8_t n) const { return _size; }
};

template <size_t maxN>
class Cache {
 private:
  /**
   * 1. dimension: poset Size, 1 <= n <= globalMaxN
   * 2. dimension: poset nthSmallest, 0 <= i <= globalMaxN / 2
   * 3. dimension: remaining Comparisons, 0 <= c <= globalMaxComparisons
   */
  unordered_set2 cache2[globalMaxN][globalMaxN][globalMaxComparisons];
  std::mutex mutex_cache[globalMaxN][globalMaxN][globalMaxComparisons];

  inline void insert(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][remainingComparisons]);
    cache2[poset.size()][poset.nth()][remainingComparisons].insert(poset);
  }

 public:
  inline void insert_ifLower(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    this->insert(poset, remainingComparisons);
  }

  inline void insert_ifUpper(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    this->insert(poset, remainingComparisons);
  }

  inline bool checkLower(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    for (int c = remainingComparisons; c < globalMaxComparisons; ++c) {
      const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
      if (cache2[poset.size()][poset.nth()][c].containsLower(poset)) {
        return true;
      }
    }
    return false;
  }

  inline bool checkUpper(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    assert(2 * poset.nth() < poset.size());
    for (int c = remainingComparisons; c >= 0; --c) {
      const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
      if (cache2[poset.size()][poset.nth()][c].containsUpper(poset)) {
        return true;
      }
    }
    return false;
  }

  void clean(const bool isLower) {
    for (uint8_t n = 1; n < globalMaxN; ++n) {
      for (uint8_t i = 0; i < globalMaxN; ++i) {
        for (uint8_t c = 0; c < globalMaxComparisons; ++c) {
          const std::lock_guard<std::mutex> lock(mutex_cache[n][i][c]);
          cache2[n][i][c].clean<maxN>(n, i, isLower);
        }
      }
    }
  }

  inline size_t size() {
    size_t sum = 0;
    // size_t max1 = 0;
    for (uint8_t n = 1; n < globalMaxN; ++n) {
      for (uint8_t i = 0; i < globalMaxN; ++i) {
        for (uint8_t c = 0; c < globalMaxComparisons; ++c) {
          const std::lock_guard<std::mutex> lock(mutex_cache[n][i][c]);
          sum += cache2[n][i][c].size(n);
          // max1 = std::max(max1, cache2[n][i][c].size(n));
        }
      }
    }
    // std::cout << max1 << std::endl;
    return sum;
  }
};