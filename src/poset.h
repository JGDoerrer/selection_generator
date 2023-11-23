#pragma once
#include <bits/stdc++.h>

#include "util.h"
using namespace std;

template <size_t maxN>
class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;
  bool comparisonTable[maxN * maxN];

  inline void swap(const uint16_t i1, const uint16_t j1, const uint16_t i2, const uint16_t j2) {
    const bool temp = comparisonTable[i1 * n + j1];
    comparisonTable[i1 * n + j1] = comparisonTable[i2 * n + j2];
    comparisonTable[i2 * n + j2] = temp;
  }

  inline void swapRows(const uint16_t i, const uint16_t j) {
    for (uint8_t k = 0; k < n; ++k) {
      swap(i, k, j, k);
    }
  }

  inline void swapCols(const uint16_t i, const uint16_t j) {
    for (uint8_t k = 0; k < n; ++k) {
      swap(k, i, k, j);
    }
  }

  inline void set(const uint16_t i, const uint16_t j, const bool value) { comparisonTable[i * n + j] = value; }

  inline bool get(const uint16_t i, const uint16_t j) const { return comparisonTable[i * n + j]; }

  void addComparisonTransitivRecursive(const uint16_t i, const uint16_t j) {
    if (false == get(i, j)) {
      set(i, j, true);

      for (uint16_t k = 0; k < n; ++k) {
        if (get(j, k) && !get(i, k)) {
          addComparisonTransitivRecursive(i, k);
        } else if (get(k, i) && !get(k, j)) {
          addComparisonTransitivRecursive(k, j);
        }
      }
    }
  }

  inline void addComparisonTransitivIterative(const uint16_t i, const uint16_t j) {
    queue<pair<uint16_t, uint16_t>> queue;
    queue.push({i, j});
    while (0 != queue.size()) {
      const auto &[i1, j1] = queue.front();
      queue.pop();

      if (false == get(i1, j1)) {
        set(i1, j1, true);
        for (uint16_t k = 0; k < n; ++k) {
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
    std::memset(comparisonTable, false, n * n);
  };

  Poset() : Poset(0, 0, 0) {}

  Poset<maxN> add_relation(const uint16_t i, const uint16_t j) const {
    Poset<maxN> newPoset = *this;
    newPoset.addComparison(i, j);
    newPoset.normalize();
    return newPoset;
  };

  // after func it holds: arr[i] < arr[j]
  void addComparison(const uint16_t i, const uint16_t j) {
    addComparisonTransitivRecursive(i, j);  // faster than iterative
  }

  uint8_t size() const { return n; }

  // true => arr[i] < arr[j]
  // ACHTUNG: is(a, b) == false IMPLIZIERT NICHT, dass arr[i] > arr[j]:
  // nur wenn arr[i] > arr[j] => false
  bool is(const uint16_t i, const uint16_t j) const { return get(i, j); }

  // can one determine n smallest element with current comparisons
  bool canDetermineNSmallest() const {
    for (uint16_t k = 0; k < n; ++k) {  // guess arr[k] is median
      uint8_t smaller = 0, bigger = 0;
      for (uint16_t i = 0; i < n; ++i) {
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

  // // Kante von (v) -> (w) g.d.w. a[v] < a[w]
  // void visit(const int v, vector<int> &result, vector<bool> &visited) {
  //   visited[v] = true;
  //   for (uint8_t w = 0; w < n; ++w) {
  //     if (is(v, w) && !visited[w]) {
  //       visit(w, result, visited);
  //     }
  //   }
  //   result.push_back(v);
  // }

  void normalize() {
    // TODO: geht besser als O(n^3), bestes möglich evtl: O(n)

    // vector<bool> visited(n, false);
    // vector<int> result{};
    // for (int v = 0; v < n; ++v) {
    //   if (!visited[v]) {
    //     visit(v, result, visited);
    //   }
    // }
    // // toposorting := std::reverse(result);

    uint64_t rowSum[n];
    for (uint16_t i = 0; i < n; ++i) {
      rowSum[i] = 0;
      // TODO: reinterpret case -> O(1)
      for (uint16_t j = 0; j < n; ++j) {
        if (get(i, j)) {
          ++rowSum[i];  // = 2 * rowSum[i] + 1;
        }
      }
    }

    for (uint16_t fromY = 0; fromY < n; ++fromY) {
      uint64_t maxVal = rowSum[fromY];
      uint16_t maxPos = fromY;
      for (uint16_t y = fromY + 1; y < n; ++y) {
        if (maxVal < rowSum[y]) {
          maxVal = rowSum[y];
          maxPos = y;
        }
      }

      if (fromY != maxPos) {
        // TODO: memcpy
        std::swap(rowSum[fromY], rowSum[maxPos]);
        swapRows(fromY, maxPos);
        swapCols(fromY, maxPos);
      }
    }

    // uint64_t colSum[n];
    // for (uint16_t i = 0; i < n; ++i) {
    //   colSum[i] = 0;
    //   // TODO: reinterpret case -> O(1)
    //   for (uint16_t j = 0; j < n; ++j) {
    //     if (get(j, i)) {
    //       colSum[i]++;// = 2 * colSum[i] + 1;
    //     }
    //   }
    // }

    // for (uint16_t fromX = 0; fromX < n; ++fromX) {
    //   uint64_t maxVal = colSum[fromX];
    //   uint16_t maxPos = fromX;
    //   for (uint16_t x = fromX + 1; x < n && rowSum[fromX] == rowSum[x]; ++x) {
    //     if (maxVal < colSum[x]) {
    //       maxVal = colSum[x];
    //       maxPos = x;
    //     }
    //   }

    //   if (fromX != maxPos) {
    //     std::swap(colSum[fromX], colSum[maxPos]);
    //     swapCols(fromX, maxPos);
    //   }
    // }
  }

  bool operator==(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == nthSmallest &&
           0 == std::memcmp(comparisonTable, poset.comparisonTable, n * n);
  }

  size_t hash() const {
    const int shift = 7;
    size_t result = 0;
    result ^= n;
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
  friend ostream &operator<<(ostream &os, const Poset<maxN2> &poset);
};

template <size_t maxN>
struct std::hash<Poset<maxN>> {
  size_t operator()(const Poset<maxN> &poset) const { return poset.hash(); }
};

template <size_t maxN>
ostream &operator<<(ostream &os, const Poset<maxN> &poset) {
  os << "n = " << (uint16_t)poset.n << ", nthSmallest = " << (uint16_t)poset.nthSmallest
     << ", comparisonsDone = " << (uint16_t)poset.comparisonsDone;
  for (uint16_t i = 0; i < poset.n; ++i) {
    cout << '\n';
    for (uint16_t j = 0; j < poset.n; ++j) {
      cout << poset.get(i, j) << " ";
    }
  }
  return os;
}