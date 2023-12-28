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
 public:
  const int m = SETWORDSNEEDED(maxN);

  graph g[SETWORDSNEEDED(maxN) * maxN];
  graph result[SETWORDSNEEDED(maxN) * maxN];

  int lab[maxN];
  int ptn[maxN];
  int orbits[maxN];

  inline Poset<maxN> &reduce_n(Poset<maxN> &poset) {
    uint8_t less[poset.n];
    uint8_t greater[poset.n];
    poset.calculate_relations(greater, less);  // TODO: MACHT DAS SINN???

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

    if (new_n != poset.n) {
      const Poset<maxN> oldPoset(poset);
      poset.n = new_n;
      poset.nthSmallest -= n_less_dropped;
      poset.comparisonTable.reset();
      for (uint8_t i = 0; i < new_n; ++i) {
        for (uint8_t j = 0; j < new_n; ++j) {
          poset.set_less(i, j, oldPoset.is_less(new_indices[i], new_indices[j]));
        }
      }

      if (poset.n <= 2 * poset.nthSmallest) {
        poset.dual();
      }
    }
    return poset;
  }

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

 public:
  Normalizer() {
    assert(maxN <= WORDSIZE);
    nauty_check(WORDSIZE, m, maxN, NAUTYVERSIONID);
  }

  inline Poset<maxN> &canonify(Poset<maxN> &poset) {
    if constexpr (false) {
      uint8_t less[poset.n];
      uint8_t greater[poset.n];
      poset.calculate_relations(less, greater);

      std::vector<std::pair<uint64_t, uint8_t>> in_out_degree(poset.n);
      for (uint8_t i = 0; i < poset.n; ++i) {
        in_out_degree[i] = {uint64_t(maxN) * uint64_t(greater[i]) + uint64_t(less[i]), i};
      }

      std::sort(in_out_degree.begin(), in_out_degree.end());

      uint8_t duplicats = 0;
      for (uint8_t i = 1; i < poset.n; ++i) {
        if (in_out_degree[i - 1].first == in_out_degree[i].first) {
          ++duplicats;
        }
      }

      if (0 == duplicats) {
        const Poset<maxN> oldPoset(poset);
        for (uint8_t i = 0; i < poset.n; ++i) {
          for (uint8_t j = 0; j < poset.n; ++j) {
            poset.set_less(i, j, oldPoset.is_less(in_out_degree[i].second, in_out_degree[j].second));
          }
        }
        return poset;
      }
    }

    canonify_nauty(poset);
    return poset;
  }

  void normalize(Poset<maxN> &poset) {
    reduce_n(poset);
    canonify(poset);
  }
};

// nur nauty: time '1.632s + 4.201s = 5.833s': n = 10, i = 4, (cache_l: 2016256, cache_u: 288692, noSol: 12, bruteForce:
// 72555), cache = (59544 + 4809 = 64353), comparisons: 16

// eig. Opti: time '1.563s + 4.122s = 5.685s': n = 10, i = 4, (cache_l: 2037863, cache_u: 288844, noSol: 12, bruteForce:
// 73298), cache = (59695 + 4808 = 64503), comparisons: 16