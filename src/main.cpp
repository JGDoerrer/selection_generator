#include <bits/stdc++.h>

#include "BS_thread_pool.hpp"
#include "cache.h"
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
constexpr bool SORT_DFS_BRANCHES = true;
constexpr bool TOP_TO_BOTTOM_SEARCH = false;

// siehe Section 5.3.3
// https://doc.lagout.org/science/0_Computer%20Science/2_Algorithms/The%20Art%20of%20Computer%20Programming%20%28vol.%203_%20Sorting%20and%20Searching%29%20%282nd%20ed.%29%20%5BKnuth%201998-05-04%5D.pdf
int getUpperBound(const int n, int t) {
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

int getLowerBound(const int n, int t) {
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

const auto randomDataTable = ([]() {
  std::array<std::array<std::array<std::array<std::pair<int, int>, globalMaxN>, globalMaxN>, globalMaxComparisons>,
             globalMaxN>
      randomDataTable;
  auto rng = std::default_random_engine{1234};

  for (int n = 0; n < globalMaxN; ++n) {
    for (int comparison = 0; comparison < globalMaxComparisons; ++comparison) {
      std::vector<std::pair<int, int>> items;
      for (int i = 0; i < n; ++i) {
        for (int j = i + 1; j < n; ++j) {
          items.push_back({i, j});
        }
      }
      std::shuffle(items.begin(), items.end(), rng);
      int pos = 0;
      for (int i = 0; i < n; ++i) {
        for (int j = i + 1; j < n; ++j) {
          randomDataTable[n][comparison][i][j] = items[pos++];
        }
      }
    }
  }
  return randomDataTable;
})();

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
    /* n=12 */ {11, 14, 17, 18, 19, 21, 21, 19, 18, 17, 14, 11}};

std::array<Normalizer<globalMaxN>, 230> norm;  // TODO: only Debug

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

template <size_t maxN>
Poset<maxN> createPosetWithComparison(const int normalizerIndex, Poset<maxN> poset, const uint16_t i,
                                      const uint16_t j) {
  poset.addComparison(i, j);
  norm[normalizerIndex].normalize(poset);
  return poset;
};

/// @param cache_lowerBound enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
///                         werden kann; z.B. wenn cache_lowerBound[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
///                         um Poset zu lösen
/// @param cache_upperBound enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
///                         cache_upperBound[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
/// @return true, wenn Median in poset in max. `maxComparisons` gefunden werden kann
template <size_t maxN>
SearchResult searchRecursive(BS::thread_pool_light &threadpool, const Poset<maxN> &poset,
                             Cache<Poset<maxN>, uint8_t> cache_lowerBound[globalMaxN],
                             Cache<Poset<maxN>, uint8_t> cache_upperBound[globalMaxN],
                             const uint8_t remainingComparisons, const uint8_t multiThreadingLevel,
                             const std::atomic<bool> &atomicBreak, Statistics &statistics, const int depth,
                             const int normalizerIndex = 229) {
  SearchResult result = NoSolution;
  if (atomicBreak) {
    return Unknown;
  } else if (cache_lowerBound[poset.size()].checkCondition(
                 poset, [=](const uint8_t item) { return remainingComparisons <= item; })) {
    ++statistics.hashMatchLowerBound;
    return NoSolution;
  } else if (cache_upperBound[poset.size()].checkCondition(
                 poset, [=](const uint8_t item) { return remainingComparisons >= item; })) {
    ++statistics.hashMatchUpperBound;
    return FoundSolution;
    // durch normalisierung können alle posets auf n == 1 reduziert werden, d.h. canDetermineNSmallest unnötig
    // } else if (poset.canDetermineNSmallest()) {
    //   result = FoundSolution;
  } else if (!poset.hasEnoughComparisons(remainingComparisons)) {
    result = NoSolution;
    ++statistics.noSolution;
  } else {
    ++statistics.bruteForce;

    const auto recursiveSearch = [&](const std::atomic<bool> &breakCondition, const int i, const int j,
                                     const int normalizerIndex1) {
      SearchResult searchResult = searchRecursive(
          threadpool, createPosetWithComparison(normalizerIndex1, poset, i, j), cache_lowerBound, cache_upperBound,
          remainingComparisons - 1, multiThreadingLevel, breakCondition, statistics, depth + 1, normalizerIndex1);
      if (searchResult == FoundSolution) {
        searchResult = searchRecursive(threadpool, createPosetWithComparison(normalizerIndex1, poset, j, i),
                                       cache_lowerBound, cache_upperBound, remainingComparisons - 1,
                                       multiThreadingLevel, breakCondition, statistics, depth + 1, normalizerIndex1);
      }
      return searchResult;
    };

    if (remainingComparisons == multiThreadingLevel) {
      std::atomic<bool> breakCondition(false);

      int normalizerIndex1 = 0;
      for (int i = 0; i < poset.size(); ++i) {
        for (int j = i + 1; j < poset.size(); ++j) {
          if (!poset.is(i, j) && !poset.is(j, i)) {
            threadpool.push_task([&]() {
              if (FoundSolution == recursiveSearch(breakCondition, i, j, normalizerIndex1)) {
                breakCondition = true;
              }
            });
            ++normalizerIndex1;
          }
        }
      }
      // while (!threadpool.isReady()) {
      //   if (atomicBreak) {
      //     breakCondition = true;
      //   }
      // }
      threadpool.wait_for_tasks();

      if (breakCondition) {
        result = FoundSolution;
      }
    } else {
      if constexpr (SORT_DFS_BRANCHES) {
        std::vector<std::pair<int, int>> temp;
        for (int i = 0; i < poset.size(); ++i) {
          for (int j = i + 1; j < poset.size(); ++j) {
            if (!poset.is(i, j) && !poset.is(j, i)) {
              temp.push_back({i, j});
            }
          }
        }

        uint8_t less[poset.size()], greater[poset.size()];
        std::memset(less, 0, poset.size());
        std::memset(greater, 0, poset.size());
        for (uint8_t i = 0; i < poset.size(); ++i) {
          for (uint8_t j = 0; j < poset.size(); ++j) {
            if (poset.is(i, j)) {
              ++less[j];
              ++greater[i];
            }
          }
        }

        std::sort(temp.begin(), temp.end(), [&](const std::pair<int, int> &a, const std::pair<int, int> &b) {
          return greater[a.first] + less[a.second] > greater[b.first] + less[b.second];
        });

        for (const auto &[i, j] : temp) {
          result = recursiveSearch(atomicBreak, i, j, normalizerIndex);
          if (result == FoundSolution) {
            break;
          }
        }
      } else {
        for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
          for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
            const auto [new_i, new_j] = randomDataTable[poset.size()][remainingComparisons][i][j];
            if (!poset.is(new_i, new_j) && !poset.is(new_j, new_i)) {
              result = recursiveSearch(atomicBreak, new_i, new_j, normalizerIndex);
            }
          }
        }
      }
    }
  }

  if (result == NoSolution) {
    cache_lowerBound[poset.size()].insertIfCondition(poset, remainingComparisons,
                                                     [=](const uint8_t item) { return remainingComparisons > item; });
  } else if (result == FoundSolution) {
    cache_upperBound[poset.size()].insertIfCondition(poset, remainingComparisons,
                                                     [=](const uint8_t item) { return remainingComparisons < item; });
  }
  return result;
}

