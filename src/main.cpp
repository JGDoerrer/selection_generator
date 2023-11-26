// g++ -Ddev -Wall -Wextra -Wconversion -Wno-unknown-pragmas
// -Wmaybe-uninitialized -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG
// -O2 -std=c++17 -g ./template.cpp -o program.out; ./program.out
// #define USE_NAUTY

#include <bits/stdc++.h>

#include "BS_thread_pool.hpp"
#include "poset.h"
#include "util.h"

// TODO: Heuristiken:
//        kann Poset in x Schritten gelöst werden -> vorzeitiger Abbruch
//        custom ordering

// TODO: josua less test überarbeiten
// TODO: multithreading implementieren
// TODO: multithreading nauty
// TODO: create make file, compile option `march native`
// TODO: ACHTUNG: post_normalize kaputt

// TODO: swap Operationen, memcpy, fine-tuning
// TODO: überall explicit static cast

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

template <size_t maxN>
/// @param cache_maximumReeched enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
///                             werden kann; z.B. wenn cache_maximumReeched[poset] = 2, dann: benötige MEHR ALS 2
///                             Schritte, um Poset zu lösen
/// @param cache_solution enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
///                       cache_solution[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
/// @return true, wenn Median in poset in max. `maxComparisons` gefunden werden kann
SearchResult search(BS::thread_pool_light &threadpool, const Poset<maxN> &poset,
                    std::unordered_map<Poset<maxN>, int> &cache_maximumReeched, std::mutex &mutex_cache_maximumReeched,
                    std::unordered_map<Poset<maxN>, int> &cache_solution, std::mutex &mutex_cache_solution,
                    const int maxComparisons, const int maxMultiThreading, const std::atomic<bool> &atomicBreak,
                    Statistics &statistics, const int comparisonsDone) {
  if (atomicBreak) {
    return Unknown;
  }

  SearchResult result = NoSolution;

  mutex_cache_maximumReeched.lock();
  const bool temp1 = cache_maximumReeched.find(poset) != cache_maximumReeched.end() &&
                     maxComparisons - comparisonsDone <= cache_maximumReeched[poset];
  mutex_cache_maximumReeched.unlock();

  if (temp1) {
    ++statistics.hashMatches;
    result = NoSolution;
  } else {
    mutex_cache_solution.lock();
    const bool temp2 = cache_solution.find(poset) != cache_solution.end() && comparisonsDone <= cache_solution[poset];
    mutex_cache_solution.unlock();

    if (temp2) {
      ++statistics.hashMatches;
      result = FoundSolution;
    } else if (poset.canDetermineNSmallest()) {
      result = FoundSolution;
    } else if (comparisonsDone == maxComparisons) {
      result = NoSolution;
    } else {
      ++statistics.functionCalls;

      // TODO: custom ordering
      const auto temp = [&](std::atomic<bool> &breakCondition, const int i, const int j) {
        const SearchResult result1 = search(
            threadpool, poset.add_relation(i, j), cache_maximumReeched, mutex_cache_maximumReeched, cache_solution,
            mutex_cache_solution, maxComparisons, maxMultiThreading, breakCondition, statistics, comparisonsDone + 1);
        if (result1 == FoundSolution) {
          const SearchResult result2 = search(
              threadpool, poset.add_relation(j, i), cache_maximumReeched, mutex_cache_maximumReeched, cache_solution,
              mutex_cache_solution, maxComparisons, maxMultiThreading, breakCondition, statistics, comparisonsDone + 1);
          if (result2 == FoundSolution) {
            breakCondition = true;
          }
        }
      };

      if (comparisonsDone == maxMultiThreading) {
        std::atomic<bool> breakCondition(false);
        std::vector<std::future<void>> futures;
        for (int i = 0; i < poset.size(); ++i) {
          for (int j = i + 1; j < poset.size(); ++j) {
            if (!poset.is(i, j) && !poset.is(j, i)) {
              futures.push_back(std::async(std::launch::async, temp, std::ref(breakCondition), i, j));
              // threadpool.submit(temp, std::ref(breakCondition), i, j);
            }
          }
        }
        for (auto &fut : futures) fut.wait();
        // threadpool.wait_for_tasks();

        if (breakCondition) {
          result = FoundSolution;
        }
      } else {
        std::atomic<bool> breakCondition(false);
        for (int i = 0; i < poset.size() && result == NoSolution; ++i) {
          for (int j = i + 1; j < poset.size() && result == NoSolution; ++j) {
            if (!poset.is(i, j) && !poset.is(j, i)) {
              temp(breakCondition, i, j);
              if (breakCondition) {
                result = FoundSolution;
              }
            }
          }
        }
      }
    }
  }

  if (result == FoundSolution) {
    mutex_cache_solution.lock();
    if (cache_solution.find(poset) == cache_solution.end() || comparisonsDone < cache_solution[poset])
      cache_solution[poset] = comparisonsDone;
    mutex_cache_solution.unlock();
  } else {
    mutex_cache_maximumReeched.lock();
    if (cache_maximumReeched.find(poset) == cache_maximumReeched.end() ||
        cache_maximumReeched[poset] < maxComparisons - comparisonsDone)
      cache_maximumReeched[poset] = maxComparisons - comparisonsDone;
    mutex_cache_maximumReeched.unlock();
  }
  return result;
}

