#include "main.h"

#if 0
std::array<Normalizer<globalMaxN>, 230> norm;  // TODO: only Debug

template <size_t maxN>
Poset<maxN> createPosetWithComparison(const int normalizerIndex, Poset<maxN> poset, const uint16_t i,
                                      const uint16_t j) {
  poset.add_less(i, j);
  norm[normalizerIndex].canonify_nauty(poset);
  return poset;
};

/// @param cache_lowerBound enthält alle Posets, für die mit max. `maxComparisons` Schritten keine Lösung bestimmt
///                         werden kann; z.B. wenn cache_lowerBound[poset] = 2, dann: benötige MEHR ALS 2 Schritte,
///                         um Poset zu lösen
/// @param cache_upperBound enhält alle Posets, für die bereits eine Lösung gefunden wurde; z.B. wenn
///                         cache_upperBound[poset] = 2, dann kann poset IN 2 Schrittem gelöst werden
/// @return true, wenn Median in poset in max. `maxComparisons` gefunden werden kann
template <size_t maxN>
SearchResult searchRecursive(const Poset<maxN> &poset, Cache<Poset<maxN>, uint8_t> cache_lowerBound[globalMaxN],
                             Cache<Poset<maxN>, uint8_t> cache_upperBound[globalMaxN],
                             const uint8_t remainingComparisons) {
  SearchResult result = NoSolution;
  if (cache_lowerBound[poset.size()].checkLower(poset, remainingComparisons)) {
    return NoSolution;
  } else if (cache_upperBound[poset.size()].checkUpper(poset, remainingComparisons)) {
    return FoundSolution;
    // durch normalisierung können alle posets auf n == 1 reduziert werden, d.h. is_solvable unnötig
  } else if (poset.is_solvable()) {
    result = FoundSolution;
  } else if (poset.is_not_solvable_in(remainingComparisons)) {
    result = NoSolution;
  } else {
    for (int i = 0; i < poset.size() && result != FoundSolution; ++i) {
      for (int j = i + 1; j < poset.size() && result != FoundSolution; ++j) {
        if (!poset.is_less(i, j) && !poset.is_less(j, i)) {
          result = searchRecursive(createPosetWithComparison(0, poset, i, j), cache_lowerBound, cache_upperBound,
                                   remainingComparisons - 1);
          if (result == FoundSolution) {
            result = searchRecursive(createPosetWithComparison(0, poset, j, i), cache_lowerBound, cache_upperBound,
                                     remainingComparisons - 1);
          }
        }
      }
    }
  }

  if (result == NoSolution) {
    cache_lowerBound[poset.size()].insert_ifLower(poset, remainingComparisons);
  } else if (result == FoundSolution) {
    cache_upperBound[poset.size()].insert_ifUpper(poset, remainingComparisons);
  }
  return result;
}

Cache<Poset<globalMaxN>, uint8_t> cache_lowerBound[globalMaxN];
Cache<Poset<globalMaxN>, uint8_t> cache_upperBound[globalMaxN];

template <size_t maxN>
int forward(Poset<maxN> poset) {
  cache_upperBound[1].insert(Poset<globalMaxN>(1, 0), 0);

  norm[0].canonify_nauty(poset);
  for (int i = 0; i < poset.size() * poset.size(); ++i) {
    if (searchRecursive(poset, cache_lowerBound, cache_upperBound, i) == FoundSolution) {
      return i;
    }
  }

  std::cerr << "Error, maxComparisons exceeded" << std::endl;
  std::cerr << poset << std::endl;
  exit(0);
}
#endif

// Option 1: nur cache befüllen

// Option 2: suche alle posets, die folgendes erfüllen:
// für ein bel. k: (sei k = 0)
// es ex. `nthSmallest` versch. i mit : is_less(i, 0)
// es ex. `n - nthSmallest - 1` versch. i mit : is_less(0, i)

// Option 3: starte mit [1] und call anti_reduceN, anti_reduceNthsmallest

// Radioactive Hardstyle Remix
// overdose
// startship hardstyle remix
// halo (techno version)
// Deck the halls - timmy trupet

std::array<std::array<std::optional<std::unordered_set<Poset<globalMaxN>>>, globalMaxN>, globalMaxN> myBigCache;

template <size_t maxN>
std::unordered_set<Poset<maxN>> find_solvable_posets(Normalizer<maxN> &normalizer, const uint8_t n, const uint8_t k) {
  if (1 == n) {
    std::unordered_set<Poset<maxN>> result{};
    result.insert(Poset<maxN>{n, k});
    return result;
  } else if (myBigCache[n][k] != std::nullopt) {
    return myBigCache[n][k].value();
  } else if (myBigCache[n][n - k - 1] != std::nullopt) {
    std::unordered_set<Poset<maxN>> result;
    for (Poset<maxN> item : myBigCache[n][n - k - 1].value()) {
      normalizer.invert_nthSmallest(item);
      result.insert(item);
    }
    return result;
  }

  myBigCache[n][k] = std::unordered_set<Poset<maxN>>{};
  for (const Poset<maxN> &poset : enlarge(normalizer, find_solvable_posets(normalizer, n - 1, k))) {
    if (poset.is_solvable()) {
      myBigCache[n][k].value().insert(poset);
    }
  }

  return myBigCache[n][k].value();
}

