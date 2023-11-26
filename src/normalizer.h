#pragma once
#include <bits/stdc++.h>

#include "poset.h"

#define USE_NAUTY
#ifdef USE_NAUTY
#include "../nauty2_8_8/nauty.h"
#endif

template <size_t maxN>
class Poset;

template <size_t maxN>
class Normalizer {
 private:
#ifdef USE_NAUTY
  graph *g;
  size_t g_sz = 0;
  int *lab;
  size_t lab_sz = 0;
  int *ptn;
  size_t ptn_sz = 0;
  graph *result;
  size_t result_sz = 0;
  int *orbits;
  size_t orbits_sz = 0;
#endif

  inline void swap(Poset<maxN> &poset, const uint16_t i1, const uint16_t j1, const uint16_t i2, const uint16_t j2) {
    const bool temp = poset.comparisonTable[i1 * poset.n + j1];
    poset.comparisonTable[i1 * poset.n + j1] = poset.comparisonTable[i2 * poset.n + j2];
    poset.comparisonTable[i2 * poset.n + j2] = temp;
  }

  inline void swapRows(Poset<maxN> &poset, const uint16_t i, const uint16_t j) {
    for (uint8_t k = 0; k < poset.n; ++k) {
      swap(poset, i, k, j, k);
    }
  }

  inline void swapCols(Poset<maxN> &poset, const uint16_t i, const uint16_t j) {
    for (uint8_t k = 0; k < poset.n; ++k) {
      swap(poset, k, i, k, j);
    }
  }

#ifdef USE_NAUTY
  void canonicalize(Poset<maxN> &poset) {
    const int m = SETWORDSNEEDED(poset.n);
    nauty_check(WORDSIZE, m, poset.n, NAUTYVERSIONID);

    EMPTYGRAPH(g, m, poset.n);
    for (uint16_t i = 0; i < poset.n; ++i) {
      for (uint16_t j = 0; j < poset.n; ++j) {
        if (poset.getValue(i, j)) {
          ADDONEARC(g, i, j, m);
        }
      }
    }

    for (uint8_t i = 0; i < poset.n; ++i) {
      lab[i] = i;
    }

    for (uint8_t i = 0; i < poset.n; ++i) {
      ptn[i] = 0;  // hier 0 oder 1?
    }
    ptn[poset.n - 1] = 0;

    EMPTYGRAPH(result, m, poset.n);

    DEFAULTOPTIONS_GRAPH(options);
    options.getcanon = TRUE;
    options.digraph = TRUE;

    statsblk stats;
    densenauty(g, lab, ptn, orbits, &options, &stats, m, poset.n, result);
    assert(stats.errstatus == 0);

    // make the new poset
    bool oldTb[poset.n * poset.n];
    for (int i = 0; i < poset.n * poset.n; ++i) {
      oldTb[i] = poset.comparisonTable[i];
    }

    for (int i = 0; i < poset.n; ++i) {
      for (int j = 0; j < poset.n; ++j) {
        poset.setValue(i, j, oldTb[lab[i] * poset.n + lab[j]]);
      }
    }
  }
#else
  // TODO: geht besser
  void canonicalize(Poset<maxN> &poset) {
    uint64_t rowSum[poset.n];
    for (uint16_t i = 0; i < poset.n; ++i) {
      rowSum[i] = 0;
      // TODO: reinterpret case -> O(1)
      for (uint16_t j = 0; j < poset.n; ++j) {
        if (poset.getValue(i, j)) {
          ++rowSum[i];  // = 2 * rowSum[i] + 1;
        }
      }
    }

    for (uint16_t fromY = 0; fromY < poset.n; ++fromY) {
      uint64_t maxVal = rowSum[fromY];
      uint16_t maxPos = fromY;
      for (uint16_t y = fromY + 1; y < poset.n; ++y) {
        if (maxVal < rowSum[y]) {
          maxVal = rowSum[y];
          maxPos = y;
        }
      }

      if (fromY != maxPos) {
        // TODO: memcpy
        std::swap(rowSum[fromY], rowSum[maxPos]);
        swapRows(poset, fromY, maxPos);
        swapCols(poset, fromY, maxPos);
      }
    }

    // uint64_t colSum[poset.n];
    // for (uint16_t i = 0; i < poset.n; ++i) {
    //   colSum[i] = 0;
    //   // TODO: reinterpret case -> O(1)
    //   for (uint16_t j = 0; j < poset.n; ++j) {
    //     if (getValue(j, i)) {
    //       colSum[i]++;// = 2 * colSum[i] + 1;
    //     }
    //   }
    // }

    // for (uint16_t fromX = 0; fromX < poset.n; ++fromX) {
    //   uint64_t maxVal = colSum[fromX];
    //   uint16_t maxPos = fromX;
    //   for (uint16_t x = fromX + 1; x < poset.n && rowSum[fromX] == rowSum[x]; ++x) {
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

 public:
  Normalizer() {
#ifdef USE_NAUTY
    const int m = SETWORDSNEEDED(maxN);
    nauty_check(WORDSIZE, m, maxN, NAUTYVERSIONID);

    DYNALLOC1(int, lab, lab_sz, maxN, "malloc");
    DYNALLOC1(int, ptn, ptn_sz, maxN, "malloc");
    DYNALLOC1(int, orbits, orbits_sz, maxN, "malloc");
    DYNALLOC2(graph, g, g_sz, m, maxN, "malloc");
    DYNALLOC2(graph, result, result_sz, m, maxN, "malloc");
#endif
  }

  ~Normalizer() {
#ifdef USE_NAUTY
    DYNFREE(lab, lab_sz);
    DYNFREE(ptn, ptn_sz);
    DYNFREE(orbits, orbits_sz);
    DYNFREE(g, g_sz);
    DYNFREE(result, result_sz);
#endif
  }

  void normalize(Poset<maxN> &poset) {
    // how many elements are less than it
    uint8_t less[poset.n], greater[poset.n];
    std::memset(less, 0, poset.n);
    std::memset(greater, 0, poset.n);
    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        if (poset.getValue(i, j)) {
          ++less[j];
          ++greater[i];
        }
      }
    }

    // can the element be ignored, because it is too large/small
    uint8_t n_less_dropped = 0;

    // maps the old indices to the new ones
    uint8_t new_indices[poset.n];
    uint8_t new_n = 0;
    uint8_t b = poset.n - 1;

    for (int i = 0; i < poset.n; ++i) {
      if (poset.nthSmallest < greater[i]) {
        new_indices[b--] = i;
      } else if ((poset.n - 1) - poset.nthSmallest < less[i]) {
        ++n_less_dropped;
        new_indices[b--] = i;
      } else {
        new_indices[new_n++] = i;
      }
    }

    // make the new poset
    bool oldTb[poset.n * poset.n];
    for (uint16_t i = 0; i < poset.n * poset.n; ++i) {
      oldTb[i] = poset.comparisonTable[i];
    }
    const uint8_t oldN = poset.n;

    poset.n = new_n;
    poset.nthSmallest -= n_less_dropped;

    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        poset.setValue(i, j, oldTb[new_indices[i] * oldN + new_indices[j]]);
      }
    }

    canonicalize(poset);
  }
};