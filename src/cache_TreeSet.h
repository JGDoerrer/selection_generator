#pragma once
#include <mutex>
#include <unordered_map>

#include "poset.h"

// immer in 4 bit Blöcke
// => 2^6 = 16 Einträge a 6 bit = 512 Bit = 8 Byte

// remove on insert

// constexpr int maxItemsPerLevel = 1;
template <std::size_t maxN>
struct PosetStructRecursive1 {
  virtual bool insert(const Poset<maxN> &poset, const uint8_t index, const std::vector<uint8_t> &order) = 0;

  virtual bool contains(const Poset<maxN> &poset, const uint8_t index, const bool is_not_solvable,
                        const std::vector<uint8_t> &order) const = 0;

  virtual void entries(Poset<maxN> temp, const uint8_t index, std::unordered_set<Poset<maxN>> &entries,
                       std::unique_ptr<PosetStructRecursive1<maxN>> &rootStruct,
                       std::unique_ptr<PosetStructRecursive1<maxN>> &topLevel, const bool is_not_solvable,
                       const std::vector<uint8_t> &order) = 0;
};

template <std::size_t maxN>
struct PosetStructRecursiveLeave : PosetStructRecursive1<maxN> {
  std::unordered_set<Poset<maxN>> items;

  PosetStructRecursiveLeave() : items(){};

  virtual bool insert(const Poset<maxN> &poset, const uint8_t index, const std::vector<uint8_t> &order) override {
    return items.insert(poset).second;
  }

  virtual bool contains(const Poset<maxN> &poset, const uint8_t index, const bool is_not_solvable,
                        const std::vector<uint8_t> &order) const override {
    if (items.contains(poset)) {
      return true;
    }
    for (const auto &it : items) {
      if ((is_not_solvable && poset.subset_of(it)) || (!is_not_solvable && it.subset_of(poset))) {
        return true;
      }
    }
    return false;
  }

  virtual void entries(Poset<maxN> temp, const uint8_t index, std::unordered_set<Poset<maxN>> &entries,
                       std::unique_ptr<PosetStructRecursive1<maxN>> &rootStruct,
                       std::unique_ptr<PosetStructRecursive1<maxN>> &topLevel, const bool is_not_solvable,
                       const std::vector<uint8_t> &order) override {
    entries.merge(std::unordered_set<Poset<maxN>>(items));
  }
};

template <std::size_t maxN>
struct PosetStructRecursiveNode : public PosetStructRecursive1<maxN> {
  std::unique_ptr<PosetStructRecursive1<maxN>> branchIsLess, branchIsNotLess;

  PosetStructRecursiveNode() : branchIsLess(nullptr), branchIsNotLess(nullptr){};

  virtual bool insert(const Poset<maxN> &poset, const uint8_t index, const std::vector<uint8_t> &order) override {
    if (poset.comparisonTable[order[index]]) {
      if (nullptr == branchIsLess) {
        if (0 == index) {
          branchIsLess = std::make_unique<PosetStructRecursiveLeave<maxN>>();
        } else {
          branchIsLess = std::make_unique<PosetStructRecursiveNode<maxN>>();
        }
      }
      return branchIsLess->insert(poset, index - 1, order);
    } else {
      if (nullptr == branchIsNotLess) {
        if (0 == index) {
          branchIsNotLess = std::make_unique<PosetStructRecursiveLeave<maxN>>();
        } else {
          branchIsNotLess = std::make_unique<PosetStructRecursiveNode<maxN>>();
        }
      }
      return branchIsNotLess->insert(poset, index - 1, order);
    }
  }

  virtual bool contains(const Poset<maxN> &poset, const uint8_t index, const bool is_not_solvable,
                        const std::vector<uint8_t> &order) const override {
    if (poset.comparisonTable[order[index]]) {
      return nullptr != branchIsLess && branchIsLess->contains(poset, index - 1, is_not_solvable, order) ||
             (!is_not_solvable && nullptr != branchIsNotLess &&
              branchIsNotLess->contains(poset, index - 1, is_not_solvable, order));
    } else {
      return (nullptr != branchIsNotLess && branchIsNotLess->contains(poset, index - 1, is_not_solvable, order)) ||
             (is_not_solvable && nullptr != branchIsLess &&
              branchIsLess->contains(poset, index - 1, is_not_solvable, order));
    }
  }

  virtual void entries(Poset<maxN> temp, const uint8_t index, std::unordered_set<Poset<maxN>> &entries,
                       std::unique_ptr<PosetStructRecursive1<maxN>> &rootStruct,
                       std::unique_ptr<PosetStructRecursive1<maxN>> &topLevel, const bool is_not_solvable,
                       const std::vector<uint8_t> &order) override {
    if (0 == index) {
      if (nullptr != topLevel) {
        std::unique_ptr<PosetStructRecursive1<maxN>> temp1 = move(topLevel);
        topLevel = nullptr;

        if (!rootStruct->contains(temp, order.size() - 1, is_not_solvable, order)) {
          entries.insert(temp);
        }
        topLevel = move(temp1);
      }
    } else {
      if (nullptr != branchIsLess) {
        temp.comparisonTable[index - 1] = true;
        branchIsLess->entries(temp, index - 1, entries, rootStruct, branchIsLess, is_not_solvable, order);
      }
      if (nullptr != branchIsNotLess) {
        temp.comparisonTable[index - 1] = false;
        branchIsNotLess->entries(temp, index - 1, entries, rootStruct, branchIsNotLess, is_not_solvable, order);
      }
    }
  }
};