// std::unordered_map<Poset<globalMaxN>, std::unordered_set<Poset<globalMaxN>>> cache_solvable_in;

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    const uint8_t n, const uint8_t nthSmallest) {
  const std::chrono::time_point start = std::chrono::high_resolution_clock::now();

  Normalizer<maxN> normalizer{};
  std::unordered_set<Poset<maxN>> source = find_solvable_posets(normalizer, n, nthSmallest);

  std::chrono::time_point end = std::chrono::high_resolution_clock::now();

  std::unordered_set<Poset<maxN>> allPos;
  // std::array<std::unordered_set<Poset<maxN>>, 10> container;
  for (int k = 1; k < n * n; ++k) {
    // container[k - 1] = source;
    allPos.merge(std::unordered_set<Poset<maxN>>(source));
    // std::cout << k - 1 << ": " << source.size() << " " << allPos.size() << std::endl;

    std::unordered_set<Poset<maxN>> destination;
    for (const Poset<maxN> &item : source) {
      // Poset<maxN> item2 = item;
      // item2.change_nth(0);
      // auto inh = cache_solvable_in.find(item2);
      // if (inh != cache_solvable_in.end()) {
      //   std::cout << inh->second.size() << std::endl;
      //   destination.merge(std::unordered_set<Poset<maxN>>(inh->second));
      //   continue;
      // }

      // TODO: übergreifenden Cache integrieren
      // TODO: anti_reduce / enlarge => improve algorithm idea
      // TODO: improve enlarge

      for (uint8_t i = 0; i < n; ++i) {
        for (uint8_t j = 0; j < n; ++j) {
          if (item.is_less(i, j)) {
            for (const Poset<maxN> &predecessor : item.remove_less(normalizer, i, j)) {
              Poset<maxN> predecessorNorm0 = predecessor;
              normalizer.canonify_nauty(predecessorNorm0);
              if (allPos.contains(predecessorNorm0)) {
                continue;
              }

              Poset<maxN> predecessorNorm = predecessor;
              predecessorNorm.add_less(j, i);
              normalizer.canonify_nauty(predecessorNorm);
              if (!allPos.contains(predecessorNorm)) {
                continue;
              }

              destination.insert(predecessorNorm0);
              // cache_solvable_in.insert(predecessorNorm0, k);

              if (predecessorNorm0 == Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest}) {
                // for (int q = 0; q < k; ++q) {
                //   std::cout << q << ": " << container[q].size() << std::endl;
                // }
                // // container[q]: für n = 7 komme ich von allen Posets in container[q] in q Schritten ans Ziel
                // // source2:      für n = 8 komme ich von allen Posets in source2 in 0 Schritten ans Ziel
                // // ges: source2'
                // std::cout << container[5].size() << std::endl;

                // std::unordered_set<Poset<maxN>> newItem{};
                // for (const Poset<maxN> &poset : enlarge(normalizer, container[5])) {
                //   newItem.insert(poset);
                // }
                // std::cout << newItem.size() << std::endl;

                return {k, end - start, std::chrono::high_resolution_clock::now() - end};
              }
            }
          }
        }
      }
    }
    source = destination;
  }

  return {std::nullopt, end - start, std::chrono::high_resolution_clock::now() - end};
}

int main() {
  // auffällig:
  // (n, 2).size() == 2 * (n - 1, 1).size() == 2 * (n - 2, 0).size()

  // 10, 0: 0 - 183231 in: 0s | 11.888s
  // 10, 1: 0 - 16999 in: 0s | 0.781s
  // 10, 2: 0 - 4090 in: 0s | 0.168s
  // 10, 3: 0 - 1590 in: 0s | 0.074s
  // 10, 4: 0 - 1008 in: 0s | 0.055s

  // Normalizer<globalMaxN> normalizer{};
  // for (int n = 1; n < 15; ++n) {
  //   for (int k = 0; k < (n + 1) / 2; ++k) {
  //     std::chrono::time_point start = std::chrono::high_resolution_clock::now();
  //     std::unordered_set<Poset<globalMaxN>> result;  // = find_solvable_posets(normalizer, n, k);
  //     std::chrono::time_point mid = std::chrono::high_resolution_clock::now();
  //     std::unordered_set<Poset<globalMaxN>> result2 = find_solvable_posets(normalizer, n, k);
  //     std::chrono::time_point end = std::chrono::high_resolution_clock::now();
  //     std::cout << n << ", " << k << ": " << result.size() << " - " << result2.size() << " in: " << mid - start << "
  //     | "
  //               << end - mid << std::endl;
  //     // assert(result == result2);
  //   }
  // }

  constexpr size_t nBound = 7;

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  for (int n = 2; n < 15; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      const auto &[comparisons, durationGeneratePosets, durationSearch] =
          startSearchBackward<globalMaxN>(n, nthSmallest);

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