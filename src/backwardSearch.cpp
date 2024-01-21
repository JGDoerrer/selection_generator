#include "main.h"

std::array<std::array<std::optional<std::unordered_set<Poset<globalMaxN>>>, globalMaxN>, globalMaxN> myBigCache;

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

int factorial(int n) {
  if (n <= 1) {
    return 1;
  } else {
    return n * factorial(n - 1);
  }
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