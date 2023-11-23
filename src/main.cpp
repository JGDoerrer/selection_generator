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
// returns pair<bool, int>
//  bool: true: in second steht solution, false: bis Tiefe x erfolglos gesucht
//  int: Lösung oder max. Suchtiefe
// States: found Solution with depth x / not found Soltion
optional<int> search(const Poset<maxN> &poset, unordered_set<Poset<maxN>> &cache1,
                     unordered_map<Poset<maxN>, pair<int, int>> &cache2, const int maxComparisons,
                     Statistics &statistics, const int depth = 0) {
  optional<int> result = nullopt;

  Poset<maxN> temp = poset;
  temp.comparisonsDone = 0;
  if (cache1.find(poset) != cache1.end()) {
    // if (cache[poset] != nullopt && depth < cache[poset].value() && poset.canDetermineNSmallest()) {
    // }

    // if ((nullopt != item) || (nullopt == item && poset.getMaxComparisonsDone() == maxComparisons)) {
    // }
    ++statistics.hashMatches;
    result = nullopt;
  } else if (cache2.find(temp) != cache2.end() && cache2[temp].second >= poset.comparisonsDone) {
    ++statistics.hashMatches;
    result = cache2[temp].first;
  } else if (poset.canDetermineNSmallest()) {
    result = depth;
  } else if (depth == maxComparisons) {
    result = nullopt;
  } else {
    ++statistics.functionCalls;

    for (int i = 0; i < poset.size(); ++i) {
      for (int j = i + 1; j < poset.size(); ++j) {
        if (poset.is(i, j) || poset.is(j, i)) {  // it holds arr[i] < arr[j]
          continue;
        }
        Poset<maxN> posetLess = poset.createNormPosetWithComp(i, j);  // a < b
        optional<int> result1 = search(posetLess, cache1, cache2, maxComparisons, statistics, depth + 1);
        if (result1.has_value()) {
          Poset<maxN> posetGreater = poset.createNormPosetWithComp(j, i);  // eig. a >= b?
          optional<int> result2 = search(posetGreater, cache1, cache2, maxComparisons, statistics, depth + 1);
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

  // int old1 = posetLess.comparisonsDone;
  // for (int kk = 0; kk < 15; ++kk) {
  //   if (old1 == kk) continue;
  //   posetLess.comparisonsDone = kk;
  //   if (cache.find(posetLess) != cache.end()) {
  //     cout << "\ncache size: " << cache.size() << endl;
  //     std::cout << "found error at k = " << kk << endl;
  //     if (!cache[posetLess].has_value()) {
  //       cout << "nullopt" << endl;
  //     } else {
  //       cout << cache[posetLess].value() << endl;
  //     }

  //     posetLess.comparisonsDone = old1;
  //     cout << posetLess << endl;
  //     exit(0);
  //   }
  // }
  // posetLess.comparisonsDone = old1;
  // cache[posetLess] = result1;

  // if (cache.find(posetLess) != cache.end()) {
  //   const optional<int> item = cache[posetLess];
  //   if ((item.has_value() && result1.has_value() && item.value() < result1.value()) ||
  //       (item.has_value() && !result1.has_value())) {
  //     result1 = item;
  //     cout << "kann das überhaupt eintreten?" << endl;
  //   }
  // }
// Todo: nauty c
  if (result == nullopt) {
    cache1.insert(poset);
  } else {
    Poset<maxN> temp = poset;
    temp.comparisonsDone = 0;
    cache2[temp] = make_pair(result.value(), poset.comparisonsDone);
  }
  return result;
}

template <size_t maxN>
optional<int> startSearch(const int n, const int nthSmallest, unordered_set<Poset<maxN>> &cache1,
                          unordered_map<Poset<maxN>, pair<int, int>> &cache2, Statistics &statistics) {
  if (0 == nthSmallest) {
    return n - 1;
  }

  // could start from i = n - 1
  for (int i = 0; i < n * n; ++i) {
    cout << "\rtry: maxComparisons = " << i << flush;
    Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest), uint8_t(i)};
    if (2 <= i) {
      // poset.addComparison(0, 1);
      // for (int k = 0; k < n - 1 && poset.comparisonsDone <= i; k += 2) {
      //   poset.addComparison(k, k + 1);
      // }
      poset.normalize();
    }
    cache1.clear();  // TODDDDDDDDDDDDDDDDDDDDDDDDDDDDOOOOOOOOOOOOOOOOOOOOO
    cache2.clear();  // TODDDDDDDDDDDDDDDDDDDDDDDDDDDDOOOOOOOOOOOOOOOOOOOOO
    const optional<int> comparisons = search(poset, cache1, cache2, i, statistics, poset.comparisonsDone);
    if (comparisons.has_value()) {
      return comparisons.value();
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
      // unordered_map<pair<Poset<maxN>, int>, optional<int>> cache2;
      unordered_set<Poset<maxN>> cache1;
      unordered_map<Poset<maxN>, pair<int, int>> cache2;
      const optional<int> comparisons = startSearch(n, nthSmallest, cache1, cache2, statistics);

      if (comparisons.has_value()) {
        cout << "\rtime '" << watch << "': n = " << n << ", i = " << nthSmallest
             << ", calls = " << statistics.functionCalls << ", hits = " << statistics.hashMatches
             << ", entries = " << cache1.size() + cache2.size() << ", comparisons: " << comparisons.value() << endl;
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

template <size_t maxN>
struct std::hash<pair<Poset<maxN>, int>> {
  size_t operator()(const pair<Poset<maxN>, int> &pair) const { return get<0>(pair).hash() ^ get<1>(pair).hash(); }
};

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