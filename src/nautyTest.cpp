// https://quuxplusone.github.io/blog/2020/01/11/canonicalizing-matrices-with-nauty/
#include <bits/stdc++.h>

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

void print2dArray(const int n, const bool* const comparisonTable) {
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
      std::cout << comparisonTable[i * n + j] << " ";
    }
    std::cout << std::endl;
  }
}

// Poset.h
constexpr size_t maxN = 15;
const uint8_t n = 5;
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

inline void swapVertex(const uint16_t i, const uint16_t j) {
  swapCols(i, j);
  swapRows(i, j);
}

void normalize() {
  const int m = SETWORDSNEEDED(n);
  nauty_check(WORDSIZE, m, n, NAUTYVERSIONID);

  EMPTYGRAPH(g, m, n);
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
      if (comparisonTable[i * n + j]) {
        ADDONEARC(g, i, j, m);
      }
    }
  }

  for (int i = 0; i < n; ++i) {
    lab[i] = i;
  }

  for (int i = 0; i < n; ++i) {
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

  for (int i = 0; i < n; ++i) {
    map[lab[i]] = i;
  }

  int i, j, min_idx;
  for (i = 0; i < n - 1; i++) {
    min_idx = i;
    for (j = i + 1; j < n; j++) {
      if (map[j] < map[min_idx]) min_idx = j;
    }

    if (min_idx != i) {
      std::swap(map[i], map[min_idx]);
      swapVertex(i, min_idx);
    };
  }
}

int main() {
  initNauty(maxN);

  for (int offset = 0; offset < 5; ++offset) {
    for (int i = 0; i < n * n; ++i) {
      comparisonTable[i] = false;
    }
    comparisonTable[((0 + offset) % n) * n + (1 + offset) % n] = true;
    comparisonTable[((1 + offset) % n) * n + (0 + offset) % n] = true;
    comparisonTable[((1 + offset) % n) * n + (4 + offset) % n] = true;
    comparisonTable[((3 + offset) % n) * n + (1 + offset) % n] = true;

    print2dArray(n, comparisonTable);

    normalize();

    std::cout << std::endl;
    print2dArray(n, comparisonTable);
    std::cout << std::endl << std::endl;
  }
}