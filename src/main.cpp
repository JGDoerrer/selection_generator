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
SearchResult search(BS::thread_pool_light &threadpool, const Poset<maxN> &poset,
                    Cache<Poset<maxN>, uint8_t> cache_lowerBound[globalMaxN],
                    Cache<Poset<maxN>, uint8_t> cache_upperBound[globalMaxN], const uint8_t remainingComparisons,
                    const uint8_t multiThreadingLevel, const std::atomic<bool> &atomicBreak, Statistics &statistics,
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
      SearchResult searchResult = search(threadpool, createPosetWithComparison(normalizerIndex1, poset, i, j),
                                         cache_lowerBound, cache_upperBound, remainingComparisons - 1,
                                         multiThreadingLevel, breakCondition, statistics, normalizerIndex1);
      if (searchResult == FoundSolution) {
        searchResult = search(threadpool, createPosetWithComparison(normalizerIndex1, poset, j, i), cache_lowerBound,
                              cache_upperBound, remainingComparisons - 1, multiThreadingLevel, breakCondition,
                              statistics, normalizerIndex1);
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
      // std::vector<std::pair<int, int>> temp;
      // for (int i = 0; i < poset.size(); ++i) {
      //   for (int j = i + 1; j < poset.size(); ++j) {
      //     if (!poset.is(i, j) && !poset.is(j, i)) {
      //       temp.push_back({i, j});
      //     }
      //   }
      // }

      // uint8_t less[poset.size()], greater[poset.size()];
      // std::memset(less, 0, poset.size());
      // std::memset(greater, 0, poset.size());
      // for (uint8_t i = 0; i < poset.size(); ++i) {
      //   for (uint8_t j = 0; j < poset.size(); ++j) {
      //     if (poset.is(i, j)) {
      //       ++less[j];
      //       ++greater[i];
      //     }
      //   }
      // }

      // std::sort(temp.begin(), temp.end(), [&](const std::pair<int, int> &a, const std::pair<int, int> &b) {
      //   return greater[a.first] + less[a.second] > greater[b.first] + less[b.second];
      // });

      // for (const auto &[i, j] : temp) {
      //   result = recursiveSearch(atomicBreak, i, j, normalizerIndex);
      //   if (result == FoundSolution) {
      //     break;
      //   }
      // }

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
const std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearch(
    BS::thread_pool_light &threadpool, const int n, const int nthSmallest,
    Cache<Poset<maxN>, uint8_t> cache_lowerBound[globalMaxN], Cache<Poset<maxN>, uint8_t> cache_upperBound[globalMaxN],
    Statistics &statistics) {
  std::chrono::nanoseconds searchDuration{}, validateDuration{};
  if (0 == nthSmallest || n <= 2) {
    return {n - 1, searchDuration, validateDuration};
  }

  const uint8_t multiLevel = 0;

  int foundSolution;
  std::atomic<bool> atomicBreak(false);

  // TODO: search from top
  // n = 12;
  // nthSmallest = 3;
  // std::cout << "begin" << std::endl;
  // for (int i = 2 * n; i >= 0; --i) {
  //   std::cout << "\rtry: maxComparisons = " << i << "   " << std::flush;
  //   Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
  //   int comparisonsDone = 0;
  //   for (int k = 0; k < n - 1 && comparisonsDone < i; k += 2) {
  //     ++comparisonsDone;
  //     poset.addComparison(k, k + 1);
  //   }
  //   norm[0].normalize(poset);
  //   const SearchResult is_possible = search(threadpool, poset, cache_lowerBound, cache_upperBound, i -
  //   comparisonsDone,
  //                                           multiLevel, atomicBreak, statistics);
  // }
  // std::cout << "end" << std::endl;

  // search from bottom
  for (int i = n - 1; i < n * n; ++i) {
    {
      int upperBound = i;
      std::cout << "\rtry: upper comparisonBound = " << upperBound << "                                    "
                << std::flush;
      Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
      int comparisonsDone = 0;
      for (int k = 0; k < n - 1 && comparisonsDone < upperBound; k += 2) {
        ++comparisonsDone;
        poset.addComparison(k, k + 1);
      }
      norm[0].normalize(poset);
      std::chrono::time_point start = std::chrono::high_resolution_clock::now();
      const SearchResult is_possible = search(threadpool, poset, cache_lowerBound, cache_upperBound,
                                              upperBound - comparisonsDone, multiLevel, atomicBreak, statistics);
      searchDuration += std::chrono::high_resolution_clock::now() - start;
      if (is_possible == FoundSolution) {
        return {upperBound, searchDuration, validateDuration};
      }
    }

    {
      int foundSolution = i - 1;
      std::cout << "\rvalidate there is no solution with " << foundSolution << " comparisons           " << std::flush;
      Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
      int comparisonsDone = 1;
      poset.addComparison(0, 1);
      norm[0].normalize(poset);

      std::chrono::time_point start = std::chrono::high_resolution_clock::now();
      const SearchResult is_possible = search(threadpool, poset, cache_lowerBound, cache_upperBound,
                                              foundSolution - comparisonsDone, multiLevel, atomicBreak, statistics);
      validateDuration += std::chrono::high_resolution_clock::now() - start;
      if (is_possible == FoundSolution) {
        return {foundSolution, searchDuration, validateDuration};
      }
    }
  }

  std::cout << "critical error: found no solution" << std::endl;
  exit(0);
  return {};
}

// template <size_t maxN>
// SearchResult searchIterative(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
//   SearchResult result = NoSolution;
//   if (poset.canDetermineNSmallest()) {
//     result = FoundSolution;
//   } else if (0 != remainingComparisons) {
//     for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
//       for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
//         if (!poset.is(i, j) && !poset.is(j, i)) {
//           result = searchIterative(createPosetWithComparison(0, poset, i, j), remainingComparisons - 1);
//           if (result == FoundSolution) {
//             result = searchIterative(createPosetWithComparison(0, poset, j, i), remainingComparisons - 1);
//           }
//         }
//       }
//     }
//   }
//   return result;
// }

// template <size_t maxN>
// SearchResult searchIterative2(const Poset<maxN> &poset, const uint8_t remainingComparisons) {
//   std::vector<Poset<maxN>> solutions;

//   for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
//     for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
//       if (!poset.is(i, j) && !poset.is(j, i)) {
//         solutions.push_back(createPosetWithComparison(0, ));
//       }
//     }
//   }

//   SearchResult result = NoSolution;
//   if (poset.canDetermineNSmallest()) {
//     result = FoundSolution;
//   } else if (0 != remainingComparisons) {
//     for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
//       for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
//         if (!poset.is(i, j) && !poset.is(j, i)) {
//           solutions.push_back(poset);
//         }
//       }
//     }
//   }
//   return result;
// }

// template <size_t maxN>
// std::optional<int> startSearchIterative(const int n, const int nthSmallest) {
//   // Der Fall `0 == nthSmallest` ist bereits bekannt
//   if (0 == nthSmallest || n <= 2) {
//     return n - 1;
//   }

//   // `maxDepth` gibt die maximale Suchtiefe an. Diese wird, solange kein Ergebnis gefunden wurde, iterativ erhöht
//   for (int maxDepth = n - 1; maxDepth < n * n; ++maxDepth) {
//     int comparisonsDone = 0;
//     Poset<maxN> poset{n, nthSmallest};  // erstelle ein leeres Poset
//     for (int k = 0; k < n - 1 && comparisonsDone < maxDepth; k += 2) {
//       poset.addComparison(k, k + 1);  // bilde inital Paar und vergleiche diese
//       ++comparisonsDone;  // reduziere unsere maximale Suchtiefe, da bereits ein Vergleich durchgeführt wurde
//     }
//     // Suche, ob durch hinzufügen von maximal `maxDepth` Vergleichen, das Poset gelöst werden kann
//     if (FoundSolution == searchIterative(poset, maxDepth - comparisonsDone)) {
//       // Bis jetzt ist bekannt, dass mit dem "Paare-Trick" das i-kleinste Element in dem Poset in
//       `maxDepth`-Schritten
//       // eindeutig gelöst werden kann. Da der Trick mit den Paaren nicht bewiesen ist, führe anschließend noch eine
//       // normale Suche mit Tiefe `maxDepth - 1` durch. Wenn diese in `NoSolution` resultiert, ist die Lösung gefunden
//       // (anderenfalls hätten wir den "Paare-Trick" widerlegt)
//       Poset<maxN> poset{n, nthSmallest};
//       // da irgendwelche 2 Elemente am Anfang verglichen werden, wähle o.B.d.A `0` und `1`
//       int comparisonsDone = 1;
//       poset.addComparison(0, 1);

//       if (NoSolution == searchIterative(poset, maxDepth - comparisonsDone - 1)) {
//         return maxDepth;
//       }

//       std::cout << "Error: \"Paare-Trick\" widerlegt" << std::endl;
//       return {};
//     }
//   }
// }

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
      // const std::optional<int> comparisons = startSearchIterative<globalMaxN>(n, nthSmallest);
      const auto &[comparisons, durationSearch, durationValidate] =
          startSearch(threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics);

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