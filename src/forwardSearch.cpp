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

int factorial(int n) {
  if (n <= 1) {
    return 1;
  } else {
    return n * factorial(n - 1);
  }
}

static int counter = 0;

std::array<std::array<std::pair<float, float>, 15>, 15> test123dash;
// std::array<std::array<std::array<float, 25>, 15>, 15> test123dash2;
std::array<std::array<std::array<float, 25>, 15>, 15> test123;
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
      if constexpr (false) {
        const auto [value, table] = get_linear_extensions(poset);
        const auto cmp = [&](const std::pair<int, int> &a, const std::pair<int, int> &b) {
          return table[a.first][a.second] < table[b.first][b.second];
        };

        std::vector<std::pair<int, int>> temp;
        for (int i = 0; i < poset.size(); ++i) {
          for (int j = i + 1; j < poset.size(); ++j) {
            if (!poset.is_less(i, j) && !poset.is_less(j, i)) {
              // if (std::max(table[i][j], table[j][i]) > pow(2, remainingComparisons - 1) * factorial(poset.nth()) *
              //                                              factorial(poset.size() - poset.nth() - 1)) {
              //   continue;
              // }
              const auto [a, b] = test123dash[poset.size()][poset.nth()];
              if ((float)log2(std::max(table[i][j], table[j][i])) > a * remainingComparisons + b) {
                ++counter;
                continue;
              } else {
              }
              if (cmp({i, j}, {j, i})) {
                temp.push_back({i, j});
              } else {
                temp.push_back({j, i});
              }
            }
          }

          std::sort(temp.rbegin(), temp.rend(), cmp);

          float temp1 = 1000;
          for (const auto &[i, j] : temp) {
            auto tem = recursiveSearch(atomicBreak, i, j, normalizer);
            if (tem == FoundSolution) {
              result = FoundSolution;
              temp1 = std::min(temp1, float(log2(std::max(table[i][j], table[j][i]))) + 0.00001f);
              break;
            }
          }
          if (1000 != temp1) {
            test123[poset.size()][poset.nth()][remainingComparisons] =
                std::max(test123[poset.size()][poset.nth()][remainingComparisons], temp1);
          }
        }
      } else if constexpr (SORT_DFS_BRANCHES) {
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

void linearRegression(const std::vector<double> &x, const std::vector<double> &y, double &a, double &b) {
  int n = x.size();

  // Calculate necessary sums
  double sumX = 0.0, sumY = 0.0, sumXY = 0.0, sumX2 = 0.0;
  for (int i = 0; i < n; ++i) {
    sumX += x[i];
    sumY += y[i];
    sumXY += x[i] * y[i];
    sumX2 += x[i] * x[i];
  }

  // Calculate coefficients 'a' and 'b'
  a = (n * sumXY - sumX * sumY) / (n * sumX2 - sumX * sumX);
  b = (sumY - a * sumX) / n;
}

int main() {
  // for (int n = 0; n < 15; ++n) {
  //   for (int k = 0; k < 15; ++k) {
  //     test123dash[n][k] = {1, log2(factorial(k) * factorial(n - k - 1)) - 1};
  //     std::cout << "test123dash[" << n << "][" << k << "] = {" << test123dash[n][k].first << ", "
  //               << test123dash[n][k].second << "};" << std::endl;
  //   }
  // }

  // test123dash[3][1] = {1.000000, 0.000000};

  // test123dash[4][1] = {1.000000, 0.000000};

  // test123dash[5][1] = {1.000000, 0.321938};
  // test123dash[5][2] = {1.000000, 0.169935};

  // test123dash[6][1] = {1.000000, 2.321938};
  // test123dash[6][2] = {1.000000, 1.584972};

  // test123dash[7][1] = {1.000000, 3.714255};
  // test123dash[7][2] = {1.000000, 2.906900};
  // test123dash[7][3] = {1.000000, 2.584972};

  // test123dash[8][2] = {1.000000, 5.076825};
  // test123dash[8][3] = {1.000000, 4.491862};

  // test123dash[9][3] = {1.000000, 6.228828 * 0.6};
  // test123dash[9][4] = {1.000000, 5.960011};

  // test123dash[3][1] = {0.584962, -0.169915};

  // test123dash[4][1] = {1.000000, -0.415028};

  // test123dash[5][1] = {0.875489, 0.944494};
  // test123dash[5][2] = {0.868483, 0.564487};

  // test123dash[6][1] = {0.709474, 3.484043};
  // test123dash[6][2] = {0.672067, 2.896703};

  // test123dash[7][1] = {0.875489, 4.461324};
  // test123dash[7][2] = {0.718510, 4.290105};
  // test123dash[7][3] = {0.869290, 3.107810};

  // test123dash[8][1] = {0.600000, 8.299217};
  // test123dash[8][2] = {0.531139, 6.996285};
  // test123dash[8][3] = {0.865463, 5.030012};

  // test123dash[9][1] = {0.660964, 9.918396};
  // test123dash[9][2] = {0.759808, 8.019551};
  // test123dash[9][3] = {0.520029, 8.243977};
  // test123dash[9][4] = {0.538434, 8.084798};

  // test123dash[10][2] = {0.574608, 11.513998};
  // test123dash[10][3] = {0.511384, 10.770338};
  // // test123dash[10][4] = {0.666737, 8.731593};
  // test123dash[10][4] = {0.681425, 8.295392};
  // // test123dash[10][4] = {0.7009844, 8.794294};
  // // test123dash[10][4] = {0.72161157, 7.85334371};

  // test123dash[11][3] = {0.689856, 11.697412};

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

      if (false && n == 9 && nthSmallest == 4) {
        std::cout.precision(6);
        for (int n = 0; n <= 11; ++n) {
          for (int k = 0; k < (n + 1) / 2; ++k) {
            // for (int i = 0; i < 25; ++i) {
            //   if (n == 10 && k == 4 && 0 != test123[n][k][i]) {
            //     std::cout << i << ": " << test123[n][k][i] << std::endl;
            //   }
            // }

            double a, b;
            std::vector<double> x, y;
            for (int i = 0; i < 25; ++i) {
              if (0 != test123[n][k][i]) {
                x.push_back(i);
                y.push_back(test123[n][k][i]);
              }
            }
            // linearRegression(x, y, a, b);
            // for (int i = 0; i < 25; ++i) {
            //   if (0 != test123[n][k][i]) {
            //     float temp = a * i + b - test123[n][k][i];
            //     if (temp < 0) {
            //       b -= temp;
            //     }
            //   }
            // }
            // b = n;
            // a = 0;
            // for (int i = 0; i < 25; ++i) {
            //   if (0 != test123[n][k][i]) {
            //     float temp = (test123[n][k][i] - n) / i;
            //     if (temp > a) {
            //       a = temp;
            //     }
            //   }
            // }

            a = 1;
            b = 0;
            for (int i = 0; i < 25; ++i) {
              if (0 != test123[n][k][i]) {
                float temp = float(test123[n][k][i]) / a - i;
                if (temp > b) {
                  b = temp;
                }
              }
            }

            if (x.size() >= 2) {
              // std::cout << "test123dash[" << n << "][" << k << "] = {" << a << ", " << b << "};" << std::endl;
            }
          }
        }
        std::cout << counter << std::endl;
        exit(0);
      }
    }
    if (n >= nBound) std::cout << std::endl;
  }
  return 0;
}

// int main() {
//   Poset<globalMaxN> poset{4, 0};
//   poset.add_less(0, 2);
//   poset.add_less(1, 2);
//   poset.add_less(1, 3);

//   std::cout << get_linear_extensions(poset) << std::endl;
//   std::cout << "[[0, 2, 5, 4], [3, 0, 5, 5], [0, 0, 0, 2], [1, 0, 3, 0]]" << std::endl;
// }