template <size_t maxN>
SearchResult startSearchNow(std::ostream &os, BS::thread_pool_light &threadpool, const int n, const int nthSmallest,
                            Cache<Poset<maxN>, uint8_t> cache_lowerBound[globalMaxN],
                            Cache<Poset<maxN>, uint8_t> cache_upperBound[globalMaxN], Statistics &statistics,
                            const bool pairWiseOptimisation, int maxComparisons, std::chrono::nanoseconds &time) {
  const std::chrono::time_point start = std::chrono::high_resolution_clock::now();
  const uint8_t multiLevel = 0;
  std::atomic<bool> atomicBreak(false);
  Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
  int comparisonsDone = 0;

  if (pairWiseOptimisation) {
    os << "# search with Pair-Optimisation & maxComparisons = " << maxComparisons << std::flush;
    for (int k = 0; k < n - 1 && comparisonsDone < maxComparisons; k += 2) {
      ++comparisonsDone;
      poset.addComparison(k, k + 1);
    }
  } else {
    os << "# search with maxComparisons = " << maxComparisons << std::flush;
    ++comparisonsDone;
    poset.addComparison(0, 1);
  }
  norm[0].normalize(poset);

  const SearchResult result =
      searchRecursive(threadpool, poset, cache_lowerBound, cache_upperBound, maxComparisons - comparisonsDone,
                      multiLevel, atomicBreak, statistics, comparisonsDone);
  if (result == FoundSolution) {
    os << " -> FoundSolution";
  } else if (result == NoSolution) {
    os << " -> NoSolution";
  }
  const std::chrono::time_point end = std::chrono::high_resolution_clock::now();
  os << " in " << end - start << std::endl;
  time += end - start;
  return result;
}

