#include "main.h"

// Option 1: nur cache befüllen

// Option 2: suche alle posets, die folgendes erfüllen:
// für ein bel. k: (sei k = 0)
// es ex. `nthSmallest` versch. i mit : is_less(i, 0)
// es ex. `n - nthSmallest - 1` versch. i mit : is_less(0, i)

// Option 3: starte mit [1] und call anti_reduceN, anti_reduceNthsmallest

template <size_t maxN>
void findAllInitPosets(Normalizer<maxN> &normalizer, const Poset<maxN> &poset, std::unordered_set<Poset<maxN>> &posets,
                       std::unordered_set<Poset<maxN>> &cache) {
  for (int i = 0; i < poset.size(); ++i) {
    for (int j = 0; j < poset.size(); ++j) {
      if (!poset.is_less(i, j) && !poset.is_less(j, i) && i != j) {
        Poset<maxN> poset2 = poset;
        poset2.addComparison(i, j);
        normalizer.canonifyNauty(poset2);
        if (poset2.canDetermineNSmallest()) {
          posets.insert(poset2);
        } else if (!cache.contains(poset2)) {
          cache.insert(poset2);
          findAllInitPosets(normalizer, poset2, posets, cache);
        }
      }
    }
  }
}

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    const int n, const int nthSmallest) {
  const std::chrono::time_point start = std::chrono::high_resolution_clock::now();
  Normalizer<maxN> normalizer{};
  std::vector<std::unordered_set<Poset<maxN>>> container(n * n);

  if constexpr (false) {
    Poset<maxN> poset{(uint8_t)n, (uint8_t)nthSmallest};
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n; ++j) {
        poset.addComparison(i, j);
      }
    }
    normalizer.canonifyNauty(poset);
    container[0].insert(poset);
  } else {
    std::unordered_set<Poset<maxN>> cache;
    findAllInitPosets(normalizer, Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest}, container[0], cache);
  }
  std::chrono::time_point end = std::chrono::high_resolution_clock::now();

  std::unordered_set<Poset<maxN>> allPos;
  for (int k = 0; k < container.size(); ++k) {
    allPos.merge(std::unordered_set<Poset<maxN>>(container[k]));
    std::cout << k << ": " << container[k].size() << " " << allPos.size() << std::endl;
    if (allPos.contains(Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest})) {
      return {k, end - start, std::chrono::high_resolution_clock::now() - end};
    }

    for (const Poset<maxN> &item : container[k]) {
      std::unordered_set<Poset<maxN>> predecessors_set;  // global???
      for (uint8_t i = 0; i < n; ++i) {
        for (uint8_t j = 0; j < n; ++j) {
          if (item.is_less(i, j)) {
            for (const Poset<maxN> &predecessor : item.removeComparison(normalizer, i, j)) {
              if (!allPos.contains(predecessor) && !predecessors_set.contains(predecessor)) {
                predecessors_set.insert(predecessor);

                Poset<maxN> predecessorNorm = predecessor;
                predecessorNorm.addComparison(j, i);
                normalizer.canonifyNauty(predecessorNorm);
                if (allPos.contains(predecessorNorm)) {
                  container[k + 1].insert(predecessor);
                }
              }
            }
          }
        }
      }
    }
  }

  return {std::nullopt, end - start, std::chrono::high_resolution_clock::now() - end};
}

int main() {
  constexpr size_t nBound = 0;

  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  for (int n = 2; n < globalMaxN; ++n) {
    for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
      Statistics statistics;
      const auto &[comparisons, durationGeneratePosets, durationSearch] =
          startSearchBackward<globalMaxN>(n, nthSmallest);

      if (comparisons.has_value()) {
        if (n >= nBound) {
          std::cout << "\rtime '" << durationGeneratePosets << " + " << durationSearch << " = "
                    << durationGeneratePosets + durationSearch << "': n = " << n << ", i = " << nthSmallest << ", "
                    << statistics << ", comparisons: " << comparisons.value() << std::endl;
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
}