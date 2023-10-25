// g++ -Ddev -Wall -Wextra -Wconversion -Wno-unknown-pragmas
// -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
// -O2 -std=c++17 -g ./template.cpp -o program.out; ./program.out

#include <bits/stdc++.h>

#include "util.h"
using namespace std;

#include "poset.h"

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

class Statistics {
 public:
  int functionCalls = 0;
  int hashMatches = 0;
};

optional<int> search(const Poset &poset,
                     unordered_map<Poset, optional<int>> &cache,
                     const int maxComparisons, const int comparisonCounter,
                     Statistics &statistics) {
  optional<int> result = nullopt;

  ++statistics.functionCalls;
  // if (cache.find(poset) != cache.end()) {
  //   ++statistics.hashMatches;
  //   return cache[poset];
  // }

  if (poset.canDetermineNSmallest()) {
    // cache[poset] =
    result = comparisonCounter;
    return result;
  }

  if (0 < maxComparisons) {
    for (int i = 0; i < poset.size(); ++i) {
      for (int j = i + 1; j < poset.size(); ++j) {
        if (poset.is(i, j)) {  // it holds arr[i] < arr[j]
          continue;
        }
        Poset posetLess = poset;
        posetLess.addComparison(i, j);  // a < b
        const optional<int> result1 =
            search(posetLess, cache, maxComparisons - 1, comparisonCounter + 1,
                   statistics);
        if (result1.has_value()) {
          Poset posetGreater = poset;
          posetGreater.addComparison(j, i);  // eig. a >= b?
          const optional<int> result2 =
              search(posetGreater, cache, maxComparisons - 1,
                     comparisonCounter + 1, statistics);
          if (result2.has_value()) {
            if (result.has_value()) {
              result = min(result, max(result1, result2));
            } else {
              result = max(result1, result2);
            }
          }
        }
      }
    }
  }

  // // intentionally don't set cache for nullopt
  // if (result != nullopt) {
  //   cache[poset] = result;
  // }
  return result;
}

optional<int> startSearch(unordered_map<Poset, optional<int>> &cache,
                          const int n, const int nthSmallest,
                          const int maxComparisons, Statistics &statistics) {
  if (0 == nthSmallest) {
    return n - 1;
  }

  // could start from i = n - 1
  for (int i = 0; i < maxComparisons; ++i) {
    cout << "\rtry: maxComparisons = " << i << flush;
    Poset poset{n, nthSmallest};
    int count = 0;
    for (int i = 0; i < n - 1; i += 2) {
      ++count;
      poset.addComparison(i, i + 1);  // WIR BRACUHEN HIER <=
    }
    const optional<int> comparisons = search(poset, cache, i, 0, statistics);
    if (comparisons.has_value()) {
      return comparisons.value() + count;
    }
  }
  return {};
}

int main() {
  cout.setf(ios::fixed, ios::floatfield);
  cout.precision(3);
  for (int n = 1; n < maxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      StopWatch watch{};

      unordered_map<Poset, optional<int>> cache;
      Statistics statistics;
      const optional<int> comparisons =
          startSearch(cache, n, nthSmallest, globalMaxComparisons, statistics);

      if (comparisons.has_value()) {
        cout << "\rtime '" << watch << "': n = " << n << ", i = " << nthSmallest
             << ", calls = " << statistics.functionCalls
             << ", hits = " << statistics.hashMatches
             << ", entries = " << cache.size()
             << ", comparisons: " << comparisons.value() << endl;
        if (comparisons != min_n_comparisons[n][nthSmallest]) {
          cerr << "Error, got " << comparisons.value() << ", but expected "
               << min_n_comparisons[n][nthSmallest] << endl;
          exit(0);
        }
      } else {
        cerr << "Error, maxComparisons exceeded" << endl;
        exit(0);
      }
    }
    cout << endl;
  }
}

// n = 5, i = 0: 1 rec calls, comparisons: 0 == 4
// n = 5, i = 1: 3104699 rec calls, comparisons: 6 == 6
// n = 5, i = 2: 4804392 rec calls, comparisons: 6 == 6