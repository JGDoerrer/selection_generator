#pragma once
#include <bits/stdc++.h>

#include "poset.h"
#include "util.h"

// A special case of option (b) is 0 < MAXN ≤ WORDSIZE, which implies that a set
// consists of a single setword. Some of the critical routines in nauty have special code to
// optimize performance in that case. The recommended way to compile for this case is to
// define MAXN to be the name WORDSIZE
#define MAXN WORDSIZE
#include "../nauty2_8_8/nauty.h"

template <size_t maxN>
class Poset;

template <size_t maxN>
class Normalizer {
 private:
  const int m = SETWORDSNEEDED(maxN);

  graph g[SETWORDSNEEDED(maxN) * maxN];
  graph result[SETWORDSNEEDED(maxN) * maxN];

  int lab[maxN];
  int ptn[maxN];
  int orbits[maxN];

 public:
  Normalizer() {
    assert(maxN <= WORDSIZE);
    nauty_check(WORDSIZE, m, maxN, NAUTYVERSIONID);
  }

  inline void reduceN(Poset<maxN> &poset) {
    uint8_t less[poset.n];
    uint8_t greater[poset.n];
    poset.getLessGreater(less, greater);

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

    std::vector<bool> oldTb(poset.comparisonTable);
    const uint8_t oldN = poset.n;
    poset.n = new_n;
    poset.nthSmallest -= n_less_dropped;
    poset.comparisonTable.resize(new_n * new_n);
    for (uint8_t i = 0; i < new_n; ++i) {
      for (uint8_t j = 0; j < new_n; ++j) {
        poset.set_less(i, j, oldTb[new_indices[i] * oldN + new_indices[j]]);
      }
    }
  }

  // invariant after method: i < n/2
  inline void reduceNthSmallest(Poset<maxN> &poset) {
    if (poset.n <= 2 * poset.nthSmallest) {
      poset.nthSmallest = poset.n - 1 - poset.nthSmallest;
      for (uint8_t i = 0; i < poset.n; ++i) {
        for (uint8_t j = i + 1; j < poset.n; ++j) {
          const bool temp = poset.is_less(i, j);
          poset.set_less(i, j, poset.is_less(j, i));
          poset.set_less(j, i, temp);
        }
      }
    }
  }

  inline void canonifyNauty(Poset<maxN> &poset) {
    EMPTYGRAPH(g, m, poset.n);
    for (uint16_t i = 0; i < poset.n; ++i) {
      for (uint16_t j = 0; j < poset.n; ++j) {
        if (poset.is_less(i, j)) {
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

    std::vector<bool> oldTb(poset.comparisonTable);
    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        poset.set_less(i, j, oldTb[lab[i] * poset.n + lab[j]]);
      }
    }
  }

  void normalize(Poset<maxN> &poset) {
    reduceN(poset);
    reduceNthSmallest(poset);

    if (true) {
      // use nauty
      canonifyNauty(poset);
    } else {
      // use mix
      uint8_t less[poset.n];
      uint8_t greater[poset.n];
      poset.getLessGreater(less, greater);

      std::vector<std::pair<uint64_t, uint8_t>> rowSum(poset.n);
      for (uint16_t i = 0; i < poset.n; ++i) {
        rowSum[i] = {100 * greater[i] + less[i], i};
      }

      std::sort(rowSum.begin(), rowSum.end());

      std::vector<bool> oldTb(poset.comparisonTable);
      for (uint8_t i = 0; i < poset.n; ++i) {
        for (uint8_t j = 0; j < poset.n; ++j) {
          poset.set_less(i, j, oldTb[rowSum[i].second * poset.n + rowSum[j].second]);
        }
      }
    }
  }
};