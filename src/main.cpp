// g++ -Ddev -Wall -Wextra -Wconversion -Wno-unknown-pragmas
// -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
// -O2 -std=c++17 -g ./template.cpp -o program.out; ./program.out
#define USE_NAUTY
constexpr bool USE_PAIR_MODE = false;

#include <bits/stdc++.h>

#include "poset.h"
#include "util.h"

// TODO: create make file, compile option `march native`
// TODO: ACHTUNG: normalize kaputt (optional: wenn 0 kleiner als alle -> liste mit n - 1 Elementen)
// TODO: Isomorphie-Test mit nauty-c

// TODO: cache nicht löschen nach iteration
// TODO: custom ordering
// TODO: swap Operationen, memcpy, fine-tuning
// TODO: überall explicit static cast
// TODO: Multi-Threading

const static int min_n_comparisons[15][15] = {
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
    /* n=12 */ {11, 14, 17, 18, 19, 21, 21, 19, 18, 17, 14, 11}};

struct Statistics {
  int functionCalls = 0;
  int hashMatches = 0;
};

std::ostream &operator<<(std::ostream &os, const Statistics &stats) {
  os << "calls = " << stats.functionCalls << ", hits = " << stats.hashMatches;
  return os;
}

template <size_t maxN>
/// @param cache_maximumReeched enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
///                             werden kann; z.B. wenn cache_maximumReeched[poset] = 2, dann: benötige MEHR ALS 2
///                             Schritte, um Poset zu lösen
/// @param cache_solution enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
///                       cache_solution[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
/// @return true, wenn Median in poset in max. `maxComparisons` gefunden werden kann
bool search(const Poset<maxN> &poset, std::unordered_map<Poset<maxN>, int> &cache_maximumReeched,
            std::unordered_map<Poset<maxN>, int> &cache_solution, const int maxComparisons, Statistics &statistics,
            const int comparisonsDone = 0) {
  bool result = false;

  if (cache_maximumReeched.find(poset) != cache_maximumReeched.end() &&
      maxComparisons - comparisonsDone <= cache_maximumReeched[poset]) {
    ++statistics.hashMatches;
    result = false;
  } else if (cache_solution.find(poset) != cache_solution.end() && comparisonsDone <= cache_solution[poset]) {
    ++statistics.hashMatches;
    result = true;
  } else if (poset.canDetermineNSmallest()) {
    result = true;
  } else if (comparisonsDone == maxComparisons) {
    result = false;
  } else {
    ++statistics.functionCalls;

    // TODO: custom ordering
    for (int i = 0; i < poset.size() && !result; ++i) {
      for (int j = i + 1; j < poset.size() && !result; ++j) {
        if (poset.is(i, j) || poset.is(j, i)) {
          continue;
        }
        const bool result1 = search(poset.add_relation(i, j), cache_maximumReeched, cache_solution, maxComparisons,
                                    statistics, comparisonsDone + 1);
        if (result1) {
          const bool result2 = search(poset.add_relation(j, i), cache_maximumReeched, cache_solution, maxComparisons,
                                      statistics, comparisonsDone + 1);
          if (result2) {
            result = true;
          }
        }
      }
    }
  }

  if (result == false) {
    if (cache_maximumReeched.find(poset) == cache_maximumReeched.end() ||
        cache_maximumReeched[poset] < maxComparisons - comparisonsDone) {
      cache_maximumReeched[poset] = maxComparisons - comparisonsDone;
    }
  } else {
    if (cache_solution.find(poset) == cache_solution.end() || comparisonsDone < cache_solution[poset]) {
      cache_solution[poset] = comparisonsDone;
    }
  }
  return result;
}

template <size_t maxN>
std::optional<int> startSearch(const int n, const int nthSmallest,
                               std::unordered_map<Poset<maxN>, int> &cache_maximumReeched,
                               std::unordered_map<Poset<maxN>, int> &cache_solution, Statistics &statistics) {
  if (0 == nthSmallest) {
    return n - 1;
  }

  for (int i = n - 1; i < n * n; ++i) {
    std::cout << "\rtry: maxComparisons = " << i << std::flush;
    int comparisonsDone = 0;
    Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
    if (2 <= i) {
      if constexpr (USE_PAIR_MODE) {
        for (int k = 0; k < n - 1 && comparisonsDone < i; k += 2) {
          ++comparisonsDone;
          poset.addComparison(k, k + 1);
        }
      } else {
        ++comparisonsDone;
        poset.addComparison(0, 1);
      }
      poset.normalize();
    }
    cache_maximumReeched.clear();  // TODO: !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    cache_solution.clear();        // TODO: !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    const bool is_possible = search(poset, cache_maximumReeched, cache_solution, i, statistics, comparisonsDone);
    if (is_possible) {
      return i;
    }
  }
  return {};
}

template <size_t maxN>
struct std::hash<std::pair<Poset<maxN>, int>> {
  size_t operator()(const pair<Poset<maxN>, int> &pair) const {
    return hash<Poset<maxN>>{}(get<0>(pair)) ^ hash<int>{}(get<1>(pair));
  }
};

int main() {
  constexpr size_t maxN = 15;
#ifdef USE_NAUTY
  initNauty(maxN);
#endif

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);
  for (int n = 1; n < maxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      StopWatch watch{};

      Statistics statistics;
      std::unordered_map<Poset<maxN>, int> cache_maximumReeched;
      std::unordered_map<Poset<maxN>, int> cache_solution;
      const std::optional<int> comparisons =
          startSearch(n, nthSmallest, cache_maximumReeched, cache_solution, statistics);

      if (comparisons.has_value()) {
        if (n >= 6)
          std::cout << "\rtime '" << watch << "': n = " << n << ", i = " << nthSmallest << ", " << statistics
                    << ", entries = " << cache_maximumReeched.size() + cache_solution.size()
                    << ", comparisons: " << comparisons.value() << std::endl;
        if (comparisons != min_n_comparisons[n][nthSmallest]) {
          std::cerr << "Error, got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
                    << std::endl;
          exit(0);
        }
      } else {
        std::cerr << "Error, maxComparisons exceeded" << std::endl;
        exit(0);
      }
    }
    if (n >= 6) std::cout << std::endl;
  }
}