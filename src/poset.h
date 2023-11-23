#pragma once
#include <bits/stdc++.h>

#include "util.h"
using namespace std;

template <size_t maxN>
class Poset {
 private:
  const uint8_t n;
  const uint8_t nthSmallest;
  uint8_t comparisonsDone = 0;
  bool comparisonTable[maxN * maxN];

  inline void swap(const size_t i1, const size_t j1, const size_t i2, const size_t j2) {
    const bool temp = comparisonTable[i1 * n + j1];
    comparisonTable[i1 * n + j1] = comparisonTable[i2 * n + j2];
    comparisonTable[i2 * n + j2] = temp;
  }

  inline void swapRows(const size_t i, const size_t j) {
    for (size_t k = 0; k < n; ++k) {
      swap(i, k, j, k);
    }
  }

  inline void swapCols(const size_t i, const size_t j) {
    for (size_t k = 0; k < n; ++k) {
      swap(k, i, k, j);
    }
  }

  inline void set(const size_t i, const size_t j, const bool value) { comparisonTable[i * n + j] = value; }

  inline bool get(const size_t i, const size_t j) const { return comparisonTable[i * n + j]; }

  void addComparisonTransitivRecursive(const size_t i, const size_t j) {
    if (false == get(i, j)) {
      set(i, j, true);

      for (size_t k = 0; k < n; ++k) {
        if (get(j, k) && !get(i, k)) {
          addComparisonTransitivRecursive(i, k);
        } else if (get(k, i) && !get(k, j)) {
          addComparisonTransitivRecursive(k, j);
        }
      }
    }
  }

  inline void addComparisonTransitivIterative(const size_t i, const size_t j) {
    queue<pair<size_t, size_t>> queue;
    queue.push({i, j});
    while (0 != queue.size()) {
      const auto &[i1, j1] = queue.front();
      queue.pop();

      if (false == get(i1, j1)) {
        set(i1, j1, true);
        for (size_t k = 0; k < n; ++k) {
          if (get(j1, k) && !get(i1, k)) {
            queue.push({i1, k});
          } else if (get(k, i1) && !get(k, j1)) {
            queue.push({k, j1});
          }
        }
      }
    }
  }

 public:
  Poset(const uint8_t n, const uint8_t nthSmallest) : n(n), nthSmallest(nthSmallest) {
    for (size_t i = 0; i < n * n; ++i) {
      comparisonTable[i] = false;
    }
  };

  uint8_t size() const { return n; }

  uint8_t nthSmallestElement() const { return nthSmallest; }

  // true => arr[i] < arr[j]
  // ACHTUNG: is(a, b) == false IMPLIZIERT NICHT, dass arr[i] > arr[j]:
  // nur wenn arr[i] > arr[j] => false
  bool is(const size_t i, const size_t j) const { return get(i, j); }

  // after func it holds: arr[i] < arr[j]
  void addComparison(const size_t i, const size_t j) {
    ++comparisonsDone;
    addComparisonTransitivRecursive(i, j);  // faster than iterative
  }

  // can one determine n smallest element with current comparisons
  bool canDetermineNSmallest() const {
    for (size_t k = 0; k < n; ++k) {  // guess arr[k] is median
      uint8_t smaller = 0, bigger = 0;
      for (size_t i = 0; i < n; ++i) {
        if (get(i, k)) {
          ++smaller;
        } else if (get(k, i)) {
          ++bigger;
        }
      }
      if (1 + smaller + bigger == n && smaller == nthSmallest) {
        return true;
      }
    }
    return false;
  }

  void normalize() {
    // TODO: geht besser als O(n^3), bestes möglich evtl: O(n)

    uint64_t rowSum[n], colSum[n];
    for (size_t i = 0; i < n; ++i) {
      rowSum[i] = 0;
      colSum[i] = 0;
      // TODO: reinterpret case -> O(1)
      for (size_t j = 0; j < n; ++j) {
        if (get(i, j)) {
          rowSum[i] = 2 * rowSum[i] + 1;
        } else if (get(j, i)) {
          colSum[i] = 2 * colSum[i] + 1;
        }
      }
    }

    for (size_t fromY = 0; fromY < n; ++fromY) {
      uint64_t maxVal = rowSum[fromY];
      size_t maxPos = fromY;
      for (size_t y = fromY + 1; y < n; ++y) {
        if (maxVal < rowSum[y]) {
          maxVal = rowSum[y];
          maxPos = y;
        }
      }

      if (fromY != maxPos) {
        // TODO: memcpy
        const uint64_t temp = rowSum[fromY];
        rowSum[fromY] = rowSum[maxPos];
        rowSum[maxPos] = temp;
        swapRows(fromY, maxPos);
        swapCols(fromY, maxPos);
      }
    }

    for (size_t fromX = 0; fromX < n; ++fromX) {
      uint64_t maxVal = colSum[fromX];
      size_t maxPos = fromX;
      for (size_t x = fromX + 1; x < n && rowSum[fromX] == rowSum[x]; ++x) {
        if (maxVal < colSum[x]) {
          maxVal = colSum[x];
          maxPos = x;
        }
      }

      if (fromX != maxPos) {
        const uint64_t temp = colSum[fromX];
        colSum[fromX] = colSum[maxPos];
        colSum[maxPos] = temp;
        swapCols(fromX, maxPos);
      }
    }
  }

  bool operator==(const Poset<maxN> &poset) const {
    if (n != poset.n || nthSmallest != nthSmallest) {
      return false;
    }
    for (size_t i = 0; i < n * n; ++i) {
      if (comparisonTable[i] != poset.comparisonTable[i]) {
        return false;
      }
    }
    if (comparisonsDone != poset.comparisonsDone) {
      return false;
    }
    return true;
  }

  size_t hash() const {
    // std::cout << "ERROR" << std::endl;
    size_t result = std::hash<int>{}(n) ^ std::hash<int>{}(nthSmallest);
    result ^= std::hash<int>{}(comparisonsDone);
    for (size_t i = 0; i < n * n; ++i) {
      if (comparisonTable[i]) {
        result = result ^ std::hash<int>{}(i);
      }
    }
    // ((hash<string>()(k.first) ^ (hash<string>()(k.second) << 1)) >> 1) ^
    // (hash<int>()(k.third) << 1);
    return result;
  }

  template <size_t maxN2>
  friend ostream &operator<<(ostream &os, const Poset<maxN2> &poset);
};

template <size_t maxN>
struct std::hash<Poset<maxN>> {
  size_t operator()(const Poset<maxN> &poset) const { return poset.hash(); }
};

template <size_t maxN>
ostream &operator<<(ostream &os, const Poset<maxN> &poset) {
  os << "n = " << (int)poset.n << ", nthSmallest = " << (int)poset.nthSmallest
     << ", comparisonsDone = " << (int)poset.comparisonsDone;
  for (int i = 0; i < poset.n; ++i) {
    cout << '\n';
    for (int j = 0; j < poset.n; ++j) {
      cout << poset.get(i, j) << " ";
    }
  }
  return os;
}