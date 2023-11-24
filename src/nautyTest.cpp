// https://quuxplusone.github.io/blog/2020/01/11/canonicalizing-matrices-with-nauty/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>

#include "../nauty2_8_8/nauty.h"

int main() {
  constexpr int n = 3;
  bool comparearray[n][n];
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
      comparearray[i][j] = false;
    }
  }
  comparearray[1][0] = true;
  comparearray[2][1] = true;

  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
      std::cout << comparearray[i][j] << " ";
    }
    std::cout << std::endl;
  }

  // ====================================

  const int m = SETWORDSNEEDED(n);
  std::cout << "m = " << m << std::endl;
  nauty_check(WORDSIZE, m, n, NAUTYVERSIONID);

  DYNALLSTAT(graph, g, g_sz);
  DYNALLOC2(graph, g, g_sz, m, n, "malloc");
  EMPTYGRAPH(g, m, n);
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
      if (comparearray[i][j]) {
        ADDONEEDGE(g, i, j, m);
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
  for (int i = 0; i < n; ++i) ptn[i] = 0;

  DYNALLSTAT(graph, result, result_sz);
  DYNALLOC2(graph, result, result_sz, m, n, "malloc");
  EMPTYGRAPH(result, m, n);

  DEFAULTOPTIONS_GRAPH(options);
  options.defaultptn = false;
  options.getcanon = true;
  options.digraph = true;

  DYNALLSTAT(int, orbits, orbits_sz);
  DYNALLOC1(int, orbits, orbits_sz, n, "malloc");

  statsblk stats;
  densenauty(g, lab, ptn, orbits, &options, &stats, m, n, result);
  assert(stats.errstatus == 0);

  for (int i = 0; i < n; ++i) {
    std::cout << "permutation: " << lab[i] << " " << std::endl;
  }
}