template <std::size_t maxN>
class PosetSet2 {
 private:
  std::unique_ptr<PosetStructRecursive1<maxN>> root;
  std::size_t _size;

 public:
  PosetSet2() : root(nullptr), _size(0) {}

  inline void insert(const Poset<maxN> &poset, const std::vector<uint8_t> &order) {
    if (nullptr == root) {
      if (0 == order.size()) {
        root = std::make_unique<PosetStructRecursiveLeave<maxN>>();
      } else {
        root = std::make_unique<PosetStructRecursiveNode<maxN>>();
      }
    }
    if (root->insert(poset, order.size() - 1, order)) {
      ++_size;
    }
  }

  inline bool contains(const Poset<maxN> &poset, const bool is_not_solvable, const std::vector<uint8_t> &order) const {
    return nullptr != root && root->contains(poset, order.size() - 1, is_not_solvable, order);
  }

  inline std::unordered_set<Poset<maxN>> entries(const uint8_t n, const uint8_t i, const bool is_not_solvable,
                                                 const std::vector<uint8_t> &order) {
    std::unordered_set<Poset<maxN>> entries;
    std::unique_ptr<PosetStructRecursive1<maxN>> a = nullptr;
    root->entries(Poset<maxN>(n, i), order.size() - 1, entries, root, a, is_not_solvable, order);
    return entries;
  }

  inline std::size_t size(const uint8_t n) const { return _size; }
};

template <std::size_t maxN, std::size_t maxC>
class PosetCache2 {
 private:
  /**
   * 1. dimension: poset Size, 1 <= n <= maxN
   * 2. dimension: poset nthSmallest, 0 <= i <= maxN / 2
   * 3. dimension: remaining Comparisons, 0 <= c <= maxC
   */
  std::mutex mutex_cache[maxN][maxN][maxC];
  PosetSet2<maxN> cache[maxN][maxN][maxC];
  std::vector<uint8_t> order[maxN][maxN];
  bool order_set[maxN][maxN];
  std::unordered_set<Poset<maxN>> temp1[maxN][maxN][maxC];

 public:
  PosetCache2() {
    for (int n = 0; n < maxN; ++n) {
      for (int k = 0; k < maxN; ++k) {
        order_set[n][k] = false;
      }
    }
  }

  inline void insert(const Poset<maxN> &poset, const uint8_t remainingComparisons, const bool is_not_solvable) {
    assert(2 * poset.nth() < poset.size());
    const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][remainingComparisons]);
    if (!order_set[poset.size()][poset.nth()]) {  // noch nicht theardsafe?
      std::vector<std::pair<float, std::pair<int, int>>> vec1;
      if (1 < poset.size()) {
        for (int i = 0; i < poset.size() - 1; ++i) {
          for (int j = 0; j < poset.size() - 1; ++j) {
            if (i != j) {
              std::size_t ones = 0, zeros = 0;
              // auto tem = cache[poset.size() - 1][poset.nth()][remainingComparisons].entries(
              //     poset.size(), poset.nth(), is_not_solvable, order[poset.size() - 1][poset.nth()]);
              for (const auto &item : temp1[poset.size() - 1][poset.nth()][remainingComparisons]) {
                if (item.is_less(i, j)) {
                  ++ones;
                } else {
                  ++zeros;
                }
              }
              const float value = (0 == ones + zeros) ? 1 / 0.f : std::abs((ones - zeros) / float(ones + zeros));
              vec1.push_back({value, std::make_pair(i + 1, j + 1)});
            }
          }
        }
        std::sort(vec1.begin(), vec1.end());
        const std::size_t maxTreeLevels = poset.size() + 3;  // should be > 0
        for (int i = 0; i < vec1.size() && i < maxTreeLevels; ++i) {
          order[poset.size()][poset.nth()].push_back(poset.toInternalPos(vec1[i].second.first, vec1[i].second.second));
        }
      }
      order_set[poset.size()][poset.nth()] = true;
    }
    cache[poset.size()][poset.nth()][remainingComparisons].insert(poset, order[poset.size()][poset.nth()]);
    temp1[poset.size()][poset.nth()][remainingComparisons].insert(poset);
  }

  inline bool check_not_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    for (int c = remainingComparisons; c < maxC; ++c) {
      const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
      if (cache[poset.size()][poset.nth()][c].contains(poset, true, order[poset.size()][poset.nth()])) {
        return true;
      }
    }
    return false;
  }

  inline bool check_solvable(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
    for (int c = remainingComparisons; c >= 0; --c) {
      const std::lock_guard<std::mutex> lock(mutex_cache[poset.size()][poset.nth()][c]);
      if (cache[poset.size()][poset.nth()][c].contains(poset, false, order[poset.size()][poset.nth()])) {
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
class CacheTreeMemoryEffient {
 private:
  /// TODO: doku aktualisieren
  /// @param cache_not_solvable enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
  ///                         werden kann; z.B. wenn cache_not_solvable[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
  ///                         um Poset zu lösen
  /// @param cache_solvable enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
  ///                         cache_solvable[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
  PosetCache2<maxN, maxC> cache_not_solvable, cache_solvable;

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
    // cache_not_solvable.clean(true);
    // cache_solvable.clean(false);
  }

  inline std::size_t size() { return cache_not_solvable.size() + cache_solvable.size(); }
};