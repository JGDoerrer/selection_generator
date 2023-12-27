#include <bits/stdc++.h>

const int min_n_comparisons[15][15] = {
    /* i=1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16 */
    /* n= 0 */ {},
    /* n= 1 */ {},
    /* n= 2 */ {1, 1},
    /* n= 3 */ {2, 3, 2},
    /* n= 4 */ {3, 4, 4, 3},
    /* n= 5 */ {4, 6, 6, 6, 4},
    /* n= 6 */ {5, 7, 8, 8, 7, 5},
    /* n= 7 */ {6, 8, 10, 10, 10, 8, 6},
    /* n= 8 */ {7, 9, 11, 12, 12, 11, 9, 7},
    /* n= 9 */ {8, 11, 12, 14, 14, 14, 12, 11, 8},
    /* n=10 */ {9, 12, 14, 15, 16, 16, 15, 14, 12, 9},
    /* n=11 */ {10, 13, 15, 17, 18, 18, 18, 17, 15, 13, 10},
    /* n=12 */ {11, 14, 17, 18, 19, 20, 20, 19, 18, 17, 14, 11}};
const int min_n_comparisons_len = 12;

#include "BS_thread_pool.hpp"
#include "poset.h"
#include "util.h"
// ===================
#include "normalizer.h"

// TODO: multithreading implementieren
// TODO: multithreading nauty testen
// TODO: ACHTUNG: post_normalize kaputt

// TODO: swap Operationen, memcpy, fine-tuning
// TODO: überall explicit static cast

constexpr size_t globalMaxComparisons = 25;
constexpr size_t globalMaxN = 15;
constexpr size_t threadCount = 20;

#include "cache.h"
#include "cache_old.h"

template<std::size_t maxN, std::size_t maxC>
using Cache = CacheTree<maxN, maxC>;
// using Cache = CacheSet<maxN, maxC>;

// siehe Section 5.3.3
// https://doc.lagout.org/science/0_Computer%20Science/2_Algorithms/The%20Art%20of%20Computer%20Programming%20%28vol.%203_%20Sorting%20and%20Searching%29%20%282nd%20ed.%29%20%5BKnuth%201998-05-04%5D.pdf
int upper_bound(const int n, int t) {
  t += 1;  // offset in Programm vs Paper
  if (t > (n + 1) / 2) {
    throw std::invalid_argument("it should hold t <= (n + 1) / 2");
  }
  if (1 == t) {
    // page 209, (5)
    return n - 1;  // exact
  } else if (2 == t) {
    // page 209, Threorem S.
    return n - 2 + ceil(log2(n));  // exact
  } else if (3 == t) {
    // page 213, (13)
    return n + 1 + ceil(log2((n - 1) / 4.0f)) + ceil(log2((n - 1) / 5.0f));
  }
  // page 210, (6); not used, because useless for n <= 15 (not fine enough)

  // page 212, (11)
  const int res1 = n - t + (t - 1) * ceil(log2(n + 2 - t));

  // page 212, (11); mit t = n + 1 - t => gibt evtl. bessere Werte
  const int res2 = n - (n + 1 - t) + ((n + 1 - t) - 1) * ceil(log2(n + 2 - (n + 1 - t)));
  return std::min(res1, res2);
}

// TODO: untested
double nCr(const int n, int k) {
  double result = 1;
  for (; k > 0; --k) {
    result *= (n - k + 1) / double(k);
  }
  return result;
}

int lower_bound(const int n, int t) {
  t += 1;  // offset in Programm vs Paper
  if (t > (n + 1) / 2) {
    throw std::invalid_argument("it should hold t <= (n + 1) / 2");
  }
  if (1 == t) {
    // page 209, (5)
    return n - 1;  // exact
  } else if (2 == t) {
    // page 209, Threorem S.
    return n - 2 + ceil(log2(n));  // exact
  }
  // page 213, (12)
  int sum = 0;
  for (int j = 0; j <= t - 2; ++j) {
    sum += ceil(log2((n - t + 2) / float(t + j)));
  }
  return n + t - 3 + sum;  // lower

  // page 213, (14); not used, because useless for n <= 15 (not fine enough)
}

enum SearchResult : uint8_t { FoundSolution, NoSolution, Unknown };

struct Statistics {
  size_t hashMatchLowerBound = 0;
  size_t hashMatchUpperBound = 0;
  size_t noSolution = 0;
  size_t bruteForce = 0;
};

std::ostream &operator<<(std::ostream &os, const Statistics &stats) {
  os << "(cache_l: " << stats.hashMatchLowerBound << ", cache_u: " << stats.hashMatchUpperBound
     << ", noSol: " << stats.noSolution << ", bruteForce: " << stats.bruteForce << ")";
  return os;
}