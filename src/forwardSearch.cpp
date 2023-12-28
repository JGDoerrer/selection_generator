#include "main.h"

constexpr bool SORT_DFS_BRANCHES = true;
constexpr bool TOP_TO_BOTTOM_SEARCH = true;

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

template <size_t maxN>
Poset<maxN> createPosetWithComparison(Normalizer<maxN> &normalizer, Poset<maxN> poset, const uint16_t i,
                                      const uint16_t j) {
  poset.add_less(i, j);
  poset.normalize(normalizer);
  return poset;
};

/// @return true, wenn Median in poset in max. `maxComparisons` gefunden werden kann
template <size_t maxN, size_t maxC>
SearchResult searchRecursive(BS::thread_pool_light &threadpool, const Poset<maxN> &poset, Cache<maxN, maxC> &cache,
                             const uint8_t remainingComparisons, const uint8_t multiThreadingLevel,
                             const std::atomic<bool> &atomicBreak, Statistics &statistics, const int depth,
                             Normalizer<maxN> &normalizer) {
  SearchResult result = NoSolution;
  if (atomicBreak) {
    return Unknown;
  } else if (cache.check_not_solvable(poset, remainingComparisons)) {
    ++statistics.hashMatchLowerBound;
    return NoSolution;
  } else if (cache.check_solvable(poset, remainingComparisons)) {
    ++statistics.hashMatchUpperBound;
    return FoundSolution;
    // durch normalisierung können alle posets auf n == 1 reduziert werden, d.h. is_solvable unnötig
    // } else if (poset.is_solvable()) {
    //   result = FoundSolution;
  } else if (poset.is_not_solvable_in(remainingComparisons)) {
    result = NoSolution;
    ++statistics.noSolution;
  } else {
    ++statistics.bruteForce;

    const auto recursiveSearch = [&](const std::atomic<bool> &breakCondition, const int i, const int j,
                                     Normalizer<maxN> &normalizer) {
      SearchResult searchResult = searchRecursive(threadpool, createPosetWithComparison(normalizer, poset, i, j), cache,
                                                  remainingComparisons - 1, multiThreadingLevel, breakCondition,
                                                  statistics, depth + 1, normalizer);
      if (searchResult == FoundSolution) {
        searchResult = searchRecursive(threadpool, createPosetWithComparison(normalizer, poset, j, i), cache,
                                       remainingComparisons - 1, multiThreadingLevel, breakCondition, statistics,
                                       depth + 1, normalizer);
      }
      return searchResult;
    };

    if (remainingComparisons == multiThreadingLevel) {
      std::atomic<bool> breakCondition(false);

      for (int i = 0; i < poset.size(); ++i) {
        for (int j = i + 1; j < poset.size(); ++j) {
          if (!poset.is_less(i, j) && !poset.is_less(j, i)) {
            threadpool.push_task([&]() {
              Normalizer<maxN> new_normalizer{};
              if (FoundSolution == recursiveSearch(breakCondition, i, j, new_normalizer)) {
                breakCondition = true;
              }
            });
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
        uint8_t less[poset.size()];
        uint8_t greater[poset.size()];
        poset.calculate_relations(less, greater);

        const auto cmp = [&](const std::pair<int, int> &a, const std::pair<int, int> &b) {
          return greater[b.first] + less[b.second] < greater[a.first] + less[a.second];
        };

        std::vector<std::pair<int, int>> temp;
        for (int i = 0; i < poset.size(); ++i) {
          for (int j = i + 1; j < poset.size(); ++j) {
            if (!poset.is_less(i, j) && !poset.is_less(j, i)) {
              // Soll zuerst i<j oder j<i vergleicht werden? -> langsamer
              if (cmp({j, i}, {i, j})) {
                temp.push_back({i, j});
              } else {
                temp.push_back({j, i});
              }
            }
          }
        }

        std::sort(temp.begin(), temp.end(), cmp);

        for (const auto &[i, j] : temp) {
          result = recursiveSearch(atomicBreak, i, j, normalizer);
          if (result == FoundSolution) {
            break;
          }
        }
      } else {
        for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
          for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
            const auto [new_i, new_j] = randomDataTable[poset.size()][remainingComparisons][i][j];
            if (!poset.is_less(new_i, new_j) && !poset.is_less(new_j, new_i)) {
              result = recursiveSearch(atomicBreak, new_i, new_j, normalizer);
            }
          }
        }
      }
    }
  }

  if (result == NoSolution) {
    cache.insert_not_solvable(poset, remainingComparisons);
  } else if (result == FoundSolution) {
    cache.insert_solvable(poset, remainingComparisons);
  }
  return result;
}

