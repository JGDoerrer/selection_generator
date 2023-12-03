#pragma once
#include <bits/stdc++.h>

#include "poset.h"

#define USE_NAUTY

#ifdef USE_NAUTY
// A special case of option (b) is 0 < MAXN ≤ WORDSIZE, which implies that a set
// consists of a single setword. Some of the critical routines in nauty have special code to
// optimize performance in that case. The recommended way to compile for this case is to
// define MAXN to be the name WORDSIZE
#define MAXN WORDSIZE
#include "../nauty2_8_8/nauty.h"
#endif

template <size_t maxN>
class Poset;

template <size_t maxN>
class Normalizer {
 private:
#ifdef USE_NAUTY
  const int m = SETWORDSNEEDED(maxN);

  graph g[SETWORDSNEEDED(maxN) * maxN];
  graph result[SETWORDSNEEDED(maxN) * maxN];

  int lab[maxN];
  int ptn[maxN];
  int orbits[maxN];
#else
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
#endif

 public:
  Normalizer() {
#ifdef USE_NAUTY
    assert(maxN <= WORDSIZE);
    nauty_check(WORDSIZE, m, maxN, NAUTYVERSIONID);
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
    uint8_t new_indices[poset.n];
    uint8_t n_less_dropped = 0;

    // maps the old indices to the new ones
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

#ifdef USE_NAUTY
    // make the new poset
    std::vector<bool> oldTb(poset.comparisonTable);
    const uint8_t oldN = poset.n;

    poset.n = new_n;
    poset.nthSmallest -= n_less_dropped;

    EMPTYGRAPH(g, m, poset.n);
    for (uint16_t i = 0; i < poset.n; ++i) {
      for (uint16_t j = 0; j < poset.n; ++j) {
        if (oldTb[new_indices[i] * oldN + new_indices[j]]) {
          ADDONEARC(g, i, j, m);
        }
      }
    }

    for (uint8_t i = 0; i < poset.n; ++i) {
      lab[i] = i;
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

    poset.comparisonTable.resize(poset.n * poset.n);
    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        poset.setValue(i, j, oldTb[new_indices[lab[i]] * oldN + new_indices[lab[j]]]);
      }
    }
#else
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

    // TODO: geht besser
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
#endif
  }
};