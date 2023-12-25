#pragma once
#include <mutex>
#include <unordered_map>

#include "poset.h"

// für n = 10, i = 4 und BottomTop:

// 4.842s:
// (cache_l: 1068601, cache_u: 209055, noSol: 7, bruteForce: 39551), cache = (51512 + 3983 = 55495)

struct PosetStruct {
  std::shared_ptr<PosetStruct> branchIsLess, branchIsNotLess;

  PosetStruct() : branchIsLess(nullptr), branchIsNotLess(nullptr){};

  // template <size_t maxN>
  // bool contains(const Poset<maxN> &poset, const uint8_t index) const {
  //   if (0 == index) {
  //     return true;
  //   } else if (poset.comparisonTable[index - 1]) {
  //     return (nullptr != branchIsLess && branchIsLess->containsLower(poset, index - 1));
  //   } else {
  //     return (nullptr != branchIsNotLess && branchIsNotLess->containsLower(poset, index - 1));
  //   }
  // }

  template <size_t maxN>
  bool containsLower(const Poset<maxN> &poset, const uint8_t index) const {
    if (0 == index) {
      return true;
    } else if (poset.comparisonTable[index - 1]) {
      return (nullptr != branchIsLess && branchIsLess->containsLower(poset, index - 1));
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
      return (nullptr != branchIsNotLess && branchIsNotLess->containsUpper(poset, index - 1));
    }
  }

  size_t size(const uint8_t index) {
    if (0 == index) {
      return 1;
    }
    size_t sum = 0;
    if (nullptr != branchIsLess) {
      sum += branchIsLess->size(index - 1);
    }
    if (nullptr != branchIsNotLess) {
      sum += branchIsNotLess->size(index - 1);
    }
    return sum;
  }

  // template <size_t maxN>
  // void entries(std::vector<Poset<maxN>> &entries, Poset<maxN> temp, const uint8_t index) {
  //   if (0 == index) {
  //     entries.push_back(temp);
  //   }
  //   size_t sum = 0;
  //   if (nullptr != branchIsLess) {
  //     temp.comparisonTable[index - 1] = true;
  //     branchIsLess->entries(entries, temp, index - 1);
  //   }
  //   if (nullptr != branchIsNotLess) {
  //     temp.comparisonTable[index - 1] = false;
  //     branchIsNotLess->entries(entries, temp, index - 1);
  //   }
  // }
};

class unordered_set2 {
 private:
  std::shared_ptr<PosetStruct> root;

 public:
  unordered_set2() : root(std::make_unique<PosetStruct>()) {}

  template <size_t maxN>
  inline void insert(const Poset<maxN> &poset) {
    std::shared_ptr<PosetStruct> level = root;
    for (int i = poset.size() * poset.size() - 1; i >= 0; --i)
      if (poset.comparisonTable[i]) {
        if (nullptr == level->branchIsLess) {
          level->branchIsLess = std::make_unique<PosetStruct>();
        }
        level = level->branchIsLess;
      } else {
        if (nullptr == level->branchIsNotLess) {
          level->branchIsNotLess = std::make_unique<PosetStruct>();
        }
        level = level->branchIsNotLess;
      }
  }

  template <size_t maxN>
  inline bool containsLower(const Poset<maxN> &poset) const {
    return root->containsLower(poset, poset.size() * poset.size());
  }

  template <size_t maxN>
  inline bool containsUpper(const Poset<maxN> &poset) const {
    return root->containsUpper(poset, poset.size() * poset.size());
  }

  inline size_t size(const uint8_t n) const { return root->size(n * n); }

  // template <size_t maxN>
  // inline void clean(const uint8_t n, const uint8_t i) {
  // TODO
  // std::shared_ptr<PosetStruct> new_root;
  // std::vector<Poset<maxN>> entries;
  // root->entries(entries, Poset<maxN>(n, i), n * n);
  // assert(entries.size() == root->size(n * n));
  // for (auto entry : entries) {
  // }
  // }
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

  void clean() {
    //   for (uint8_t n = 1; n < globalMaxN; ++n) {
    //     for (uint8_t i = 0; i < globalMaxN; ++i) {
    //       for (uint8_t c = 0; c < globalMaxComparisons; ++c) {
    //         const std::lock_guard<std::mutex> lock(mutex_cache[n][i][c]);
    //         cache2[n][i][c].clean<maxN>(n, i);
    //       }
    //     }
    //   }
  }
};