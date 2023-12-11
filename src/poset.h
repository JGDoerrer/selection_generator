#pragma once
#include <bits/stdc++.h>

#include "normalizer.h"

template <size_t maxN>
class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;
  std::vector<bool> comparisonTable;

  inline void set_less(const uint16_t i, const uint16_t j, const bool value) { comparisonTable[i * n + j] = value; }

  void addComparisonTransitivRecursive(const uint16_t i, const uint16_t j) {
    if (false == is_less(i, j)) {
      set_less(i, j, true);

      for (uint16_t k = 0; k < n; ++k) {
        if (i != k && j != k) {
          if (is_less(j, k) && !is_less(i, k)) {
            addComparisonTransitivRecursive(i, k);
          } else if (is_less(k, i) && !is_less(k, j)) {
            addComparisonTransitivRecursive(k, j);
          }
        }
      }
    }
  }

  inline void addComparisonTransitivIterative(const uint16_t i, const uint16_t j) {
    std::queue<std::pair<uint16_t, uint16_t>> queue;
    queue.push({i, j});
    while (0 != queue.size()) {
      const auto &[i1, j1] = queue.front();
      queue.pop();

      if (false == is_less(i1, j1)) {
        set_less(i1, j1, true);
        for (uint16_t k = 0; k < n; ++k) {
          if (is_less(j1, k) && !is_less(i1, k)) {
            queue.push({i1, k});
          } else if (is_less(k, i1) && !is_less(k, j1)) {
            queue.push({k, j1});
          }
        }
      }
    }
  }

  // Kante von (v) -> (w) g.d.w. a[v] < a[w]
  void visit(const int v, std::vector<bool> &visited) const {
    visited[v] = true;
    for (uint8_t w = 0; w < n; ++w) {
      if ((is_less(v, w) || is_less(w, v)) && !visited[w]) {
        visit(w, visited);
      }
    }
  }

  uint8_t numberConnectedComponents() const {
    std::vector<bool> visited(n, false);
    uint8_t components = 0;
    for (uint8_t v = 0; v < n; ++v) {
      if (!visited[v]) {
        ++components;
        visit(v, visited);
      }
    }
    return components;
  }

 public:
  inline bool isRedundant(const uint16_t i, const uint16_t j) const {
    for (uint16_t k = 0; k < n; ++k) {
      if (is_less(i, k) && is_less(k, j)) {
        return true;
      }
    }

    return false;
  }

  // invariant: Sei M = poset.removeComparison(i, j), dann gilt für alle m in M : m.addComparison(i, j) == poset
  // speichert in `result` alle möglichen Posets, die nach Aufruf von `addComparison(i, j)`, dem aktuellen entsprechen
  // TODO: dedupliziere Posets (normalisieren)
  inline std::unordered_set<Poset<maxN>> removeComparison_slow(const uint16_t i, const uint16_t j) const {
    std::unordered_set<Poset<maxN>> result;
    std::queue<std::tuple<uint16_t, uint16_t, Poset<maxN>>> queue;
    queue.push({i, j, *this});
    while (0 != queue.size()) {
      auto [i_remove, j_remove, poset] = queue.front();
      queue.pop();

      if (poset.is_less(i_remove, j_remove)) {
        poset.set_less(i_remove, j_remove, false);

        // füge item zur Ergebnismenge hinzu, wenn ALLE vorhandenen relationen transitiv keine nicht vorhandenen
        // Relationen erzeugen
        bool success = true;
        for (uint8_t in = 0; in < n; ++in) {
          for (uint8_t jn = 0; jn < n; ++jn) {
            for (uint16_t k = 0; k < n; ++k) {
              if (poset.is_less(in, jn) && poset.is_less(jn, k) && !poset.is_less(in, k)) {
                success = false;
              }
            }
          }
        }
        if (success) {
          result.insert(poset);
        }

        for (uint8_t in = 0; in < n; ++in) {
          for (uint8_t jn = 0; jn < n; ++jn) {
            if (poset.is_less(in, jn)) {
              Poset<maxN> poset3 = poset;
              poset3.set_less(in, jn, false);
              poset3.addComparison(i, j);
              if (*this == poset3) {
                queue.push({in, jn, poset});
              }
            }
          }
        }
      }
    }
    return result;
  }

  inline std::unordered_set<Poset<maxN>> removeComparison(const uint16_t i, const uint16_t j) const {
    std::unordered_set<Poset<maxN>> result;
    if (!is_less(i, j)) {
      return result;
    }
    for (uint8_t in = 0; in < n; ++in) {
      for (uint8_t jn = 0; jn < n; ++jn) {
        if (is_less(in, jn)) {
          for (uint16_t k = 0; k < n; ++k) {
            assert(!(is_less(jn, k) && !is_less(in, k)));
            assert(!(is_less(k, in) && !is_less(k, jn)));
          }
        }
      }
    }

    std::unordered_set<Poset<maxN>> temp;
    std::queue<std::tuple<uint16_t, uint16_t, Poset<maxN>>> queue;
    {
      Poset<maxN> poset = *this;
      poset.set_less(i, j, false);
      temp.insert(poset);

      bool success = true;
      for (uint8_t k = 0; k < n; ++k) {
        if (poset.is_less(i, k) && poset.is_less(k, j)) {
          success = false;
          break;
        }
      }
      if (success) {
        result.insert(poset);
      }

      for (uint16_t k = 0; k < n; ++k) {
        if (poset.is_less(j, k) && poset.is_less(i, k)) {
          queue.push({i, k, poset});
        } else if (poset.is_less(k, i) && poset.is_less(k, j)) {
          queue.push({k, j, poset});
        }
      }
    }
    while (0 != queue.size()) {
      auto [i_removed, j_removed, poset] = queue.front();
      queue.pop();

      // füge item zur Ergebnismenge hinzu, wenn ALLE vorhandenen relationen transitiv keine nicht vorhandenen
      // Relationen erzeugen
      bool success = true;
      for (uint8_t k = 0; k < n; ++k) {
        if (poset.is_less(i_removed, k) && poset.is_less(k, j_removed)) {
          success = false;
        }
      }
      for (uint8_t in = 0; in < n && success; ++in) {
        for (uint8_t jn = 0; jn < n && success; ++jn) {
          if (poset.is_less(in, jn)) {
            for (uint16_t k = 0; k < n && success; ++k) {
              if (poset.is_less(jn, k) && !poset.is_less(in, k)) {
                success = false;
              }
            }
          }
        }
      }
      if (success) {
        result.insert(poset);
      }

      for (uint8_t in = 0; in < n; ++in) {
        for (uint8_t jn = 0; jn < n; ++jn) {
          if (poset.is_less(in, jn)) {
            Poset<maxN> poset3 = poset;
            poset3.set_less(in, jn, false);
            poset3.addComparison(i, j);
            if (*this == poset3) {
              Poset<maxN> poset2 = poset;
              poset2.set_less(in, jn, false);
              if (!temp.contains(poset2)) {
                temp.insert(poset2);
                queue.push({in, jn, poset2});
              }
            }
          }
        }
      }
    }
    return result;
  }

  // inline void removeAllTransitivComparisonsRecursiv(Normalizer<maxN> &normalizer,
  //                                                   std::unordered_set<Poset<maxN>> &result) const {
  //   for (uint8_t i = 0; i < n; ++i) {
  //     for (uint8_t j = 0; j < n; ++j) {
  //       if (is_less(i, j) && isRedundant(i, j)) {
  //         Poset<maxN> poset2 = *this;
  //         poset2.set_less(i, j, false);
  //         normalizer.normalize(poset2);

  //         if (!result.contains(poset2)) {
  //           result.insert(poset2);
  //           removeAllTransitivComparisonsRecursiv(normalizer, result);
  //         }
  //       }
  //     }
  //   }
  // }

  // // inline bool canDetermineNSmallestWOTransitiv(const uint16_t i, const uint16_t j) const {
  // //   Poset temp{n, nthSmallest};
  // //   for (int i1 = 0; i1 < n; ++i1) {
  // //     for (int j1 = 0; j1 < n; ++j1) {
  // //       if (is_less(i1, j1) && (i != i1 || j != j1)) {
  // //         temp.addComparison(i1, j1);
  // //       }
  // //     }
  // //   }
  // //   return temp.canDetermineNSmallest();
  // // }

  // iterator for all "ones"

  Poset(const uint8_t n, const uint8_t nthSmallest) : n(n), nthSmallest(nthSmallest), comparisonTable(n * n, false) {}

  // after func it holds: arr[i] < arr[j]
  void addComparison(const uint16_t i, const uint16_t j) {
    addComparisonTransitivRecursive(i, j);  // faster than iterative
  }

  uint8_t size() const { return n; }

  uint8_t nth() const { return nthSmallest; }

  // true => arr[i] < arr[j]
  // ACHTUNG: is_less(a, b) == false IMPLIZIERT NICHT, dass arr[i] > arr[j]:
  // nur wenn arr[i] > arr[j] => false
  inline bool is_less(const uint16_t i, const uint16_t j) const { return comparisonTable[i * n + j]; }

  // can one determine n smallest element with current comparisons
  bool canDetermineNSmallest() const {
    if (0 == n) {
      return true;
    }
    for (uint16_t k = 0; k < n; ++k) {  // guess arr[k] is_less median
      uint8_t smaller = 0, bigger = 0;
      for (uint16_t i = 0; i < n; ++i) {
        if (is_less(i, k)) {
          ++smaller;
        } else if (is_less(k, i)) {
          ++bigger;
        }
      }
      if (1 + smaller + bigger == n && smaller == nthSmallest) {
        return true;
      }
    }
    return false;
  }

  inline bool hasEnoughComparisons(const uint8_t remainingComparisons) const {
    if (0 == remainingComparisons) {
      return false;
    }
    // very rarely used, senseless???
    if (remainingComparisons + 1 < numberConnectedComponents()) {
      return false;
    }
    return true;
  }

  // how many elements are less than it
  void getLessGreater(uint8_t less[], uint8_t greater[]) const {
    std::memset(less, 0, n);
    std::memset(greater, 0, n);
    for (uint8_t i = 0; i < n; ++i) {
      for (uint8_t j = 0; j < n; ++j) {
        if (is_less(i, j)) {
          ++less[j];
          ++greater[i];
        }
      }
    }
  }

  bool operator==(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == poset.nthSmallest && comparisonTable == poset.comparisonTable;
  }

  size_t hash() const {
    constexpr int shift = 7;
    size_t result = 0;
    result ^= n;
    result = (result << shift) | (result >> (8 * sizeof(size_t) - shift));
    result ^= nthSmallest;
    for (uint16_t i = 0; i < n * n; ++i) {
      if (comparisonTable[i]) {
        result = (result << shift) | (result >> (8 * sizeof(size_t) - shift));
        result ^= i;
      }
    }
    return result;
  }

  template <size_t maxN2>
  friend std::ostream &operator<<(std::ostream &os, const Poset<maxN2> &poset);

  friend class Normalizer<maxN>;
};

template <size_t maxN>
struct std::hash<Poset<maxN>> {
  size_t operator()(const Poset<maxN> &poset) const { return poset.hash(); }
};

template <size_t maxN>
std::ostream &operator<<(std::ostream &os, const Poset<maxN> &poset) {
  os << "n = " << (uint16_t)poset.n << ", nthSmallest = " << (uint16_t)poset.nthSmallest;
  for (uint16_t i = 0; i < poset.n; ++i) {
    std::cout << '\n';
    for (uint16_t j = 0; j < poset.n; ++j) {
      std::cout << poset.is_less(i, j) << " ";
    }
  }
  return os;
}