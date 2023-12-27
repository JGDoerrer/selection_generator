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

std::array<std::array<std::array<std::optional<std::unordered_set<Poset<globalMaxN>>>, globalMaxN>, globalMaxN>,
           globalMaxComparisons>
    myBigCache;

template <size_t maxN>
std::unordered_set<Poset<maxN>> find_solvable_posets(Normalizer<maxN> &normalizer, const uint8_t comparisons,
                                                     const uint8_t n, const uint8_t k) {
  assert(k < n);
  if (myBigCache[comparisons][n][k] != std::nullopt) {
    return myBigCache[comparisons][n][k].value();
  } else if (n <= 2 * k) {
    std::unordered_set<Poset<maxN>> result{};
    for (Poset<maxN> item : find_solvable_posets(normalizer, comparisons, n, n - k - 1)) {
      item.dual();
      normalizer.canonify_nauty(item);
      result.insert(item);
    }
    myBigCache[comparisons][n][k] = result;
    return myBigCache[comparisons][n][k].value();
  } else {
    if (0 == comparisons) {
      if (1 == n) {
        std::unordered_set<Poset<maxN>> result{};
        result.insert(Poset<maxN>{n, k});
        myBigCache[comparisons][n][k] = result;
        return myBigCache[comparisons][n][k].value();
      } else {
        myBigCache[comparisons][n][k] = enlarge(normalizer, find_solvable_posets(normalizer, comparisons, n - 1, k));
        return myBigCache[comparisons][n][k].value();
      }
    } else {
      if (true) {
        std::unordered_set<Poset<maxN>> allPos;
        for (int i = 0; i < comparisons; ++i) {
          allPos.merge(std::unordered_set<Poset<maxN>>(find_solvable_posets(normalizer, i, n, k)));
        }

        std::unordered_set<Poset<maxN>> result{};
        for (const Poset<maxN> &item : find_solvable_posets(normalizer, comparisons - 1, n, k)) {
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

                  result.insert(predecessorNorm0);
                }
              }
            }
          }
        }

        //         if (k < n - 1) {
        //           std::unordered_set<Poset<maxN>> allPos2;
        //           for (int i = 0; i < comparisons; ++i) {
        //             allPos2.merge(std::unordered_set<Poset<maxN>>(find_solvable_posets(normalizer, i, n, k)));
        //           }

        //           std::unordered_set<Poset<maxN>> result2{};
        //           for (const Poset<maxN> &item :
        //                enlarge(normalizer, find_solvable_posets(normalizer, comparisons - 1, n - 1, k))) {
        //             for (uint8_t i = 0; i < n; ++i) {
        //               for (uint8_t j = 0; j < n; ++j) {
        //                 if (item.is_less(i, j)) {
        //                   for (const Poset<maxN> &predecessor : item.remove_less(normalizer, i, j)) {
        //                     Poset<maxN> predecessorNorm0 = predecessor;
        //                     normalizer.canonify_nauty(predecessorNorm0);
        //                     if (allPos2.contains(predecessorNorm0)) {
        //                       continue;
        //                     }

        //                     Poset<maxN> predecessorNorm = predecessor;
        //                     predecessorNorm.add_less(j, i);
        //                     normalizer.canonify_nauty(predecessorNorm);
        //                     if (!allPos2.contains(predecessorNorm)) {
        //                       continue;
        //                     }

        //                     result2.insert(predecessorNorm0);
        //                   }
        //                 }
        //               }
        //             }
        //           }

        //           // auto temp0 = find_solvable_posets(normalizer, comparisons, n - 1, k);
        //           // auto temp = enlarge(normalizer, temp0);
        //           std::cout << (int)n << " " << (int)k << " " << (int)comparisons << " -> " << result2.size() << " "
        //                     << result.size() << std::endl;
        //           if (result2.size() != result.size()) {
        //             for (auto item : result) {
        //               if (!result2.contains(item)) {
        //                 std::cout << item << std::endl;
        //               }
        //             }
        //             for (auto item : find_solvable_posets(normalizer, comparisons - 1, n - 1, k))
        //               std::cout << item << std::endl;

        // 0 0 0
        // 0 0 0
        // 1 0 0

        // 0 < 2

        //             // for (auto item : result2) {
        //             //   std::cout << item << std::endl;
        //             // }
        //             // std::cout << std::endl;
        //             // for (auto item : result) {
        //             //   std::cout << item << std::endl;
        //             // }
        //           }
        //         }

        myBigCache[comparisons][n][k] = result;
        return myBigCache[comparisons][n][k].value();
        // std::unordered_set<Poset<maxN>> result{};
        // result.insert(Poset<maxN>{n, k});
        // myBigCache[comparisons][n][k] = result;
        // return myBigCache[comparisons][n][k].value();
      } else {
        // myBigCache[comparisons][n][k] = enlarge(normalizer, find_solvable_posets(normalizer, comparisons, n - 1, k));
        // return myBigCache[comparisons][n][k].value();
      }
    }
  }
}

