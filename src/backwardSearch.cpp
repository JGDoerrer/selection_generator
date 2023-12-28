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
    std::unordered_map<Poset<globalMaxN>, int> &allPos_reduced, const uint8_t n, const uint8_t nthSmallest) {
  Normalizer<maxN> normalizer{};
  std::chrono::nanoseconds duration0g{}, duration1g{};

  // std::unordered_set<Poset<maxN>> source = find_solvable_posets(normalizer, 0, n, nthSmallest);
  std::unordered_set<Poset<maxN>> source_reduced{{Poset<globalMaxN>(1, 0)}};

  for (int k = 1; k < n * n; ++k) {
    std::chrono::nanoseconds duration0{}, duration1{};
    const auto start0 = std::chrono::high_resolution_clock::now();
    auto source_new = enlarge(normalizer, source_reduced, n, nthSmallest);
    // std::unordered_set<Poset<maxN>> source_new2;
    // for (auto item : source_new) {
    //   bool found = false;
    //   for (auto item2 : source_new2) {
    //     if (item2.subset_of(item)) {
    //       found = true;
    //     }
    //   }
    //   if (!found) {
    //     source_new2.insert(item);
    //   } else {
    //     // assert(false);
    //   }
    // }
    // source_new = source_new2;
    // source_new.delete all from source
    const auto end0 = std::chrono::high_resolution_clock::now();
    duration0 = end0 - start0;
    duration0g += duration0;

    const auto start1 = std::chrono::high_resolution_clock::now();
    std::unordered_set<Poset<maxN>> destination;  // pqrc1
    std::unordered_set<Poset<maxN>> destination_reduced;
    for (const Poset<maxN> &item : source_new) {
      for (uint8_t i = 0; i < n; ++i) {
        for (uint8_t j = 0; j < n; ++j) {
          if (item.is_less(i, j)) {
            for (const Poset<maxN> &predecessor : item.remove_less(normalizer, i, j)) {
              if (predecessor == Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest}) {
                std::cout << "# " << k << ": " << source_reduced.size() << " => " << source_new.size() << " in "
                          << duration0 << " ~ " << duration1 << " | total cached: " << allPos_reduced.size()
                          << " (found solution)" << std::endl;
                return {k, duration0g, duration1g};
              }

              Poset<maxN> predecessorNorm0_reduced = predecessor;
              normalizer.normalize(predecessorNorm0_reduced);
              if (allPos_reduced.contains(predecessorNorm0_reduced) && allPos_reduced[predecessorNorm0_reduced] < k) {
                continue;
              }

              Poset<maxN> predecessorNorm_reduced = predecessor;
              predecessorNorm_reduced.add_less(j, i);
              normalizer.normalize(predecessorNorm_reduced);
              if (!(allPos_reduced.contains(predecessorNorm_reduced) && allPos_reduced[predecessorNorm_reduced] < k)) {
                continue;
              }

              // Poset<maxN> predecessorNorm0 = predecessor;
              // normalizer.canonify(predecessorNorm0);
              // destination.insert(predecessorNorm0);

              destination_reduced.insert(predecessorNorm0_reduced);
              allPos_reduced[predecessorNorm0_reduced] = k;
            }
          }
        }
      }
    }
    // source = destination;
    const auto end1 = std::chrono::high_resolution_clock::now();
    duration1 = end1 - start1;
    duration1g += duration1;

    std::cout << "# " << k << ": " << source_reduced.size() << " => " << source_new.size() << " in " << duration0
              << " ~ " << duration1 << " | total cached: " << allPos_reduced.size() << std::endl;

    source_reduced = destination_reduced;
  }

  return {std::nullopt, duration0g, duration1g};
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
        std::unordered_set<Poset<globalMaxN>> result =
            enlarge(normalizer, std::unordered_set<Poset<globalMaxN>>{{Poset<globalMaxN>(1, 0)}}, n, k);
        std::chrono::time_point mid = std::chrono::high_resolution_clock::now();
        std::unordered_set<Poset<globalMaxN>> result2 = find_solvable_posets(normalizer, n, k);
        std::chrono::time_point end = std::chrono::high_resolution_clock::now();
        std::cout << n << ", " << k << ": " << result.size() << " - " << result2.size() << " in: " << mid - start
                  << " | " << end - mid << std::endl;
        assert(result == result2);
      }
    }
  } else {
    constexpr size_t nBound = 0;

    std::unordered_map<Poset<globalMaxN>, int> allPos_reduced;
    allPos_reduced[Poset<globalMaxN>(1, 0)] = 0;

    for (int n = 2; n < 15; ++n) {
      for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
        const auto &[comparisons, durationGeneratePosets, durationSearch] =
            startSearchBackward<globalMaxN>(allPos_reduced, n, nthSmallest);

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