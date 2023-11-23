// g++ -Ddev -Wall -Wextra -Wconversion -Wno-unknown-pragmas
// -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
// -O2 -std=c++17 -g ./template.cpp -o program.out; ./program.out

#include <bits/stdc++.h>

#include "poset.h"
#include "util.h"
using namespace std;

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

template <size_t maxN>
optional<int> search(const Poset<maxN> &poset, unordered_map<Poset<maxN>, optional<int>> &cache,
                     const int maxComparisons, const int comparisonCounter, Statistics &statistics) {
  if (cache.find(poset) != cache.end()) {
    ++statistics.hashMatches;
    return cache[poset];
  } else if (poset.canDetermineNSmallest()) {
    return comparisonCounter;
  } else if (0 == maxComparisons) {
    return nullopt;
  }

  ++statistics.functionCalls;

  optional<int> result = nullopt;
  for (int i = 0; i < poset.size(); ++i) {
    for (int j = i + 1; j < poset.size(); ++j) {
      if (poset.is(i, j) || poset.is(j, i)) {  // it holds arr[i] < arr[j]
        continue;
      }
      Poset<maxN> posetLess = poset;
      posetLess.addComparison(i, j);  // a < b
      posetLess.normalize();
      const optional<int> result1 = search(posetLess, cache, maxComparisons - 1, comparisonCounter + 1, statistics);
      cache[posetLess] = result1;
      if (result1.has_value()) {
        Poset<maxN> posetGreater = poset;
        posetGreater.addComparison(j, i);  // eig. a >= b?
        posetGreater.normalize();
        const optional<int> result2 =
            search(posetGreater, cache, maxComparisons - 1, comparisonCounter + 1, statistics);
        cache[posetGreater] = result2;
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

  return result;
}

template <size_t maxN>
optional<int> startSearch(const int n, const int nthSmallest, unordered_map<Poset<maxN>, optional<int>> &cache,
                          Statistics &statistics) {
  if (0 == nthSmallest) {
    return n - 1;
  }

  // could start from i = n - 1
  for (int i = 0; i < n * n; ++i) {
    cout << "\rtry: maxComparisons = " << i << flush;
    Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
    int count = 0;
    // for (int i = 0; i < n - 1; i += 2) {
    //   ++count;
    //   poset.addComparison(i, i + 1);  // WIR BRACUHEN HIER <=
    // }
    cache.clear();  // TODDDDDDDDDDDDDDDDDDDDDDDDDDDDOOOOOOOOOOOOOOOOOOOOOoo
    const optional<int> comparisons = search(poset, cache, i, 0, statistics);
    cache[poset] = comparisons;
    if (comparisons.has_value()) {
      return comparisons.value() + count;
    }
  }
  return {};
}

int main() {
  constexpr size_t maxN = 8;

  cout.setf(ios::fixed, ios::floatfield);
  cout.precision(3);
  for (int n = 1; n < maxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      StopWatch watch{};

      Statistics statistics;
      unordered_map<Poset<maxN>, optional<int>> cache;
      const optional<int> comparisons = startSearch(n, nthSmallest, cache, statistics);

      if (comparisons.has_value()) {
        cout << "\rtime '" << watch << "': n = " << n << ", i = " << nthSmallest
             << ", calls = " << statistics.functionCalls << ", hits = " << statistics.hashMatches
             << ", entries = " << cache.size() << ", comparisons: " << comparisons.value() << endl;
        if (comparisons != min_n_comparisons[n][nthSmallest]) {
          cerr << "Error, got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
               << endl;
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

// int main() {
//   constexpr int maxN = 10;
//   int n = 6;
//   unordered_map<Poset<maxN>, optional<int>> cache;
//   for (int i0 = 0; i0 < n; ++i0) {
//     for (int j0 = 0; j0 < n; ++j0) {
//       for (int i1 = 0; i1 < n; ++i1) {
//         for (int j1 = 0; j1 < n; ++j1) {
//           for (int i2 = 0; i2 < n; ++i2) {
//             for (int j2 = 0; j2 < n; ++j2) {
//               Poset<maxN> poset{n, 2};
//               poset.addComparison(i0, j0);
//               if (!(poset.is(i1, j1) || poset.is(j1, i1))) {
//                 poset.addComparison(i1, j1);
//                 if (!(poset.is(i2, j2) || poset.is(j2, i2))) {
//                   poset.addComparison(i2, j2);
//                   poset.normalize();
//                   cache[poset] = 0;
//                 }
//               }
//             }
//           }
//         }
//       }
//     }
//   }
//   cout << cache.size() << endl;
// }