// Ang. Poset enlarge(p)[0] kann nicht auf p reduziert werden

// std::unordered_map<Poset<globalMaxN>, std::unordered_set<Poset<globalMaxN>>> cache_solvable_in;

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    const uint8_t n, const uint8_t nthSmallest) {
  const std::chrono::time_point start = std::chrono::high_resolution_clock::now();

  Normalizer<maxN> normalizer{};
  std::unordered_set<Poset<maxN>> source;  // pqrc1 = find_solvable_posets(normalizer, 0, n, nthSmallest);
  std::unordered_set<Poset<maxN>> source_reduced{{Poset<globalMaxN>(1, 0)}};

  std::chrono::time_point end = std::chrono::high_resolution_clock::now();

  std::unordered_set<Poset<maxN>> allPos_reduced;
  // std::array<std::unordered_set<Poset<maxN>>, 10> container;
  for (int k = 1; k < n * n; ++k) {
    // container[k - 1] = source;
    // allPos.merge(std::unordered_set<Poset<maxN>>(source));

    // source = super_enlarge_to(source_reduced, n, k); // alle resultierenden Posets Größe n, k
    std::unordered_set<Poset<maxN>> source_new, source_old = source_reduced;
    while (0 != source_old.size()) {
      std::unordered_set<Poset<maxN>> remainder1;
      std::unordered_set<Poset<maxN>> remainder2;
      assert(2 * nthSmallest < n);
      for (auto item : source_old) {
        assert(item.size() <= n);
        assert(item.nth() < item.size());
        if (item.size() < n) {
          if (item.nth() == nthSmallest) {
            remainder1.insert(item);
          } else if (item.nth() < nthSmallest) {
            remainder2.insert(item);
          }
        } else if (item.size() == n && item.nth() == nthSmallest) {
          source_new.insert(item);
        }
      }
      // n = 8, k = 3->n = 8, k = 4 n = 8, k = 5->n = 8, k = 6->n = 8, k = 2
      source_old = enlarge(normalizer, remainder1);
      source_old.merge(enlarge2(normalizer, remainder2));
      // std::cout << source_old.size() << std::endl;

      // for (auto item : enlarge2(normalizer, remainder)) {
      //   auto tz = item;
      //   source_old.insert(item);

      //   normalizer.reduce_n(item);
      //   normalizer.canonify_nauty(item);
      //   bool suc = false;
      //   for (auto item2 : remainder) {
      //     normalizer.reduce_n(item2);
      //     normalizer.canonify_nauty(item2);
      //     if (item == item2) {
      //       suc = true;
      //     }
      //   }
      //   if (!suc) {
      //     std::cout << *remainder.begin() << std::endl;
      //     std::cout << tz << std::endl;
      //     exit(0);
      //     std::cout << "error" << std::endl;
      //   }
      // }
    }

    if (0 == source_new.size()) {
      for (auto item : source_reduced) std::cout << item << std::endl;
      std::cout << std::endl;
    }

    allPos_reduced.merge(std::unordered_set<Poset<maxN>>(source_reduced));
    std::cout << k - 1 << ": " << source.size() << " " << source_reduced.size() << " " << source_new.size() << " | "
              << allPos_reduced.size() << std::endl;

    // std::unordered_set<Poset<maxN>> destination; // pqrc1
    std::unordered_set<Poset<maxN>> destination_reduced;
    for (Poset<maxN> item : source_new) {
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
              if (predecessor == Poset<maxN>{(uint8_t)n, (uint8_t)nthSmallest}) {
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

              Poset<maxN> predecessorNorm0_reduced = predecessor;
              normalizer.reduce_n(predecessorNorm0_reduced);
              normalizer.canonify_nauty(predecessorNorm0_reduced);
              if (allPos_reduced.contains(predecessorNorm0_reduced)) {
                continue;
              }

              Poset<maxN> predecessorNorm_reduced = predecessor;
              predecessorNorm_reduced.add_less(j, i);
              normalizer.reduce_n(predecessorNorm_reduced);
              normalizer.canonify_nauty(predecessorNorm_reduced);
              if (!allPos_reduced.contains(predecessorNorm_reduced)) {
                continue;
              }

              destination_reduced.insert(predecessorNorm0_reduced);

              // Poset<maxN> predecessorNorm0 = predecessor; // pqrc1
              // normalizer.canonify_nauty(predecessorNorm0);
              // destination.insert(predecessorNorm0);

              // cache_solvable_in.insert(predecessorNorm0, k);
            }
          }
        }
      }
    }
    // source = destination; // pqrc1
    source_reduced = destination_reduced;
  }

  return {std::nullopt, end - start, std::chrono::high_resolution_clock::now() - end};
}

