// https://quuxplusone.github.io/blog/2020/01/11/canonicalizing-matrices-with-nauty/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>

#include "../nauty2_8_8/nauty.h"

int main() {
  for (int iQ = 0; iQ < 5; ++iQ) {
    constexpr int n = 5;
    bool comparearray[n][n];
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        comparearray[i][j] = false;
      }
    }
    int offset = iQ;
    comparearray[(0 + offset) % n][(1 + offset) % n] = true;
    comparearray[(1 + offset) % n][(0 + offset) % n] = true;
    comparearray[(1 + offset) % n][(4 + offset) % n] = true;
    comparearray[(3 + offset) % n][(1 + offset) % n] = true;

    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        std::cout << comparearray[i][j] << " ";
      }
      std::cout << std::endl;
    }

    // ====================================

    const int m = SETWORDSNEEDED(n);
    nauty_check(WORDSIZE, m, n, NAUTYVERSIONID);

    DYNALLSTAT(graph, g, g_sz);
    DYNALLOC2(graph, g, g_sz, m, n, "malloc");
    EMPTYGRAPH(g, m, n);
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (comparearray[i][j]) {
          ADDONEARC(g, i, j, m);
        }
      }
    }

    DYNALLSTAT(int, lab, lab_sz);
    DYNALLOC1(int, lab, lab_sz, n, "malloc");
    for (int i = 0; i < n; ++i) {
      lab[i] = i;
    }

    DYNALLSTAT(int, ptn, ptn_sz);
    DYNALLOC1(int, ptn, ptn_sz, n, "malloc");
    for (int i = 0; i < n; ++i) ptn[i] = 1;  // hier 0 oder 1?
    ptn[n - 1] = 0;

    DYNALLSTAT(graph, result, result_sz);
    DYNALLOC2(graph, result, result_sz, m, n, "malloc");
    EMPTYGRAPH(result, m, n);

    DEFAULTOPTIONS_GRAPH(options);
    options.getcanon = TRUE;
    options.digraph = TRUE;

    DYNALLSTAT(int, orbits, orbits_sz);
    DYNALLOC1(int, orbits, orbits_sz, n, "malloc");

    statsblk stats;
    densenauty(g, lab, ptn, orbits, &options, &stats, m, n, result);
    assert(stats.errstatus == 0);

    // ====================================

    DYNALLSTAT(int, map, map_sz);
    DYNALLOC1(int, map, map_sz, n, "malloc");
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
        for (uint8_t k = 0; k < n; ++k) {
          const bool temp = comparearray[i][k];
          comparearray[i][k] = comparearray[min_idx][k];
          comparearray[min_idx][k] = temp;
        }
        for (uint8_t k = 0; k < n; ++k) {
          const bool temp = comparearray[k][i];
          comparearray[k][i] = comparearray[k][min_idx];
          comparearray[k][min_idx] = temp;
        }
      };
    }

    std::cout << std::endl;
    for (int i = 0; i < n; ++i) {
      std::cout << map[i] << ", ";
    }
    std::cout << std::endl;
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        std::cout << comparearray[i][j] << " ";
      }
      std::cout << std::endl;
    }
    std::cout << std::endl;
    std::cout << std::endl;
  }
}