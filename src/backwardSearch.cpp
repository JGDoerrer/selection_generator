#include <bits/stdc++.h>

#include "cache.h"
#include "poset.h"
#include "util.h"
// ===================
#include "normalizer.h"

constexpr size_t globalMaxN = 15;
std::array<Normalizer<globalMaxN>, 230> norm;  // TODO: only Debug

// ACHTUNG: bis jetzt nur ein Pfad, aber alle nötig
// void findEquivalents2(const Poset<globalMaxN> &poset, std::unordered_set<Poset<globalMaxN>> &result, const bool
// begin) {
//   for (int i = 0; i < poset.size(); ++i) {
//     for (int j = 0; j < poset.size(); ++j) {
//       if (poset.is_less(i, j)) {
//         if (poset.isRedundant(i, j) || (begin && poset.canDetermineNSmallestWOTransitiv(i, j))) {
//           Poset<globalMaxN> poset2 = poset;
//           poset2.removeComparison(i, j);
//           norm[0].normalize(poset2);

//           if (!result.contains(poset2)) {
//             result.insert(poset2);
//             findEquivalents2(poset2, result, begin);
//           }
//         }
//       }
//     }
//   }
// }

int main() {
  int k = 0;  // sollte unabhängig von k sein
  for (int n = 2; n < 8; ++n) {
    Poset<globalMaxN> poset{(uint8_t)n, (uint8_t)k};
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n; ++j) {
        poset.addComparison(i, j);
      }
    }
    // poset.addComparison(0, 2);
    // poset.addComparison(4, 5);
    // poset.addComparison(3, 5);
    // poset.addComparison(1, 3);
    // poset.addComparison(3, 4);

    // Problem: berechne alle Posets, die am "ende" rauskommen

    // std::cout << poset << std::endl << std::endl;

    // poset.addComparison(2, 3);

    // std::cout << poset << std::endl << std::endl;

    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (poset.is_less(i, j)) {
          std::unordered_set<Poset<globalMaxN>> possible_solutions = poset.removeComparison_slow(i, j);
          std::unordered_set<Poset<globalMaxN>> possible_solutions2 = poset.removeComparison(i, j);
          std::cout << "n = " << n << " -> i = " << i << ", j = " << j << ": " << possible_solutions.size()
                    << std::endl;
          if (possible_solutions != possible_solutions2) {
            for (auto item : possible_solutions) {
              std::cout << item << std::endl;
              item.addComparison(i, j);
              assert(item == poset);
            }
            std::cout << std::endl;
            std::cout << std::endl;
            for (auto item : possible_solutions2) {
              std::cout << item << std::endl;
              item.addComparison(i, j);
              assert(item == poset);
            }
            std::cout << possible_solutions.size() << " " << possible_solutions2.size() << std::endl;
          }
          assert(possible_solutions == possible_solutions2);
        }
      }
    }

    // std::cout << std::endl;
  }
  std::cout << "success" << std::endl;
}