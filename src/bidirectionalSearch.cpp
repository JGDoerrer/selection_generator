#include "main.h"

PosetCacheSet<globalMaxN, globalMaxComparisons> poset_cache;
std::atomic<int> dynLevel = 0;

constexpr bool TOP_TO_BOTTOM_SEARCH = true;

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
  if (remainingComparisons <= dynLevel) {
    return (poset_cache.check_solvable(poset, remainingComparisons)) ? FoundSolution : NoSolution;
  } else if (remainingComparisons <= dynLevel + 1) {
    if (poset_cache.check_solvable(poset, remainingComparisons)) {
      return FoundSolution;
    }
  }

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
      threadpool.wait_for_tasks();

      if (breakCondition) {
        result = FoundSolution;
      }
    } else {
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

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    PosetCacheSet<maxN, globalMaxComparisons> &poset_cache, const uint8_t n, const uint8_t nthSmallest,
    const std::atomic<bool> &breaker) {
  dynLevel = 0;
  Normalizer<maxN> normalizer{};
  std::chrono::nanoseconds duration_build_posets_total{}, duration_test_posets_total{};
  std::unordered_set<Poset<maxN>> source{{Poset<globalMaxN>(1, 0)}};
  for (int k = 1; k < n * n; ++k) {
    std::chrono::nanoseconds duration_build_posets{}, duration_test_posets{};
    const auto start = std::chrono::high_resolution_clock::now();
    const auto source_new = enlarge(normalizer, source, n, nthSmallest);
    const auto mid = std::chrono::high_resolution_clock::now();
    duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    std::unordered_set<Poset<maxN>> destination;
    for (const Poset<maxN> &item : source_new) {
      for (uint8_t i = 0; i < n; ++i) {
        for (uint8_t j = 0; j < n; ++j) {
          if (item.is_less(i, j)) {
            for (const Poset<maxN> &predecessor :
                 item.remove_less(normalizer, i, j, [&poset_cache, k](const Poset<maxN> &poset) {
                   return poset_cache.check_solvable(poset, k - 1);
                 })) {
              if (predecessor == Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest} || breaker) {
                duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
                duration_test_posets_total += duration_test_posets;
                std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in "
                          << duration_build_posets << " ~ " << duration_test_posets
                          << " | total cached: " << poset_cache.size() << " ";
                if (breaker) {
                  std::cout << "(breaked)";
                } else {
                  std::cout << "(found solution)";
                }
                std::cout << std::endl;
                return {k, duration_build_posets_total, duration_test_posets_total};
              }

              Poset<maxN> predecessor_normalized = predecessor;
              predecessor_normalized.normalize(normalizer);
              if (poset_cache.check_solvable(predecessor_normalized, k - 1)) {
                continue;
              }

              destination.insert(predecessor_normalized);
              poset_cache.insert_solvable(predecessor_normalized, k);
            }
          }
        }
      }
    }
    dynLevel = k;
    duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
    duration_test_posets_total += duration_test_posets;

    std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in " << duration_build_posets
              << " ~ " << duration_test_posets << " | total cached: " << poset_cache.size() << std::endl;

    source = destination;
  }

  return {std::nullopt, duration_build_posets_total, duration_test_posets_total};
}

using namespace std::chrono;

int main() {
  constexpr size_t nBound = 0;

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  BS::thread_pool_light threadpool(threadCount);
  Cache<globalMaxN, globalMaxComparisons> cache;
  cache.insert_solvable(Poset<globalMaxN>(1, 0), 0);

  poset_cache.insert_solvable(Poset<globalMaxN>(1, 0), 0);

  for (int n = 2; n < globalMaxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      Statistics statistics;

      std::atomic<bool> breaker = false;
      auto result1 = std::async(
          std::launch::async, [&]() { return startSearchBackward<globalMaxN>(poset_cache, n, nthSmallest, breaker); });
      auto result2 = std::async(std::launch::async, [&]() {
        auto resultX = startSearch(std::cout, threadpool, n, nthSmallest, cache, statistics, TOP_TO_BOTTOM_SEARCH);
        breaker = true;
        return resultX;
      });
      while (!result1.valid() && !result2.valid()) {
        result1.wait_for(100ms);
        result2.wait_for(100ms);
      }
      const auto &[compUseless, durationSearch1, durationValidate1] = result1.get();
      const auto &[comparisons, durationSearch2, durationValidate2] = result2.get();
      auto durationSearch = durationSearch2;
      auto durationValidate = durationValidate2;

      if (comparisons.has_value()) {
        if (n >= nBound) {
          std::cout << "time '" << durationSearch << " + " << durationValidate << " = "
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