template <size_t maxN>
std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward2(
    const uint8_t n, const uint8_t nthSmallest) {
  Normalizer<maxN> normalizer{};
  const std::chrono::time_point start = std::chrono::high_resolution_clock::now();
  for (int i = 0; i < n * n; ++i) {
    if (find_solvable_posets(normalizer, i, n, nthSmallest)
            .contains(Poset<globalMaxN>{(uint8_t)n, (uint8_t)nthSmallest})) {
      const std::chrono::time_point end = std::chrono::high_resolution_clock::now();
      return {i, end - end, end - start};
    }
  }
  const std::chrono::time_point end = std::chrono::high_resolution_clock::now();
  return {std::nullopt, end - end, end - start};
}

template <size_t maxN>
std::unordered_set<Poset<maxN>> find_solvable_posets2(Normalizer<maxN> &normalizer, const uint8_t comparisons,
                                                      const uint8_t n, const uint8_t k) {
  assert(k < n);
  if (myBigCache[comparisons][n][k] != std::nullopt) {
    return myBigCache[comparisons][n][k].value();
  } else if (n <= 2 * k) {
    std::unordered_set<Poset<maxN>> result{};
    for (Poset<maxN> item : find_solvable_posets2(normalizer, comparisons, n, n - k - 1)) {
      item.dual();
      result.insert(item);
    }
    myBigCache[comparisons][n][k] = result;
    return myBigCache[comparisons][n][k].value();
  } else {
    if (0 == comparisons) {
      if (1 == n) {
        std::unordered_set<Poset<maxN>> result{};
        result.insert(Poset<maxN>{n, k});
        myBigCache[comparisons][n][k] = result;
        return myBigCache[comparisons][n][k].value();
      } else {
        myBigCache[comparisons][n][k] = enlarge(normalizer, find_solvable_posets2(normalizer, comparisons, n - 1, k));
        return myBigCache[comparisons][n][k].value();
      }
    } else {
      std::unordered_set<Poset<maxN>> allPos;
      for (int i = 0; i < comparisons; ++i) {
        allPos.merge(std::unordered_set<Poset<maxN>>(find_solvable_posets2(normalizer, i, n, k)));
      }

      std::unordered_set<Poset<maxN>> result{};
      for (const Poset<maxN> &item : find_solvable_posets2(normalizer, comparisons - 1, n, k)) {
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

                result.insert(predecessorNorm0);
              }
            }
          }
        }
      }

      myBigCache[comparisons][n][k] = result;
      return myBigCache[comparisons][n][k].value();
    }
  }
}