template <size_t maxN, size_t maxC>
SearchResult startSearchNow(std::ostream &os, BS::thread_pool_light &threadpool, const int n, const int nthSmallest,
                            Cache<maxN, maxC> &cache, Statistics &statistics, const bool pairWiseOptimisation,
                            int maxComparisons, std::chrono::nanoseconds &time) {
  const std::chrono::time_point start = std::chrono::high_resolution_clock::now();
  const uint8_t multiLevel = 0;
  std::atomic<bool> atomicBreak(false);
  Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
  int comparisonsDone = 0;

  if (pairWiseOptimisation) {
    os << "# search with Pair-Optimisation & maxComparisons = " << maxComparisons << std::flush;
    for (int k = 0; k < n - 1 && comparisonsDone < maxComparisons; k += 2) {
      ++comparisonsDone;
      poset.add_less(k, k + 1);
    }
  } else {
    os << "# search with maxComparisons = " << maxComparisons << std::flush;
    ++comparisonsDone;
    poset.add_less(0, 1);
  }
  // ACHTUNG: hier könnte reduce_n falsch sein!
  Normalizer<maxN> normalizer{};
  poset.normalize(normalizer);

  const SearchResult result = searchRecursive(threadpool, poset, cache, maxComparisons - comparisonsDone, multiLevel,
                                              atomicBreak, statistics, comparisonsDone, normalizer);
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

template <size_t maxN, size_t maxC>
const std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearch(
    std::ostream &os, BS::thread_pool_light &threadpool, const int n, const int nthSmallest, Cache<maxN, maxC> &cache,
    Statistics &statistics, const bool topToBottom) {
  std::chrono::nanoseconds searchDuration{}, validateDuration{};
  const int lower = lower_bound(n, nthSmallest);
  const int upper = upper_bound(n, nthSmallest);
  if (lower == upper) {
    return {lower, searchDuration, validateDuration};
  }
  assert(2 < n);

  if (topToBottom) {
    // searchRecursive from top
    for (int i = upper - 1; i >= lower; --i) {
      if (startSearchNow(os, threadpool, n, nthSmallest, cache, statistics, true, i, searchDuration) == NoSolution) {
        if (startSearchNow(os, threadpool, n, nthSmallest, cache, statistics, false, i, validateDuration) ==
            NoSolution) {
          return {i + 1, searchDuration, validateDuration};
        }
      }
    }

    return {lower, searchDuration, validateDuration};
  } else {
    // searchRecursive from bottom
    for (int i = lower; i < upper; ++i) {
      if (startSearchNow(os, threadpool, n, nthSmallest, cache, statistics, true, i, searchDuration) == FoundSolution) {
        return {i, searchDuration, validateDuration};
      }

      if (startSearchNow(os, threadpool, n, nthSmallest, cache, statistics, false, i, validateDuration) ==
          FoundSolution) {
        return {i, searchDuration, validateDuration};
      }
    }

    return {upper, searchDuration, validateDuration};
  }
}

using namespace std::chrono;

int main() {
  constexpr size_t nBound = 9;

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  BS::thread_pool_light threadpool(threadCount);
  Cache<globalMaxN, globalMaxComparisons> cache;
  cache.insert_solvable(Poset<globalMaxN>(1, 0), 0);

  for (int n = 1; n < globalMaxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      Statistics statistics;
      const auto &[comparisons, durationSearch, durationValidate] =
          startSearch(std::cout, threadpool, n, nthSmallest, cache, statistics, TOP_TO_BOTTOM_SEARCH);

      // auto result1 = std::async(std::launch::async, [&]() {
      //   return startSearch(std::cout, threadpool, n, nthSmallest, cache, statistics, true);
      // });
      // auto result2 = std::async(std::launch::async, [&]() {
      //   Statistics statistics2;
      //   return startSearch(std::cout, threadpool, n, nthSmallest, cache, statistics2, false);
      // });
      // while (!result1.valid() && !result2.valid()) {
      //   result1.wait_for(100ms);
      //   result2.wait_for(100ms);
      // }
      // const auto &[comparisons, durationSearch1, durationValidate1] = result1.get();
      // const auto &[comparisons2, durationSearch2, durationValidate2] = result2.get();
      // assert(comparisons == comparisons2);
      // auto durationSearch = (durationSearch1 + durationValidate1);
      // auto durationValidate = (durationSearch2 + durationValidate2);

      // StopWatch watch;
      // cache.clean();
      // std::cout << watch << std::endl;

      if (comparisons.has_value()) {
        if (n >= nBound) {
          std::cout << "\rtime '" << durationSearch << " + " << durationValidate << " = "
                    << durationSearch + durationValidate << "': n = " << n << ", i = " << nthSmallest << ", "
                    << statistics << ", cache = " << cache.size() << ", comparisons: " << comparisons.value()
                    << std::endl;
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