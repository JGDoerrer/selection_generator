#pragma once
#include <bits/stdc++.h>

// #include "util.h"

#ifdef USE_NAUTY
#include "../nauty2_8_8/nauty.h"

DYNALLSTAT(graph, g, g_sz);
DYNALLSTAT(int, lab, lab_sz);
DYNALLSTAT(int, ptn, ptn_sz);
DYNALLSTAT(graph, result, result_sz);
DYNALLSTAT(int, orbits, orbits_sz);
DYNALLSTAT(int, map, map_sz);

void initNauty(const int maxN) {
  const int m = SETWORDSNEEDED(maxN);
  nauty_check(WORDSIZE, m, maxN, NAUTYVERSIONID);

  DYNALLOC1(int, lab, lab_sz, maxN, "malloc");
  DYNALLOC1(int, ptn, ptn_sz, maxN, "malloc");
  DYNALLOC1(int, orbits, orbits_sz, maxN, "malloc");
  DYNALLOC1(int, map, map_sz, maxN, "malloc");
  DYNALLOC2(graph, g, g_sz, m, maxN, "malloc");
  DYNALLOC2(graph, result, result_sz, m, maxN, "malloc");
}
#endif

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
    // how many elements are less than it
    std::vector<int> less(n, 0);
    std::vector<int> greater(n, 0);

    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (getValue(i, j)) {
          ++less[j];
          ++greater[i];
        }
      }
    }

    // can the element be ignored, because it is too large/small
    std::vector<int> dropped(n, false);
    int n_less_dropped = 0;

    for (int i = 0; i < n; ++i) {
      if (greater[i] > nthSmallest) {
        dropped[i] = true;
      } else if (less[i] >= n - nthSmallest) {  // oder >= ???
        dropped[i] = true;
        ++n_less_dropped;
      }
    }

    // maps the old indices to the new ones
    std::vector<int> new_indices(n, 0);
    int new_n = 0;
    int b = n - 1;

    for (int i = 0; i < n; ++i) {
      if (!dropped[i]) {
        new_indices[new_n++] = i;
      } else {
        new_indices[b--] = i;
      }
    }

    // make the new poset
    bool oldTb[n * n];
    for (int i = 0; i < n * n; ++i) {
      oldTb[i] = comparisonTable[i];
    }
    int oldN = n;

    n = new_n;
    nthSmallest -= n_less_dropped;

    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (oldTb[new_indices[i] * oldN + new_indices[j]]) {
          setValue(i, j, true);
        } else {
          setValue(i, j, false);
        }
      }
    }

    post_normalize();
  }

#ifdef USE_NAUTY
  void post_normalize() {
    const int m = SETWORDSNEEDED(n);
    nauty_check(WORDSIZE, m, n, NAUTYVERSIONID);

    EMPTYGRAPH(g, m, n);
    for (uint16_t i = 0; i < n; ++i) {
      for (uint16_t j = 0; j < n; ++j) {
        if (getValue(i, j)) {
          ADDONEARC(g, i, j, m);
        }
      }
    }

    for (uint8_t i = 0; i < n; ++i) {
      lab[i] = i;
    }

    for (uint8_t i = 0; i < n; ++i) {
      ptn[i] = 1;  // hier 0 oder 1?
    }
    ptn[n - 1] = 0;

    EMPTYGRAPH(result, m, n);

    DEFAULTOPTIONS_GRAPH(options);
    options.getcanon = TRUE;
    options.digraph = TRUE;

    statsblk stats;
    densenauty(g, lab, ptn, orbits, &options, &stats, m, n, result);
    assert(stats.errstatus == 0);

    for (uint8_t i = 0; i < n; ++i) {
      map[lab[i]] = i;
    }

    // TODO: erstelle comaprsionTable neu => schneller

    int i, j, min_idx;
    for (i = 0; i < n - 1; ++i) {
      min_idx = i;
      for (j = i + 1; j < n; ++j) {
        if (map[j] < map[min_idx]) min_idx = j;
      }

      if (min_idx != i) {
        std::swap(map[i], map[min_idx]);
        swapCols(i, min_idx);
        swapRows(i, min_idx);
      }
    }
  }
#else
  void post_normalize() {
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
        if (getValue(i, j)) {
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
    //     if (getValue(j, i)) {
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
#endif

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