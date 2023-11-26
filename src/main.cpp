// #define USE_NAUTY

#include <bits/stdc++.h>

#include "BS_thread_pool.hpp"
#include "poset.h"
#include "util.h"

// TODO: Heuristiken:
//        kann Poset in x Schritten gelöst werden -> vorzeitiger Abbruch

// TODO: josua less test überarbeiten
// TODO: multithreading implementieren
// TODO: multithreading nauty
// TODO: create make file, compile option `march native`
// TODO: ACHTUNG: post_normalize kaputt

// TODO: swap Operationen, memcpy, fine-tuning
// TODO: überall explicit static cast

constexpr size_t globalMaxComparisons = 25;
constexpr size_t globalMaxN = 15;
std::array<std::array<std::array<std::array<std::pair<int, int>, globalMaxN>, globalMaxN>, globalMaxComparisons>,
           globalMaxN>
    randomDataTable;

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
  size_t functionCalls = 0;
  size_t hashMatches = 0;
};

std::ostream &operator<<(std::ostream &os, const Statistics &stats) {
  os << "calls = " << stats.functionCalls << ", hits = " << stats.hashMatches;
  return os;
}

enum SearchResult : uint8_t { FoundSolution, NoSolution, Unknown };

template <typename F, typename G>
inline bool checkMapThreadSafe(const F &poset, const std::unordered_map<F, G> &cache, std::mutex &mutex_cache,
                               const std::function<bool(const G)> &condition) {
  mutex_cache.lock();
  const bool resultValue = cache.find(poset) != cache.end() && condition(cache.at(poset));
  mutex_cache.unlock();
  return resultValue;
}

template <size_t maxN>
/// @param cache_lowerBound enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
///                         werden kann; z.B. wenn cache_lowerBound[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
///                         um Poset zu lösen
/// @param cache_upperBound enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
///                         cache_upperBound[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
/// @return true, wenn Median in poset in max. `maxComparisons` gefunden werden kann
SearchResult search(BS::thread_pool_light &threadpool, const Poset<maxN> &poset,
                    std::unordered_map<Poset<maxN>, int> &cache_lowerBound, std::mutex &mutex_cache_lowerBound,
                    std::unordered_map<Poset<maxN>, int> &cache_upperBound, std::mutex &mutex_cache_upperBound,
                    const int maxComparisons, const int maxMultiThreading, const std::atomic<bool> &atomicBreak,
                    Statistics &statistics, const int comparisonsDone) {
  SearchResult result = NoSolution;
  if (atomicBreak) {
    return Unknown;
  } else if (checkMapThreadSafe(poset, cache_lowerBound, mutex_cache_lowerBound,
                                std::function<bool(const int item)>(
                                    [=](const int item) { return maxComparisons - comparisonsDone <= item; }))) {
    ++statistics.hashMatches;
    return NoSolution;
  } else if (checkMapThreadSafe(poset, cache_upperBound, mutex_cache_upperBound,
                                std::function<bool(const int item)>(
                                    [=](const int item) { return maxComparisons - comparisonsDone >= item; }))) {
    ++statistics.hashMatches;
    return FoundSolution;
  } else if (poset.canDetermineNSmallest()) {
    result = FoundSolution;
  } else if (comparisonsDone == maxComparisons) {
    result = NoSolution;
  } else {
    ++statistics.functionCalls;

    const auto temp1 = [&](const std::atomic<bool> &breakCondition, const int i, const int j) {
      SearchResult searchResult = search(threadpool, poset.add_relation(i, j), cache_lowerBound, mutex_cache_lowerBound,
                                         cache_upperBound, mutex_cache_upperBound, maxComparisons, maxMultiThreading,
                                         breakCondition, statistics, comparisonsDone + 1);
      if (searchResult == FoundSolution) {
        searchResult = search(threadpool, poset.add_relation(j, i), cache_lowerBound, mutex_cache_lowerBound,
                              cache_upperBound, mutex_cache_upperBound, maxComparisons, maxMultiThreading,
                              breakCondition, statistics, comparisonsDone + 1);
      }
      return searchResult;
    };

    const auto temp2 = [&](std::atomic<bool> &breakCondition, const int i, const int j) {
      if (FoundSolution == temp1(breakCondition, i, j)) {
        breakCondition = true;
      }
    };

    if (comparisonsDone == maxMultiThreading) {
      std::atomic<bool> breakCondition(false);

      // std::vector<std::future<void>> futures;
      for (int i = 0; i < poset.size(); ++i) {
        for (int j = i + 1; j < poset.size(); ++j) {
          if (!poset.is(i, j) && !poset.is(j, i)) {
            // futures.push_back(std::async(std::launch::async, temp, std::ref(breakCondition), i, j));
            threadpool.push_task(temp2, std::ref(breakCondition), i, j);
          }
        }
      }
      // while (!threadpool.isReady()) {
      //   if (atomicBreak) {
      //     breakCondition = true;
      //   }
      // }
      // std::cout << std::endl << "futures.size(): " << futures.size() << std::endl;
      // for (auto &fut : futures) fut.wait();
      threadpool.wait_for_tasks();

      if (breakCondition) {
        result = FoundSolution;
      }
    } else {
      for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
        for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
          const auto [new_i, new_j] = randomDataTable[poset.size()][comparisonsDone][i][j];
          if (!poset.is(new_i, new_j) && !poset.is(new_j, new_i)) {
            result = temp1(atomicBreak, new_i, new_j);
          }
        }
      }
    }
  }

  if (result == FoundSolution) {
    mutex_cache_upperBound.lock();
    if (cache_upperBound.find(poset) == cache_upperBound.end() ||
        maxComparisons - comparisonsDone < cache_upperBound[poset]) {
      cache_upperBound[poset] = maxComparisons - comparisonsDone;
    }
    mutex_cache_upperBound.unlock();
  } else if (result == NoSolution) {
    mutex_cache_lowerBound.lock();
    if (cache_lowerBound.find(poset) == cache_lowerBound.end() ||
        maxComparisons - comparisonsDone > cache_lowerBound[poset]) {
      cache_lowerBound[poset] = maxComparisons - comparisonsDone;
    }
    mutex_cache_lowerBound.unlock();
  }
  return result;
}