int main() {
  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  // auffällig:
  // (n, 2).size() == 2 * (n - 1, 1).size() == 2 * (n - 2, 0).size()

  // 10, 0: 0 - 183231 in: 0.000s | 2.145s
  // 10, 1: 0 - 16999 in: 0.000s | 0.152s
  // 10, 2: 0 - 4090 in: 0.000s | 0.034s
  // 10, 3: 0 - 1590 in: 0.000s | 0.012s
  // 10, 4: 0 - 1008 in: 0.000s | 0.005s

  // 11, 0: 0 - 2567284 in: 0.000s | 51.587s
  // 11, 1: 0 - 183231 in: 0.000s | 3.742s
  // 11, 2: 0 - 33998 in: 0.000s | 0.750s
  // 11, 3: 0 - 10225 in: 0.000s | 0.160s
  // 11, 4: 0 - 5088 in: 0.000s | 0.072s
  // 11, 5: 0 - 3969 in: 0.000s | 0.045s

  if constexpr (false) {
    Normalizer<globalMaxN> normalizer{};
    for (auto item : enlarge2(normalizer, std::unordered_set<Poset<globalMaxN>>{{Poset<globalMaxN>(2, 0)}})) {
      std::cout << item << std::endl;
    }
  } else if constexpr (false) {
    int globN = 7;
    const std::chrono::time_point start = std::chrono::high_resolution_clock::now();
    Normalizer<globalMaxN> normalizer{};
    std::unordered_set<Poset<globalMaxN>> comp[globalMaxComparisons];
    comp[0].insert(Poset<globalMaxN>(1, 0));
    std::cout << "comparisons = " << 0 << ": " << comp[0].size() << " " << 1 << std::endl;
    for (int i = 1; i < globalMaxComparisons; ++i) {
      int maxN1 = 0;
      for (int n = 1; n <= globN; ++n) {
        for (int k = 0; k < (n + 1) / 2; ++k) {
          for (auto item : find_solvable_posets2(normalizer, i, n, k)) {
            normalizer.reduce_n(item);
            normalizer.canonify_nauty(item);
            bool cont = false;
            for (int j = 0; j < i; ++j) {
              if (comp[j].contains(item)) cont = true;
            }
            if (!cont) {
              maxN1 = std::max(maxN1, (int)item.size());
              comp[i].insert(item);
            }
          }
        }
      }
      // std::cout << std::endl;
      std::cout << "comparisons = " << i << ": " << comp[i].size() << " " << maxN1 << std::endl;
      if (0 == comp[i].size()) break;
      // for (auto item : comp[i]) std::cout << item << std::endl;
    }

    for (int n = 1; n <= globN; ++n) {
      for (int k = 0; k < (n + 1) / 2; ++k) {
        for (int i = 0; i < globalMaxComparisons; ++i) {
          int found = -1;
          if (comp[i].contains(Poset<globalMaxN>(n, k))) {
            found = i;
          }
          if (-1 != found) {
            assert(min_n_comparisons[n][k] == i);
            // std::cout << "n = " << n << ", k = " << k << ": " << i << std::endl;
          }
        }
      }
    }
    const std::chrono::time_point end = std::chrono::high_resolution_clock::now();
    std::cout << "success in " << end - start << std::endl;

    std::unordered_set<Poset<globalMaxN>> allPos;
    allPos.merge(comp[0]);
    allPos.merge(comp[1]);
    allPos.merge(comp[2]);

    auto A = comp[3];
    for (int i = 0; i < globN; ++i)
      for (auto item : enlarge(normalizer, A)) {
        if (item.size() <= globN) {
          A.insert(item);
        }
      }
    std::unordered_set<Poset<globalMaxN>> B;

    for (const Poset<globalMaxN> &item : A) {
      for (uint8_t i = 0; i < item.size(); ++i) {
        for (uint8_t j = 0; j < item.size(); ++j) {
          if (item.is_less(i, j)) {
            for (const Poset<globalMaxN> &predecessor : item.remove_less(normalizer, i, j)) {
              Poset<globalMaxN> predecessorNorm0 = predecessor;
              normalizer.canonify_nauty(predecessorNorm0);
              if (allPos.contains(predecessorNorm0)) {
                continue;
              }

              Poset<globalMaxN> predecessorNorm = predecessor;
              predecessorNorm.add_less(j, i);
              normalizer.canonify_nauty(predecessorNorm);
              if (!allPos.contains(predecessorNorm)) {
                continue;
              }

              B.insert(predecessorNorm0);
            }
          }
        }
      }
    }

    std::cout << A.size() << " " << B.size() << std::endl;
  } else if constexpr (false) {
    Normalizer<globalMaxN> normalizer{};
    std::chrono::time_point start = std::chrono::high_resolution_clock::now();

    int n = 3;
    int nthSmallest = 1;
    find_solvable_posets(normalizer, 2, n, nthSmallest);
    // const auto &[comparisons, durationGeneratePosets, durationSearch] =
    //     startSearchBackward2<globalMaxN>(n, nthSmallest);

    std::chrono::time_point end = std::chrono::high_resolution_clock::now();
    // std::cout << (comparisons.has_value() ? comparisons.value() : -1) << " " << (end - start) << std::endl;
  } else if constexpr (false) {
    Normalizer<globalMaxN> normalizer{};

    for (int n = 1; n < 12; ++n) {
      for (int k = 0; k < (n + 1) / 2; ++k) {
        std::chrono::time_point start = std::chrono::high_resolution_clock::now();
        std::unordered_set<Poset<globalMaxN>> result;  // = find_solvable_posets(normalizer, n, k);
        std::chrono::time_point mid = std::chrono::high_resolution_clock::now();
        std::unordered_set<Poset<globalMaxN>> result2 = find_solvable_posets(normalizer, 0, n, k);
        std::chrono::time_point end = std::chrono::high_resolution_clock::now();
        std::cout << n << ", " << k << ": " << result.size() << " - " << result2.size() << " in: " << mid - start
                  << " | " << end - mid << std::endl;
        // assert(result == result2);
      }
    }
  } else {
    constexpr size_t nBound = 0;

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
}