template <size_t maxN>
std::optional<int> startSearch(BS::thread_pool_light &threadpool, const int n, const int nthSmallest,
                               std::unordered_map<Poset<maxN>, int> &cache_maximumReeched,
                               std::unordered_map<Poset<maxN>, int> &cache_solution, Statistics &statistics) {
  if (0 == nthSmallest || n <= 2) {
    return n - 1;
  }

  int foundSolution;
  std::mutex mutex_cache_maximumReeched;
  std::mutex mutex_cache_solution;
  std::atomic<bool> atomicBreak(false);

  for (int i = n - 1; i < n * n; ++i) {
    std::cout << "\rtry: maxComparisons = " << i << std::flush;
    Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
    int comparisonsDone = 0;
    for (int k = 0; k < n - 1 && comparisonsDone < i; k += 2) {
      ++comparisonsDone;
      poset.addComparison(k, k + 1);
    }
    poset.normalize();

    const SearchResult is_possible =
        search(threadpool, poset, cache_maximumReeched, mutex_cache_maximumReeched, cache_solution,
               mutex_cache_solution, i, 0, atomicBreak, statistics, comparisonsDone);
    if (is_possible == FoundSolution) {
      foundSolution = i;
      break;
    }
  }

  cache_solution.clear();

  std::cout << "\rvalidate solution: " << foundSolution << "      " << std::flush;
  Poset<maxN> poset{uint8_t(n), uint8_t(nthSmallest)};
  int comparisonsDone = 1;
  poset.addComparison(0, 1);
  poset.normalize();

  const SearchResult is_possible =
      search(threadpool, poset, cache_maximumReeched, mutex_cache_maximumReeched, cache_solution, mutex_cache_solution,
             foundSolution - 1, 0, atomicBreak, statistics, comparisonsDone);
  if (is_possible == NoSolution) {
    return foundSolution;
  } else {
    std::cout << "found also solution with -1" << std::endl;
  }

  return {};
}

int main() {
  constexpr size_t maxN = 15;
  constexpr size_t nBound = 0;
#ifdef USE_NAUTY
  initNauty(maxN);
#endif

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  BS::thread_pool_light threadpool;
  std::unordered_map<Poset<maxN>, int> cache_maximumReeched, cache_solution;
  for (int n = 1; n < maxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      StopWatch watch{};

      Statistics statistics;
      const std::optional<int> comparisons =
          startSearch(threadpool, n, nthSmallest, cache_maximumReeched, cache_solution, statistics);

      if (comparisons.has_value()) {
        if (n >= nBound)
          std::cout << "\rtime '" << watch << "': n = " << n << ", i = " << nthSmallest << ", " << statistics
                    << ", entries = " << cache_solution.size() + cache_maximumReeched.size()
                    << ", comparisons: " << comparisons.value() << std::endl;
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