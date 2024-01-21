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

  inline Poset<maxN> &canonify_nauty(Poset<maxN> &poset) {
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

    const Poset<maxN> oldPoset(poset);
    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        poset.set_less(i, j, oldPoset.is_less(lab[i], lab[j]));
      }
    }
    return poset;
  }

  inline std::vector<int> canonify_nauty_indicies(const Poset<maxN> &poset) {
    EMPTYGRAPH(g, m, poset.n);
    for (uint16_t i = 0; i < poset.n; ++i) {
      for (uint16_t j = 0; j < poset.n; ++j) {
        if (poset.is_less(i, j)) {
          ADDONEARC(g, i, j, m);
        }
      }
    }

    std::vector<int> lab2(poset.n);
    for (uint8_t i = 0; i < poset.n; ++i) {
      lab2[i] = i;
      ptn[i] = 0;  // hier 0 oder 1?
    }
    ptn[poset.n - 1] = 0;

    EMPTYGRAPH(result, m, poset.n);

    DEFAULTOPTIONS_GRAPH(options);
    options.getcanon = TRUE;
    options.digraph = TRUE;

    statsblk stats;
    densenauty(g, lab2.data(), ptn, orbits, &options, &stats, m, poset.n, result);
    assert(stats.errstatus == 0);

    return lab2;
  }

 public:
  Normalizer() {
    assert(maxN <= WORDSIZE);
    nauty_check(WORDSIZE, m, maxN, NAUTYVERSIONID);
  }

  friend class Poset<maxN>;
};