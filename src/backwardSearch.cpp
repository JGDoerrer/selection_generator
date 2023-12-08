#include <bits/stdc++.h>

#include "BS_thread_pool.hpp"
#include "cache.h"
#include "poset.h"
#include "util.h"
// ===================
#include "normalizer.h"

// TODO: multithreading implementieren
// TODO: multithreading nauty testen
// TODO: ACHTUNG: post_normalize kaputt

// TODO: swap Operationen, memcpy, fine-tuning
// TODO: überall explicit static cast

constexpr size_t globalMaxComparisons = 25;
constexpr size_t globalMaxN = 15;
constexpr size_t threadCount = 20;
constexpr bool SORT_DFS_BRANCHES = true;
constexpr bool TOP_TO_BOTTOM_SEARCH = false;

std::array<Normalizer<globalMaxN>, 230> norm;  // TODO: only Debug

// siehe Section 5.3.3
// https://doc.lagout.org/science/0_Computer%20Science/2_Algorithms/The%20Art%20of%20Computer%20Programming%20%28vol.%203_%20Sorting%20and%20Searching%29%20%282nd%20ed.%29%20%5BKnuth%201998-05-04%5D.pdf
int getUpperBound(const int n, int t) {
  t += 1;  // offset in Programm vs Paper
  if (t > (n + 1) / 2) {
    throw std::invalid_argument("it should hold t <= (n + 1) / 2");
  }
  if (1 == t) {
    // page 209, (5)
    return n - 1;  // exact
  } else if (2 == t) {
    // page 209, Threorem S.
    return n - 2 + ceil(log2(n));  // exact
  } else if (3 == t) {
    // page 213, (13)
    return n + 1 + ceil(log2((n - 1) / 4.0f)) + ceil(log2((n - 1) / 5.0f));
  }
  // page 210, (6); not used, because useless for n <= 15 (not fine enough)

  // page 212, (11)
  const int res1 = n - t + (t - 1) * ceil(log2(n + 2 - t));

  // page 212, (11); mit t = n + 1 - t => gibt evtl. bessere Werte
  const int res2 = n - (n + 1 - t) + ((n + 1 - t) - 1) * ceil(log2(n + 2 - (n + 1 - t)));
  return std::min(res1, res2);
}

// TODO: untested
double nCr(const int n, int k) {
  double result = 1;
  for (; k > 0; --k) {
    result *= (n - k + 1) / double(k);
  }
  return result;
}

int getLowerBound(const int n, int t) {
  t += 1;  // offset in Programm vs Paper
  if (t > (n + 1) / 2) {
    throw std::invalid_argument("it should hold t <= (n + 1) / 2");
  }
  if (1 == t) {
    // page 209, (5)
    return n - 1;  // exact
  } else if (2 == t) {
    // page 209, Threorem S.
    return n - 2 + ceil(log2(n));  // exact
  }
  // page 213, (12)
  int sum = 0;
  for (int j = 0; j <= t - 2; ++j) {
    sum += ceil(log2((n - t + 2) / float(t + j)));
  }
  return n + t - 3 + sum;  // lower

  // page 213, (14); not used, because useless for n <= 15 (not fine enough)
}

// int main() {
//   for (int n = 0; n <= 15; ++n) {
//     for (int t = 0; t < (n + 1) / 2; ++t) {
//       const int lower = getLowerBound(n, t);
//       const int upper = getUpperBound(n, t);
//       if (lower < upper) {
//         std::cout << "n = " << n << ", t = " << t << ": ";
//         std::cout << lower << " - " << upper;
//         std::cout << std::endl;
//       } else {
//         std::cout << "n = " << n << ", t = " << t << ": ";
//         std::cout << lower;
//         std::cout << std::endl;
//       }
//     }
//   }
// }

// ACHTUNG: bis jetzt nur ein Pfad, aber alle nötig
// void findEquivalents2(const Poset<globalMaxN> &poset, std::unordered_set<Poset<globalMaxN>> &result, const bool
// begin) {
//   for (int i = 0; i < poset.size(); ++i) {
//     for (int j = 0; j < poset.size(); ++j) {
//       if (poset.is(i, j)) {
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
        if (poset.is(i, j)) {
          std::unordered_set<Poset<globalMaxN>> possible_solutions = poset.removeComparison(i, j);
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