template <size_t maxN>
std::optional<int> startSearch(BS::thread_pool_light &threadpool, const int n, const int nthSmallest,
                               std::unordered_map<Poset<maxN>, int> &cache_lowerBound,
                               std::unordered_map<Poset<maxN>, int> &cache_upperBound, Statistics &statistics) {
  if (0 == nthSmallest || n <= 2) {
    return n - 1;
  }

  int foundSolution;
  std::mutex mutex_cache_lowerBound;
  std::mutex mutex_cache_upperBound;
  std::atomic<bool> atomicBreak(false);

  for (int i = n - 1; i < n * n; ++i) {
    std::cout << "\rtry: maxComparisons = " << i << "   " << std::flush;
    Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
    int comparisonsDone = 0;
    for (int k = 0; k < n - 1 && comparisonsDone < i; k += 2) {
      ++comparisonsDone;
      poset.addComparison(k, k + 1);
    }
    poset.normalize();

    const SearchResult is_possible =
        search(threadpool, poset, cache_lowerBound, mutex_cache_lowerBound, cache_upperBound, mutex_cache_upperBound, i,
               0, atomicBreak, statistics, comparisonsDone);
    if (is_possible == FoundSolution) {
      foundSolution = i;
      break;
    }
  }

  std::cout << "\rvalidate solution: " << foundSolution << "      " << std::flush;
  Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
  int comparisonsDone = 1;
  poset.addComparison(0, 1);
  poset.normalize();

  const SearchResult is_possible =
      search(threadpool, poset, cache_lowerBound, mutex_cache_lowerBound, cache_upperBound, mutex_cache_upperBound,
             foundSolution - 1, 0, atomicBreak, statistics, comparisonsDone);
  if (is_possible == NoSolution) {
    return foundSolution;
  } else {
    std::cout << "found also solution with -1" << std::endl;
    exit(0);
  }

  return {};
}

int main() {
  constexpr size_t maxComparisons = globalMaxComparisons;
  constexpr size_t maxN = globalMaxN;
  constexpr size_t nBound = 8;
#ifdef USE_NAUTY
  initNauty(maxN);
#endif

  auto rng = std::default_random_engine{1234};
  for (int n = 0; n < maxN; ++n) {
    for (int comparison = 0; comparison < maxComparisons; ++comparison) {
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

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  BS::thread_pool_light threadpool;
  std::unordered_map<Poset<maxN>, int> cache_lowerBound, cache_upperBound;
  for (int n = 1; n < maxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      StopWatch watch{};

      Statistics statistics;
      const std::optional<int> comparisons =
          startSearch(threadpool, n, nthSmallest, cache_lowerBound, cache_upperBound, statistics);

      if (comparisons.has_value()) {
        if (n >= nBound)
          std::cout << "\rtime '" << watch << "': n = " << n << ", i = " << nthSmallest << ", " << statistics
                    << ", cache = (" << cache_upperBound.size() << " + "
                    << cache_lowerBound.size() << " = " << cache_upperBound.size() + cache_lowerBound.size()
                    << "), comparisons: " << comparisons.value() << std::endl;
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