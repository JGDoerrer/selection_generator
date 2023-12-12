#include <bits/stdc++.h>

#include "cache.h"
#include "poset.h"
#include "util.h"
// ===================
#include "normalizer.h"

enum SearchResult : uint8_t { FoundSolution, NoSolution, Unknown };

// Attention: not working
template <size_t maxN>
SearchResult search(Normalizer<maxN> &normalizer, const Poset<maxN> &poset, const uint8_t remainingComparisons) {
  const Poset<maxN> poset_empty{poset.size(), poset.nth()};
  if (poset == poset_empty) {
    return FoundSolution;
  } else if (0 == remainingComparisons) {
    return NoSolution;
  }

  std::unordered_set<Poset<maxN>> predecessors;
  for (int i = 0; i < poset.size(); ++i) {
    for (int j = 0; j < poset.size(); ++j) {
      if (poset.is_less(i, j)) {
        std::unordered_set<Poset<maxN>> possible_solutions = poset.removeComparison(normalizer, i, j);
        predecessors.merge(possible_solutions);
      }
    }
  }

  SearchResult result = NoSolution;
  for (const Poset<maxN> &predecessor : predecessors) {
    if (FoundSolution == search(normalizer, predecessor, remainingComparisons - 1)) {
      return FoundSolution;
    }
  }

  return result;
}

template <size_t maxN>
std::optional<int> startSearchBackward(const int n, const int k) {
  Normalizer<maxN> normalizer{};
  for (int maxDepth = n - 1; maxDepth < n * n; ++maxDepth) {
    Poset<maxN> poset{(uint8_t)n, (uint8_t)k};
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n; ++j) {
        poset.addComparison(i, j);
      }
    }
    if (FoundSolution == search(normalizer, poset, maxDepth)) {
      std::cout << "found solution with maxDepth = " << maxDepth << std::endl;
      return maxDepth;
    } else {
      std::cout << "found nix with maxDepth = " << maxDepth << std::endl;
    }
  }
  return {};
}

template <size_t maxN>
void test(Poset<maxN> &poset, const bool debug) {
  Normalizer<maxN> normalizer{};
  normalizer.canonifyNauty(poset);

  if (debug) std::cout << poset << std::endl;

  std::unordered_set<Poset<maxN>> allPos;
  for (int i = 0; i < poset.size(); ++i) {
    for (int j = 0; j < poset.size(); ++j) {
      if (poset.is_less(i, j)) {
        std::unordered_set<Poset<maxN>> possible_solutions = poset.removeComparison(normalizer, i, j);
        if (!possible_solutions.empty()) {
          if (debug)
            std::cout << "n = " << (int)poset.size() << ": (" << i << ", " << j << ") -> " << possible_solutions.size()
                      << std::endl;

          for (auto item : possible_solutions) {
            if (debug) std::cout << item << std::endl;
            bool success = false;
            for (int i1 = 0; i1 < poset.size() && !success; ++i1) {
              for (int j1 = 0; j1 < poset.size() && !success; ++j1) {
                if (i1 != j1) {
                  Poset<maxN> copy = item;
                  copy.addComparison(i1, j1);
                  normalizer.canonifyNauty(copy);
                  if (copy == poset) {
                    success = true;
                  }
                }
              }
            }
            assert(success);
          }

          allPos.merge(possible_solutions);
        }
      }
    }
  }
  std::cout << "for n = " << (int)poset.size() << ": " << poset.countNumberOfOnes() << " -> " << allPos.size()
            << std::endl;
}

template <size_t maxN>
int main_template() {
  int k = 0;  // sollte unabhängig von k sein
  if constexpr (false) {
    for (int n = 2; n < 16; ++n) {
      Poset<maxN> poset{(uint8_t)n, (uint8_t)k};
      for (int i = 0; i < n; ++i) {
        for (int j = i + 1; j < n; ++j) {
          poset.addComparison(i, j);
        }
      }

      test(poset, false);
    }
  } else if constexpr (false) {
    int n = 6;
    Poset<maxN> poset{(uint8_t)n, (uint8_t)k};
    poset.addComparison(0, 2);
    // poset.addComparison(1, 3);
    poset.addComparison(3, 4);
    // poset.addComparison(3, 5);
    poset.addComparison(4, 5);

    poset.addComparison(2, 3);

    test(poset, true);
  } else {
    startSearchBackward<maxN>(5, 2);
  }

  std::cout << "success" << std::endl;
  return 0;
}

int main() { return main_template<15>(); }