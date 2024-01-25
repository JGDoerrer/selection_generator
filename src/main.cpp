#include <bits/stdc++.h>

#include "util.h"
#include "poset.h"
// ====
#include "cache_set.h"
#include "cache_tree.h"

std::ostream &operator<<(std::ostream &os, const std::chrono::nanoseconds &nanos) {
  os << (std::chrono::duration_cast<std::chrono::milliseconds>(nanos).count() / 1000.0) << "s";
  return os;
}

// using Cache = CacheTreeMemoryEffient;
using Cache = CacheTree;
// using Cache = CacheSet;

enum SearchResult : uint8_t { FoundSolution, NoSolution, Unknown };

std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    PosetCacheSet<true> &poset_cache, const uint8_t n, const uint8_t nthSmallest, const int maxComparisons) {
  std::chrono::nanoseconds duration_build_posets_total{}, duration_test_posets_total{};
  std::unordered_set<Poset> source{{Poset(1, 0)}};
  for (int k = 1; k < maxComparisons; ++k) {
    std::chrono::nanoseconds duration_build_posets{}, duration_test_posets{};
    const auto start = std::chrono::high_resolution_clock::now();
    const auto source_new = Poset::enlarge(source, n, nthSmallest);
    const auto mid = std::chrono::high_resolution_clock::now();
    duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    std::unordered_set<Poset> destination;
    for (const Poset &item : source_new) {
      for (uint8_t i = 0; i < n; ++i) {
        for (uint8_t j = 0; j < n; ++j) {
          if (item.is_less(i, j)) {
            for (const Poset &predecessor : item.remove_less(i, j, [&poset_cache, k](const Poset &poset) {
                   return poset_cache.check(poset, k - 1);
                 })) {
              if (predecessor == Poset{(uint8_t)n, (uint8_t)nthSmallest}) {
                duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
                duration_test_posets_total += duration_test_posets;
                std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in "
                          << duration_build_posets << " ~ " << duration_test_posets
                          << " | total cached: " << poset_cache.size() << " (found solution)" << std::endl;
                return {k, duration_build_posets_total, duration_test_posets_total};
              }

              Poset predecessor_normalized = predecessor;
              predecessor_normalized.normalize();
              if (poset_cache.check(predecessor_normalized, k - 1)) {
                continue;
              }

              destination.insert(predecessor_normalized);
              poset_cache.insert(predecessor_normalized, k);
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
    PosetCacheSet<true> poset_cache;
    poset_cache.insert(Poset(1, 0), 0);

    const int n = 11;
    const int nthSmallest = 3;

    const auto &[comparisons, durationGeneratePosets, durationSearch] =
        startSearchBackward(poset_cache, n, nthSmallest, n * n);

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

    PosetCacheSet<true> poset_cache;
    poset_cache.insert(Poset(1, 0), 0);

    for (int n = 2; n < 15; ++n) {
      for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
        const auto &[comparisons, durationGeneratePosets, durationSearch] =
            startSearchBackward(poset_cache, n, nthSmallest, n * n);

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