template <size_t maxN>
const std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearch(
    std::ostream &os, BS::thread_pool_light &threadpool, const int n, const int nthSmallest,
    Cache<Poset<maxN>, uint8_t> cache_lowerBound[globalMaxN], Cache<Poset<maxN>, uint8_t> cache_upperBound[globalMaxN],
    Statistics &statistics) {
  std::chrono::nanoseconds searchDuration{}, validateDuration{};
  const int lower = getLowerBound(n, nthSmallest);
  const int upper = getUpperBound(n, nthSmallest);
  if (lower == upper) {
    return {lower, searchDuration, validateDuration};
  }
  assert(2 < n);

  if constexpr (TOP_TO_BOTTOM_SEARCH) {
    // searchRecursive from top
    for (int i = upper - 1; i >= lower; --i) {
      if (startSearchNow(os, threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics, true, i,
                         searchDuration) == NoSolution) {
        if (startSearchNow(os, threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics, false, i,
                           validateDuration) == NoSolution) {
          return {i + 1, searchDuration, validateDuration};
        }
      }
    }

    return {lower, searchDuration, validateDuration};
  } else {
    // searchRecursive from bottom
    for (int i = lower; i < upper; ++i) {
      if (startSearchNow(os, threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics, true, i,
                         searchDuration) == FoundSolution) {
        return {i, searchDuration, validateDuration};
      }

      if (startSearchNow(os, threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics, false, i,
                         validateDuration) == FoundSolution) {
        return {i, searchDuration, validateDuration};
      }
    }

    return {upper, searchDuration, validateDuration};
  }
}

int main() {
  constexpr size_t nBound = 9;

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  BS::thread_pool_light threadpool(threadCount);
  Cache<Poset<globalMaxN>, uint8_t> cache_lowerBound[globalMaxN];
  Cache<Poset<globalMaxN>, uint8_t> cache_upperBound[globalMaxN];
  cache_upperBound[1].insert(Poset<globalMaxN>(1, 0), 0);

  for (int n = 1; n < globalMaxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      Statistics statistics;
      const auto &[comparisons, durationSearch, durationValidate] =
          startSearch(std::cout, threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics);

      int cache_upper_size = 0, cache_lower_size = 0;
      for (int i = 0; i < globalMaxN; ++i) {
        cache_upper_size += cache_upperBound[i].size();
        cache_lower_size += cache_lowerBound[i].size();
      }

      if (comparisons.has_value()) {
        if (n >= nBound) {
          std::cout << "\rtime '" << durationSearch << " + " << durationValidate << " = "
                    << durationSearch + durationValidate << "': n = " << n << ", i = " << nthSmallest << ", "
                    << statistics << ", cache = (" << cache_lower_size << " + " << cache_upper_size << " = "
                    << cache_upper_size + cache_lower_size << "), comparisons: " << comparisons.value() << std::endl;
        }
        if (comparisons != min_n_comparisons[n][nthSmallest]) {
          std::cerr << "Error: got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
                    << std::endl;
          exit(0);
        }
      } else {
        std::cerr << "Error, maxComparisons exceeded" << std::endl;
        exit(0);
      }
    }
    if (n >= nBound) std::cout << std::endl;
  }
}

// int main() {
//   for (int n = 0; n <= 15; ++n) {
//     for (int t = 0; t < (n + 1) / 2; ++t) {
//       const int lower = getLowerBound(n, t);
//       const int upper = getUpperBound(n, t);
//       if (lower < upper) {
//         std::cout << "n = " << n << ", t = " << t << ": ";
//         std::cout << lower << " - " << upper;
//         std::cout << std::endl;
//       } else {
//         std::cout << "n = " << n << ", t = " << t << ": ";
//         std::cout << lower;
//         std::cout << std::endl;
//       }
//     }
//   }
// }

// std::unordered_set<Poset<globalMaxN>> posets;
// int stat1 = 0;
// void rec(std::vector<bool> &temp, const int n, const int k, int pos) {
//   if (-1 == pos) {
//     Poset<globalMaxN> poset(n, k);

//     int p1 = 0;
//     for (int i = 0; i < n; ++i)
//       for (int j = i + 1; j < n; ++j)
//         if (temp[p1++]) {
//           poset.addComparison(i, j);
//         }

//     ++stat1;
//     if (!poset.canDetermineNSmallest()) {
//       for (int i = 0; i < n; ++i)
//         for (int j = i + 1; j < n; ++j)
//           if (!poset.is(i, j)) {
//             Poset<globalMaxN> poset2 = createPosetWithComparison(0, poset, i, j);
//             if (poset2.canDetermineNSmallest()) {
//               posets.insert(poset);
//             }
//           }
//     }
//   } else {
//     temp[pos] = false;
//     rec(temp, n, k, pos - 1);
//     temp[pos] = true;
//     rec(temp, n, k, pos - 1);
//   }
// }

// int main() {
//   int n = 4;
//   int k = 1;
//   std::vector<bool> temp(n * (n - 1) / 2, 0);
//   rec(temp, n, k, n * (n - 1) / 2);
//   std::cout << posets.size() << std::endl;
//   std::cout << stat1 << std::endl;
//   for (auto poset : posets) std::cout << poset << std::endl;
//