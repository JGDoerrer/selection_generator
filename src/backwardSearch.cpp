#include "main.h"

// TODO: alternative, da aktuell zu langsam: nur Cache für forwardSeach befüllen

std::array<std::array<std::optional<std::unordered_set<Poset<globalMaxN>>>, globalMaxN>, globalMaxN> myBigCache;

template <size_t maxN>
std::unordered_set<Poset<maxN>> find_solvable_posets(Normalizer<maxN> &normalizer, const uint8_t n, const uint8_t k) {
  assert(2 * k < n);
  if (myBigCache[n][k] == std::nullopt) {
    if (1 == n) {
      myBigCache[n][k] = std::unordered_set<Poset<maxN>>{{Poset<maxN>{1, 0}}};
    } else {
      std::unordered_set<Poset<maxN>> result{};
      if (n - 1 <= 2 * k) {
        for (Poset<maxN> item : find_solvable_posets(normalizer, n - 1, (n - 1) - k - 1)) {
          item.dual();
          normalizer.canonify(item);
          result.insert(item);
        }
      } else {
        result = find_solvable_posets(normalizer, n - 1, k);
      }
      myBigCache[n][k] = enlarge_n(normalizer, result);
    }
  }
  return myBigCache[n][k].value();
}

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    std::unordered_map<Poset<globalMaxN>, int> &poset_cache, const uint8_t n, const uint8_t nthSmallest) {
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
            for (const Poset<maxN> &predecessor : item.remove_less(normalizer, i, j)) {
              if (predecessor == Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest}) {
                duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
                duration_test_posets_total += duration_test_posets;
                std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in "
                          << duration_build_posets << " ~ " << duration_test_posets
                          << " | total cached: " << poset_cache.size() << " (found solution)" << std::endl;
                return {k, duration_build_posets_total, duration_test_posets_total};
              }

              Poset<maxN> predecessor_normalized = predecessor;
              normalizer.normalize(predecessor_normalized);
              if (poset_cache.contains(predecessor_normalized) && poset_cache[predecessor_normalized] < k) {
                continue;
              }

              Poset<maxN> predecessor_check = predecessor.with_less(j, i);
              normalizer.normalize(predecessor_check);
              if (!(poset_cache.contains(predecessor_check) && poset_cache[predecessor_check] < k)) {
                continue;
              }

              destination.insert(predecessor_normalized);
              poset_cache[predecessor_normalized] = k;
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

  // auffällig:
  // (n, 2).size() == 2 * (n - 1, 1).size() == 2 * (n - 2, 0).size()

  // 10, 0: 0 - 183231 in: 0.000s | 1.822s
  // 10, 1: 0 - 16999 in: 0.000s | 0.140s
  // 10, 2: 0 - 4090 in: 0.000s | 0.031s
  // 10, 3: 0 - 1590 in: 0.000s | 0.009s
  // 10, 4: 0 - 1008 in: 0.000s | 0.004s

  // 11, 0: 0 - 2567284 in: 0.000s | 39.674s
  // 11, 1: 0 - 183231 in: 0.000s | 3.085s
  // 11, 2: 0 - 33998 in: 0.000s | 0.583s
  // 11, 3: 0 - 10225 in: 0.000s | 0.163s
  // 11, 4: 0 - 5088 in: 0.000s | 0.064s
  // 11, 5: 0 - 3969 in: 0.000s | 0.041s

  if constexpr (false) {
    Normalizer<globalMaxN> normalizer{};
    for (auto item : enlarge_nk(normalizer, std::unordered_set<Poset<globalMaxN>>{{Poset<globalMaxN>(2, 0)}})) {
      std::cout << item << std::endl;
    }
  } else if constexpr (false) {
    Normalizer<globalMaxN> normalizer{};

    for (int n = 1; n < 12; ++n) {
      for (int k = 0; k < (n + 1) / 2; ++k) {
        std::chrono::time_point start = std::chrono::high_resolution_clock::now();
        std::unordered_set<Poset<globalMaxN>>
            result;  // = enlarge(normalizer, std::unordered_set<Poset<globalMaxN>>{{Poset<globalMaxN>(1, 0)}}, n, k);
        std::chrono::time_point mid = std::chrono::high_resolution_clock::now();
        std::unordered_set<Poset<globalMaxN>> result2 = find_solvable_posets(normalizer, n, k);
        std::chrono::time_point end = std::chrono::high_resolution_clock::now();
        std::cout << n << ", " << k << ": " << result.size() << " - " << result2.size() << " in: " << mid - start
                  << " | " << end - mid << std::endl;
        assert(result == result2);
      }
    }
  } else if constexpr (false) {
    std::unordered_map<Poset<globalMaxN>, int> poset_cache;
    poset_cache[Poset<globalMaxN>(1, 0)] = 0;

    const int n = 12;
    const int nthSmallest = 5;

    const auto &[comparisons, durationGeneratePosets, durationSearch] =
        startSearchBackward<globalMaxN>(poset_cache, n, nthSmallest);

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

    std::unordered_map<Poset<globalMaxN>, int> poset_cache;
    poset_cache[Poset<globalMaxN>(1, 0)] = 0;

    for (int n = 2; n < 15; ++n) {
      for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
        const auto &[comparisons, durationGeneratePosets, durationSearch] =
            startSearchBackward<globalMaxN>(poset_cache, n, nthSmallest);

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