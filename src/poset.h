#pragma once
#include <bits/stdc++.h>

#include "normalizer.h"

template <size_t maxN>
class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;
  bool comparisonTable[maxN * maxN];

  inline void setValue(const uint16_t i, const uint16_t j, const bool value) { comparisonTable[i * n + j] = value; }

  inline bool getValue(const uint16_t i, const uint16_t j) const { return comparisonTable[i * n + j]; }

  void addComparisonTransitivRecursive(const uint16_t i, const uint16_t j) {
    if (false == getValue(i, j)) {
      setValue(i, j, true);

      for (uint16_t k = 0; k < n; ++k) {
        if (getValue(j, k) && !getValue(i, k)) {
          addComparisonTransitivRecursive(i, k);
        } else if (getValue(k, i) && !getValue(k, j)) {
          addComparisonTransitivRecursive(k, j);
        }
      }
    }
  }

  inline void addComparisonTransitivIterative(const uint16_t i, const uint16_t j) {
    std::queue<std::pair<uint16_t, uint16_t>> queue;
    queue.push({i, j});
    while (0 != queue.size()) {
      const auto &[i1, j1] = queue.front();
      queue.pop();

      if (false == getValue(i1, j1)) {
        setValue(i1, j1, true);
        for (uint16_t k = 0; k < n; ++k) {
          if (getValue(j1, k) && !getValue(i1, k)) {
            queue.push({i1, k});
          } else if (getValue(k, i1) && !getValue(k, j1)) {
            queue.push({k, j1});
          }
        }
      }
    }
  }

 public:
  Poset(const uint8_t n, const uint8_t nthSmallest) : n(n), nthSmallest(nthSmallest) {
    std::memset(comparisonTable, false, n * n);
  };

  Poset() : Poset(0, 0, 0) {}

  // after func it holds: arr[i] < arr[j]
  void addComparison(const uint16_t i, const uint16_t j) {
    addComparisonTransitivRecursive(i, j);  // faster than iterative
  }

  uint8_t size() const { return n; }

  // true => arr[i] < arr[j]
  // ACHTUNG: is(a, b) == false IMPLIZIERT NICHT, dass arr[i] > arr[j]:
  // nur wenn arr[i] > arr[j] => false
  bool is(const uint16_t i, const uint16_t j) const { return getValue(i, j); }

  // can one determine n smallest element with current comparisons
  bool canDetermineNSmallest() const {
    if (0 == n) {
      return true;
    }
    for (uint16_t k = 0; k < n; ++k) {  // guess arr[k] is median
      uint8_t smaller = 0, bigger = 0;
      for (uint16_t i = 0; i < n; ++i) {
        if (getValue(i, k)) {
          ++smaller;
        } else if (getValue(k, i)) {
          ++bigger;
        }
      }
      if (1 + smaller + bigger == n && smaller == nthSmallest) {
        return true;
      }
    }
    return false;
  }

  bool operator==(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == poset.nthSmallest &&
           0 == std::memcmp(comparisonTable, poset.comparisonTable, n * poset.n);
  }

  size_t hash() const {
    constexpr int shift = 7;
    size_t result = 0;
    result ^= n;
    result = (result << shift) | (result >> (8 * sizeof(size_t) - shift));
    result ^= nthSmallest;
    for (uint16_t i = 0; i < n * n; ++i) {
      if (comparisonTable[i]) {
        result = (result << shift) | (result >> (8 * sizeof(size_t) - shift));
        result ^= i;
      }
    }
    return result;
  }

  template <size_t maxN2>
  friend std::ostream &operator<<(std::ostream &os, const Poset<maxN2> &poset);

  friend class Normalizer<maxN>;
};

template <size_t maxN>
struct std::hash<Poset<maxN>> {
  size_t operator()(const Poset<maxN> &poset) const { return poset.hash(); }
};

template <size_t maxN>
std::ostream &operator<<(std::ostream &os, const Poset<maxN> &poset) {
  os << "n = " << (uint16_t)poset.n << ", nthSmallest = " << (uint16_t)poset.nthSmallest;
  for (uint16_t i = 0; i < poset.n; ++i) {
    std::cout << '\n';
    for (uint16_t j = 0; j < poset.n; ++j) {
      std::cout << poset.getValue(i, j) << " ";
    }
  }
  return os;
}