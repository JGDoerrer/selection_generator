#include <bits/stdc++.h>

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
    /* n=12 */ {11, 14, 17, 18, 19, 20, 20, 19, 18, 17, 14, 11}};
const int min_n_comparisons_len = 12;

std::ostream &operator<<(std::ostream &os, const std::chrono::nanoseconds &nanos) {
  os << (std::chrono::duration_cast<std::chrono::milliseconds>(nanos).count() / 1000.0) << "s";
  return os;
}

#include "poset.h"
// ===================
#include "normalizer.h"

constexpr size_t globalMaxComparisons = 25;
constexpr size_t globalMaxN = 15;
constexpr size_t threadCount = 20;

#include "cache_Set.h"

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    PosetCacheSet<maxN, globalMaxComparisons> &poset_cache, const uint8_t n, const uint8_t nthSmallest,
    const int maxComparisons) {
  Normalizer<maxN> normalizer{};
  std::chrono::nanoseconds duration_build_posets_total{}, duration_test_posets_total{};
  std::unordered_set<Poset<maxN>> source{{Poset<globalMaxN>(1, 0)}};
  for (int k = 1; k < maxComparisons; ++k) {
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
              if (predecessor == Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest}) {
                duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
                duration_test_posets_total += duration_test_posets;
                std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in "
                          << duration_build_posets << " ~ " << duration_test_posets
                          << " | total cached: " << poset_cache.size() << " (found solution)" << std::endl;
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
    duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
    duration_test_posets_total += duration_test_posets;

    std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in " << duration_build_posets
              << " ~ " << duration_test_posets << " | total cached: " << poset_cache.size() << std::endl;

    source = destination;
  }

  return {std::nullopt, duration_build_posets_total, duration_test_posets_total};
}

int main() {
  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  if constexpr (false) {
    PosetCacheSet<globalMaxN, globalMaxComparisons> poset_cache;
    poset_cache.insert_solvable(Poset<globalMaxN>(1, 0), 0);

    const int n = 11;
    const int nthSmallest = 3;

    const auto &[comparisons, durationGeneratePosets, durationSearch] =
        startSearchBackward<globalMaxN>(poset_cache, n, nthSmallest, n * n);

    if (comparisons.has_value()) {
      std::cout << "\rtime '" << durationGeneratePosets << " + " << durationSearch << " = "
                << durationGeneratePosets + durationSearch << "': n = " << n << ", i = " << nthSmallest
                << ", comparisons: " << comparisons.value() << std::endl;
      if (comparisons != min_n_comparisons[n][nthSmallest]) {
        std::cerr << "Error: got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
                  << std::endl;
        exit(0);
      }
    } else {
      std::cerr << "Error: got 'nothing' but expected " << min_n_comparisons[n][nthSmallest] << std::endl;
      exit(0);
    }
  } else {
    constexpr size_t nBound = 0;

    PosetCacheSet<globalMaxN, globalMaxComparisons> poset_cache;
    poset_cache.insert_solvable(Poset<globalMaxN>(1, 0), 0);

    for (int n = 2; n < 15; ++n) {
      for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
        const auto &[comparisons, durationGeneratePosets, durationSearch] =
            startSearchBackward<globalMaxN>(poset_cache, n, nthSmallest, n * n);

        if (comparisons.has_value()) {
          if (n >= nBound) {
            std::cout << "\rtime '" << durationGeneratePosets << " + " << durationSearch << " = "
                      << durationGeneratePosets + durationSearch << "': n = " << n << ", i = " << nthSmallest
                      << ", comparisons: " << comparisons.value() << std::endl;
          }
          if (comparisons != min_n_comparisons[n][nthSmallest]) {
            std::cerr << "Error: got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
                      << std::endl;
            exit(0);
          }
        } else {
          std::cerr << "Error: got 'nothing' but expected " << min_n_comparisons[n][nthSmallest] << std::endl;
          exit(0);
        }
      }
      if (n >= nBound) std::cout << std::endl;
    }
  }
}