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

int main32() {
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
  return 0;
}

using namespace std;

bitset<14> operator&(bitset<14> a, const bitset<14> b) {
  a &= b;
  return a;
}

bitset<14> operator|(bitset<14> a, const bitset<14> b) {
  a &= b;
  return a;
}

template <std::size_t maxN>
unordered_set<int> to_set(bitset<maxN> value) {
  unordered_set<int> result;
  for (uint8_t i = 0; i < maxN; ++i, value >>= 1) {
    bitset<maxN> item{value};
    if ((item &= 0b1).any()) {
      result.insert(i);
    }
  }
  return result;
}

template <std::size_t maxN>
bool is_down_set(const bitset<maxN> D, const bitset<maxN> U, const Poset<maxN> &poset) {
  bitset<maxN> temp = D;
  temp |= U;
  if (temp != U) {
    return false;
  }
  for (uint8_t x = 0; x < maxN; ++x) {
    for (uint8_t y = 0; y < maxN; ++y) {
      if ((bitset<maxN>{D >> x} & 0b1).any() && (bitset<maxN>{U >> y} & 0b1).any() &&
          (bitset<maxN>{1 << y} & ~D).any() && poset.is_less(y, x)) {
        return false;
      }
    }
  }

  return true;
}

template <std::size_t maxN>
vector<bitset<maxN>> get_permutations(const uint8_t level) {
  vector<bitset<maxN>> result;
  if (0 == level) {
    result.push_back(0);
    return result;
  } else {
    for (bitset<maxN> item : get_permutations<maxN>(level - 1)) {
      item <<= 1;
      result.push_back(item);
      item |= 1;
      result.push_back(item);
    }
    return result;
  }
}

int rec(const int n, vector<vector<int>> &adj, vector<int> &result) {
  result[n] = (0 == n) ? 1 : 0;
  for (int i : adj[n]) {
    result[n] += rec(i, adj, result);
  }
  return result[n];
}

int rec2(const int n, vector<vector<int>> &adj, vector<int> &result) {
  result[n] = (15 == n) ? 1 : 0;
  for (int i : adj[n]) {
    result[n] += rec2(i, adj, result);
  }
  return result[n];
}

int main() {
  constexpr size_t maxN = 14;

  Poset<maxN> P{4, 0};
  P.add_less(0, 2);
  P.add_less(1, 2);
  P.add_less(1, 3);

  const bitset<maxN> U = (1 << P.size()) - 1;
  vector<bitset<maxN>> downsets;
  for (const bitset<maxN> D : get_permutations<maxN>(P.size())) {
    if (is_down_set(D, U, P)) {
      std::cout << to_set(D) << " is downset of " << to_set(U) << std::endl;
      downsets.push_back(D);
    }
  }

  int counter = 0;
  vector<vector<int>> adj(16);
  vector<vector<int>> adj2(16);
  for (bitset<maxN> D1 : downsets) {
    if (is_down_set(D1, U, P)) {
      bitset<maxN> u = U;
      for (uint8_t x = 0; x < maxN; ++x, u >>= 1) {
        bitset<maxN> item{u};
        if ((item &= 0b1).any()) {
          bitset<maxN> D2 = D1;
          if (!(bitset<maxN>{D2 >> x} & 0b1).any()) {
            D2 |= bitset<maxN>{1 << x};
            if (is_down_set(D2, U, P)) {
              adj2[D1.to_ulong()].push_back(D2.to_ulong());
              adj[D2.to_ulong()].push_back(D1.to_ulong());
              std::cout << to_set(D1) << " to " << to_set(D2) << std::endl;
              // std::cout << D1.to_ulong() << " to " << D2.to_ulong() << std::endl;
              ++counter;
            }
          }
        }
      }
    }
  }
  cout << counter << endl;

  vector<int> d1(16);
  cout << "d(D): " << rec(15, adj, d1) << endl;
  for (int i = 0; i < 16; ++i) {
    if (0 != d1[i]) cout << to_set(bitset<maxN>{i}) << ": " << d1[i] << endl;
  }

  vector<int> u1(16);
  cout << "u(D): " << rec2(0, adj2, u1) << endl;
  for (int i = 0; i < 16; ++i) {
    if (0 != u1[i]) cout << to_set(bitset<maxN>{i}) << ": " << u1[i] << endl;
  }

  vector<vector<int>> t1(P.size(), vector<int>(P.size(), 0));
  for (int j = 0; j < P.size(); ++j) {
    for (int k = 0; k < P.size(); ++k) {
      if (j != k)
        for (int v = 0; v < 16; ++v) {
          auto w = v | (1 << j);
          bool found = false;
          for (auto ww : adj2[v]) {
            if (w == ww) found = true;
          }
          if (found && !bitset<maxN>((1 << k) & w).any()) {
            t1[j][k] += d1[v] * u1[w];
          }
        }
    }
  }
  cout << t1 << endl;
  // [[0, 2, 5, 4], [3, 0, 5, 5], [0, 0, 0, 2], [1, 0, 3, 0]]
}
// {} is downset of {3, 2, 1, 0}
// {1} is downset of {3, 2, 1, 0}
// {3, 1} is downset of {3, 2, 1, 0}
// {0} is downset of {3, 2, 1, 0}
// {1, 0} is downset of {3, 2, 1, 0}
// {3, 1, 0} is downset of {3, 2, 1, 0}
// {2, 1, 0} is downset of {3, 2, 1, 0}
// {3, 2, 1, 0} is downset of {3, 2, 1, 0}
// {} to {1}
// {} to {0}
// {1} to {3, 1}
// {1} to {0, 1}
// {3, 1} to {0, 3, 1}
// {0} to {1, 0}
// {1, 0} to {3, 1, 0}
// {1, 0} to {2, 1, 0}
// {3, 1, 0} to {2, 3, 1, 0}
// {2, 1, 0} to {3, 2, 1, 0}
// 10