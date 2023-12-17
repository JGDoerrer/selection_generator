#pragma once
#include <bits/stdc++.h>

#include "normalizer.h"

template <size_t maxN>
class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;
  std::bitset<maxN * maxN> comparisonTable;

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
  Poset(const uint8_t n, const uint8_t nthSmallest) : n(n), nthSmallest(nthSmallest), comparisonTable() {}

  uint8_t size() const { return n; }

  uint8_t nth() const { return nthSmallest; }

  // `set_less(i, j)` and add all transitiv comparisons
  void addComparison(const uint16_t i, const uint16_t j) {
    addComparisonTransitivRecursive(i, j);  // faster than iterative
  }

  // checks, whether it holds `arr[i] < arr[j]`, e.g. `is_less(i, j) == true` => `arr[i] < arr[j]`
  // Attention: `!is_less(i, j)` IMPLIES NOT `arr[i] > arr[j]`
  inline bool is_less(const uint16_t i, const uint16_t j) const { return comparisonTable[i * n + j]; }

  inline void set_less(const uint16_t i, const uint16_t j, const bool value) { comparisonTable[i * n + j] = value; }

  // can one determine nthsmallest element with current comparisons
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

  inline bool isRedundant(const uint16_t i, const uint16_t j) const {
    for (uint16_t k = 0; k < n; ++k) {
      if (is_less(i, k) && is_less(k, j)) {
        return true;
      }
    }

    return false;
  }

  size_t count() const { return comparisonTable.count(); }

  // Definiere Poset `p` ist vollständig gdw. jede transitive '1' gesetzt ist (d.h. es ex. kein a, b, c mit
  //     `is_less(a, b) && is_less(b, c) && !is_less(a, c)`)
  // input: vollständiges Poset
  // output: Menge an Posets M, wobei für alle m in M gilt: m ist vollständig, NICHT normalisiert, aber dedupliziert,
  //         durch Aufruf von `m.addComparison(i, j)` erhält man wieder `*this` und keine unnötigen Vergleiche
  //         gespeichert haben
  inline std::unordered_set<Poset<maxN>> removeComparison(Normalizer<maxN> &normalizer, const uint16_t i,
                                                          const uint16_t j) const {
    // prüfe auf Vollständigkeit
    for (uint8_t in = 0; in < n; ++in) {
      for (uint8_t jn = 0; jn < n; ++jn) {
        for (uint16_t k = 0; k < n; ++k) {
          if (in != jn && jn != k && k != in) {
            if (is_less(in, jn) && is_less(jn, k) && !is_less(in, k)) {
              std::cout << (int)in << " " << (int)jn << " " << (int)k << std::endl;
              assert(false);
            }
          }
        }
      }
    }

    std::unordered_set<Poset<maxN>> result, resultNormalized;
    // siehe unten
    if (!is_less(i, j) || isRedundant(i, j)) {
      return result;
    }

    Poset<maxN> poset_initial = *this;
    poset_initial.set_less(i, j, false);

    std::queue<Poset<maxN>> queue{};
    queue.push(poset_initial);

    result.insert(poset_initial);
    normalizer.canonifyNauty(poset_initial);
    resultNormalized.insert(poset_initial);

    while (!queue.empty()) {
      Poset<maxN> poset = queue.front();
      queue.pop();

      for (uint8_t i1 = 0; i1 < n; ++i1) {
        for (uint8_t j1 = 0; j1 < n; ++j1) {
          // brich ab, wenn eine der folgenden Bedingungen entritt:
          // abs(j-i) >= abs(j1-i1): wenn das gilt, kann es sich um keinen transitiven Vergleich handeln? (Beweis: tbd)
          // !poset.is_less(i1, j1): der zu entfernende Vergleich existiert gar nicht
          // poset.isRedundant(i1, j1): wenn es sich bei (i1, j1) um eine redundante Kante handelt, muss dieser Pfad
          // auch nicht weiterverfolgt werden? (Beweis: tbd)
          if (i1 == j1 || abs(j - i) >= abs(j1 - i1) || !poset.is_less(i1, j1) || poset.isRedundant(i1, j1)) {
            continue;
          }

          Poset<maxN> poset_next = poset;
          poset_next.set_less(i1, j1, false);

          // wenn durch hinzufügen des ursprünglich entfernten Vergleichs, nicht wieder das ursprüngliche Poset
          // entsteht, brich ab
          Poset<maxN> poset_check = poset_next;
          poset_check.addComparison(i, j);
          if (*this != poset_check) {
            continue;
          }

          // wenn das poset normalisert schon in result ist, abbruch
          Poset<maxN> poset_norm = poset_next;
          normalizer.canonifyNauty(poset_norm);
          if (resultNormalized.contains(poset_norm)) {
            continue;
          }
          resultNormalized.insert(poset_norm);
          result.insert(poset_next);

          // mache sonst in den nächsten Schritten mit dem Poset weiter und versuche noch mehr Vergleiche zu entfernen
          queue.push(poset_next);
        }
      }
    }

    // for (auto item : result) {
    //   for (uint8_t in = 0; in < n; ++in) {
    //     for (uint8_t jn = 0; jn < n; ++jn) {
    //       for (uint16_t k = 0; k < n; ++k) {
    //         if (in != jn && jn != k && k != in) {
    //           if (item.is_less(in, jn) && item.is_less(jn, k) && !item.is_less(in, k)) {
    //             std::cout << (int)in << " " << (int)jn << " " << (int)k << std::endl;
    //             assert(false);
    //           }
    //         }
    //       }
    //     }
    //   }
    // }

    return result;
  }

  bool subset_of(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == poset.nthSmallest && (~comparisonTable | poset.comparisonTable).all();
  }

  // TODO: iterator for all "ones"

  bool operator==(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == poset.nthSmallest && comparisonTable == poset.comparisonTable;
  }

  size_t hash() const {
    const std::hash<std::bitset<maxN * maxN>> hash1;
    return ((size_t)n << (size_t)4) ^ nthSmallest ^ hash1(comparisonTable);
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