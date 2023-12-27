#pragma once
#include <bits/stdc++.h>

#include "normalizer.h"

template <std::size_t maxN>
class Poset {
 private:
 public:
  uint8_t n;
  uint8_t nthSmallest;

  std::bitset<maxN * maxN> comparisonTable;

 private:
  inline void set_less(const uint16_t i, const uint16_t j, const bool value) {
    this->comparisonTable[i * n + j] = value;
  }

  Poset<maxN> &add_and_close_recursive(const uint16_t i, const uint16_t j) {
    if (false == this->is_less(i, j)) {
      this->set_less(i, j, true);

      for (uint16_t k = 0; k < this->n; ++k) {
        if (i != k && j != k) {
          if (this->is_less(j, k) && !this->is_less(i, k)) {
            this->add_and_close_recursive(i, k);
          } else if (this->is_less(k, i) && !this->is_less(k, j)) {
            this->add_and_close_recursive(k, j);
          }
        }
      }
    }
    return *this;
  }

  inline Poset<maxN> &add_and_close_iterative(const uint16_t i, const uint16_t j) {
    std::queue<std::pair<uint16_t, uint16_t>> queue;
    queue.push({i, j});
    while (!queue.empty()) {
      const auto &[i1, j1] = queue.front();
      queue.pop();

      if (false == this->is_less(i1, j1)) {
        this->set_less(i1, j1, true);
        for (uint16_t k = 0; k < this->n; ++k) {
          if (this->is_less(j1, k) && !this->is_less(i1, k)) {
            queue.push({i1, k});
          } else if (this->is_less(k, i1) && !this->is_less(k, j1)) {
            queue.push({k, j1});
          }
        }
      }
    }
    return *this;
  }

  // Kante von (v) -> (w) g.d.w. a[v] < a[w]
  void visit(const int v, std::vector<bool> &visited) const {
    visited[v] = true;
    for (uint8_t w = 0; w < this->n; ++w) {
      if ((this->is_less(v, w) || this->is_less(w, v)) && !visited[w]) {
        this->visit(w, visited);
      }
    }
  }

  inline uint8_t count_connected_components() const {
    std::vector<bool> visited(this->n, false);
    uint8_t components = 0;
    for (uint8_t v = 0; v < this->n; ++v) {
      if (!visited[v]) {
        ++components;
        this->visit(v, visited);
      }
    }
    return components;
  }

  bool is_closed() const {  // only debug
    for (uint8_t in = 0; in < n; ++in) {
      for (uint8_t jn = 0; jn < n; ++jn) {
        for (uint16_t k = 0; k < n; ++k) {
          if (in != jn && jn != k && k != in) {
            if (is_less(in, jn) && is_less(jn, k) && !is_less(in, k)) {
              return false;
            }
          }
        }
      }
    }
    return true;
  }

  void generate_permutations(Normalizer<maxN> &normalizer, const int index,
                             std::unordered_set<Poset<maxN>> &result) const {
    if (!result.contains(*this)) {
      result.insert(*this);

      if (index < this->n - 1) {
        for (int index1 = index; index1 < (int)(this->n) - 1; ++index1) {
          if (!this->is_less(index1, this->n - 1) && !this->is_less(this->n - 1, index1)) {
            this->with_less(index1, this->n - 1).generate_permutations(normalizer, index1 + 1, result);
            this->with_less(this->n - 1, index1).generate_permutations(normalizer, index1 + 1, result);
          }
        }
      }
    }
  }

 public:
  // returns all canonified Posets, which can be builded from *this
  std::unordered_set<Poset<maxN>> enlarge(Normalizer<maxN> &normalizer) const {
    Poset<maxN> temp{uint8_t(uint8_t(n) + uint8_t(1)), nthSmallest};
    for (uint8_t i = 0; i < n; ++i) {
      for (uint8_t j = 0; j < n; ++j) {
        temp.set_less(i, j, is_less(i, j));
      }
    }
    std::unordered_set<Poset<maxN>> result2;
    temp.generate_permutations(normalizer, 0, result2);

    std::unordered_set<Poset<maxN>> result;
    for (Poset<maxN> item : result2) {
      normalizer.canonify_nauty(item);
      result.insert(item);
    }
    return result;
  }

  /// @brief constructs an empty Poset
  /// @param n
  /// @param nthSmallest
  Poset(const uint8_t n, const uint8_t nthSmallest) : n(n), nthSmallest(nthSmallest), comparisonTable() {}

  /// @brief clones a poset
  /// @param poset
  Poset(const Poset<maxN> &poset)
      : n(poset.n), nthSmallest(poset.nthSmallest), comparisonTable(poset.comparisonTable) {}

  /// @return size of the Poset
  inline uint8_t size() const { return this->n; }

  /// @return the nthSmallest Element
  inline uint8_t nth() const { return this->nthSmallest; }

  /// @brief checks, whether it holds `arr[i] < arr[j]`, e.g. `is_less(i, j) == true` => `arr[i] < arr[j]`
  //         Attention: `!is_less(i, j)` IMPLIES NOT `arr[i] > arr[j]`
  /// @param i
  /// @param j
  /// @return
  inline bool is_less(const uint16_t i, const uint16_t j) const { return this->comparisonTable[i * n + j]; }

  /// @brief
  /// @return how many comparisons are set (including transitiv)
  std::size_t count() const { return this->comparisonTable.count(); }

  /// @brief adds i < j to the poset
  /// @param i
  /// @param j
  /// @return
  inline Poset<maxN> &add_less(const uint16_t i, const uint16_t j) {
    return this->add_and_close_recursive(i, j);  // faster than iterative
  }

  /// @brief
  /// @param i
  /// @param j
  /// @return retunrs a clone of the poset, with i < j added
  inline Poset<maxN> with_less(const uint16_t i, const uint16_t j) const { return Poset<maxN>{*this}.add_less(i, j); }

  /// @brief
  /// @return true, if can one determine nthsmallest element with current comparisons
  bool is_solvable() const {
    if (0 == this->n) {
      return true;
    }
    for (uint16_t k = 0; k < this->n; ++k) {  // guess arr[k] is_less median
      uint8_t smaller = 0, bigger = 0;
      for (uint16_t i = 0; i < this->n; ++i) {
        if (this->is_less(i, k)) {
          ++smaller;
        } else if (this->is_less(k, i)) {
          ++bigger;
        }
      }
      if (1 + smaller + bigger == this->n && smaller == this->nthSmallest) {
        return true;
      }
    }
    return false;
  }

  /// @brief
  /// @param remainingComparisons
  /// @return true if poset is definitely not solvable in `remainingComparisons` Comparisons
  inline bool is_not_solvable_in(const uint8_t remainingComparisons) const {
    if (0 == remainingComparisons) {
      return true;
    }
    // // very rarely used, senseless???
    if (remainingComparisons + 1 < this->count_connected_components()) {
      return true;
    }
    return false;
    // return !this->is_solvable_in(remainingComparisons);
  }

  // NOT CHECKED, adapted from rust
  inline bool is_solvable_in(uint8_t max_comparisons) const {
    if (0 == this->nthSmallest || this->nthSmallest == this->n - 1) {
      return this->n - 1 <= max_comparisons;
    } else if (1 == nthSmallest) {
      uint8_t less[this->n], greater[this->n];
      this->calculate_relations(less, greater);

      int num_groups = 0;
      int s = 0;

      for (int i = 0; i < this->n; ++i) {
        if (0 == greater[i]) {
          num_groups += 1;
          s += 1 << less[i];
        }
      }

      // return max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 3;
      return max_comparisons >= num_groups + (std::numeric_limits<uint32_t>::digits - __builtin_clz(s)) - 3;
    } else if (this->nthSmallest == this->n - 2) {
      uint8_t less[this->n], greater[this->n];
      this->calculate_relations(less, greater);

      int num_groups = 0;
      int s = 0;

      for (int i = 0; i < this->n; ++i) {
        if (0 == less[i]) {
          num_groups += 1;
          s += 1 << greater[i];
        }
      }

      // return max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 3
      return max_comparisons >= num_groups + (std::numeric_limits<uint32_t>::digits - __builtin_clz(s)) - 3;
    } else if (this->n - 1 < min_n_comparisons_len) {
      uint8_t less[this->n], greater[this->n];
      this->calculate_relations(less, greater);

      int comps = min_n_comparisons[this->n - 1][std::min((int)this->nthSmallest, this->n - this->nthSmallest - 1)];

      // comps -= less[0..this->n]
      //     .iter()
      //     .filter(|elem| **elem == 1)
      //     .count() as u8;

      comps -= std::count_if(less, less + this->n, [](uint8_t elem) { return elem == 1; });

      if (comps <= max_comparisons) {
        return true;
      }

      for (int i = 0; i < this->n - 1; ++i) {
        if (less[i] < 2) {
          continue;
        }

        // j_loop:
        for (int j = i + 1; j < this->n; ++j) {
          if (!this->is_less(j, i)) {
            continue;
          }

          if (1 == greater[j]) {
            comps -= 1;

            if (comps <= max_comparisons) {
              return true;
            }
          } else {
            bool breaked = false;
            for (int k = 0; k < this->n; ++k) {
              if (i != k && j != k && this->is_less(j, k) && this->is_less(k, i)) {
                // continue j_loop;
                breaked = true;
                break;
              }
            }
            if (breaked) {
              continue;
            }

            comps -= 1;

            if (comps <= max_comparisons) {
              return true;
            }
          }
        }
      }

      return comps <= max_comparisons;
    } else {
      return true;
    }
  }

  /// @brief returns how many elements are less or greater than it
  /// @param less wenn `less[3] = 2`, dann gibt es {a, b} mit a != b, is_less(a, 3) und is_less(b, 3)
  /// @param greater
  void calculate_relations(uint8_t less[], uint8_t greater[]) const {
    std::memset(less, 0, this->n);
    std::memset(greater, 0, this->n);
    for (uint8_t i = 0; i < this->n; ++i) {
      for (uint8_t j = 0; j < this->n; ++j) {
        if (this->is_less(i, j)) {
          ++less[j];
          ++greater[i];
        }
      }
    }
  }

  /// @brief
  /// @param i
  /// @param j
  /// @return true, if comparison (i, j) is redundant
  inline bool is_redundant(const uint16_t i, const uint16_t j) const {
    for (uint16_t k = 0; k < this->n; ++k) {
      if (this->is_less(i, k) && this->is_less(k, j)) {
        return true;
      }
    }
    return false;
  }

  /// @brief *this is a closed Poset
  /// @param normalizer
  /// @param i
  /// @param j
  /// @return Menge an Posets M, wobei für alle m in M gilt: m ist vollständig, NICHT normalisiert, aber dedupliziert,
  //          durch Aufruf von `m.add_less(i, j)` erhält man wieder `*this` und keine unnötigen Vergleiche
  //          gespeichert haben
  inline std::unordered_set<Poset<maxN>> remove_less(Normalizer<maxN> &normalizer, const uint16_t i,
                                                     const uint16_t j) const {
    // assert(this->is_closed());  // check if input closed

    std::unordered_set<Poset<maxN>> result, resultNormalized;
    if (!this->is_less(i, j) || this->is_redundant(i, j)) {
      return result;
    }

    Poset<maxN> poset_initial = *this;
    poset_initial.set_less(i, j, false);

    std::queue<Poset<maxN>> queue{};
    queue.push(poset_initial);

    result.insert(poset_initial);
    normalizer.canonify_nauty(poset_initial);
    resultNormalized.insert(poset_initial);

    while (!queue.empty()) {
      Poset<maxN> poset = queue.front();
      queue.pop();

      for (uint8_t i1 = 0; i1 < this->n; ++i1) {
        for (uint8_t j1 = 0; j1 < this->n; ++j1) {
          // brich ab, wenn eine der folgenden Bedingungen entritt:
          // abs(j-i) >= abs(j1-i1): wenn das gilt, kann es sich um keinen transitiven Vergleich handeln? (Beweis: tbd)
          // !poset.is_less(i1, j1): der zu entfernende Vergleich existiert gar nicht
          // poset.is_redundant(i1, j1): wenn es sich bei (i1, j1) um eine redundante Kante handelt, muss dieser Pfad
          // auch nicht weiterverfolgt werden? (Beweis: tbd)
          if (i1 == j1 || abs(j - i) >= abs(j1 - i1) || !poset.is_less(i1, j1) || poset.is_redundant(i1, j1)) {
            continue;
          }

          Poset<maxN> poset_next = poset;
          poset_next.set_less(i1, j1, false);

          // wenn durch hinzufügen des ursprünglich entfernten Vergleichs, nicht wieder das ursprüngliche Poset
          // entsteht, brich ab
          if (*this != poset_next.with_less(i, j)) {
            continue;
          }

          // wenn das poset normalisert schon in result ist, abbruch
          Poset<maxN> poset_norm = poset_next;
          normalizer.canonify_nauty(poset_norm);
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

    // check if output closed
    // for (auto item : result) {
    //   assert(item.is_closed());  // check if item closed
    // }

    return result;
  }

  // invariant after method: 2 * i < n
  inline Poset<maxN> &dual() {
    this->nthSmallest = this->n - 1 - this->nthSmallest;
    for (uint8_t i = 0; i < this->n; ++i) {
      for (uint8_t j = i + 1; j < this->n; ++j) {
        const bool temp = this->is_less(i, j);
        this->set_less(i, j, this->is_less(j, i));
        this->set_less(j, i, temp);
      }
    }
    return *this;
  }

  /// @brief
  /// @param poset
  /// @return true, if *this is a subset of `poset`
  bool subset_of(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == poset.nthSmallest && (~comparisonTable | poset.comparisonTable).all();
  }

  // TODO: iterator for all "ones"

  /// @brief
  /// @param poset
  /// @return
  bool operator==(const Poset<maxN> &poset) const {
    return n == poset.n && nthSmallest == poset.nthSmallest && comparisonTable == poset.comparisonTable;
  }

  /// @brief
  /// @return hash of poset
  std::size_t hash() const {
    const std::hash<std::bitset<maxN * maxN>> hash1;
    return ((std::size_t)n << (std::size_t)4) ^ nthSmallest ^ hash1(comparisonTable);
  }

  template <std::size_t maxN2>
  friend std::ostream &operator<<(std::ostream &os, const Poset<maxN2> &poset);

  friend class Normalizer<maxN>;

  template <std::size_t maxN2>
  friend std::unordered_set<Poset<maxN2>> enlarge(Normalizer<maxN2> &normalizer,
                                                  const std::unordered_set<Poset<maxN2>> &setOfPosets);

  template <std::size_t maxN2>
  friend std::unordered_set<Poset<maxN2>> enlarge2(Normalizer<maxN2> &normalizer,
                                                   const std::unordered_set<Poset<maxN2>> &setOfPosets);
};

template <std::size_t maxN>
struct std::hash<Poset<maxN>> {
  std::size_t operator()(const Poset<maxN> &poset) const { return poset.hash(); }
};

template <std::size_t maxN>
std::ostream &operator<<(std::ostream &os, const Poset<maxN> &poset) {
  os << "n = " << (uint16_t)poset.n << ", nthSmallest = " << (uint16_t)poset.nthSmallest;
  for (uint16_t i = 0; i < poset.n; ++i) {
    os << '\n';
    for (uint16_t j = 0; j < poset.n; ++j) {
      os << poset.is_less(i, j) << " ";
    }
  }
  return os;
}

template <std::size_t maxN>
bool canTheLastElementBeReduced(const Poset<maxN> &poset) {
  uint8_t greater = 0;
  for (uint8_t k = 0; k < poset.size(); ++k) {
    if (poset.is_less(k, poset.size() - 1)) {
      ++greater;
    }
  }
  return poset.nth() < greater;
}

// gibt ALLE closed, canonfified Posets zurück, die sich durch die Menge bilden lassen und das letzte wegreduziert
// werden kann
template <std::size_t maxN>
std::unordered_set<Poset<maxN>> enlarge(Normalizer<maxN> &normalizer,
                                        const std::unordered_set<Poset<maxN>> &setOfPosets) {
  std::unordered_set<Poset<maxN>> result;
  std::unordered_map<Poset<maxN>, int> swap_init;
  for (const Poset<maxN> &poset : setOfPosets) {
    Poset<maxN> temp{uint8_t(poset.n + uint8_t(1)), poset.nthSmallest};
    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        temp.set_less(i, j, poset.is_less(i, j));
      }
    }
    swap_init[temp] = -1;
  }

  while (!swap_init.empty()) {
    std::unordered_map<Poset<maxN>, int> temp;
    for (const auto &[poset, number] : swap_init) {
      for (int k = number + 1; k < poset.n - 1; ++k) {  // auflösen???
        if (!poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k)) {
          Poset<maxN> a1 = poset.with_less(k, poset.n - 1);
          if (!result.contains(a1) && canTheLastElementBeReduced(a1)) {
            bool is_same = true;  // INEFFIZIENT, canonify???
            for (uint8_t i = 0; i < poset.n - 1; ++i) {
              for (uint8_t j = 0; j < poset.n - 1; ++j) {
                if (a1.is_less(i, j) != poset.is_less(i, j)) {
                  is_same = false;
                }
              }
            }
            if (is_same) {
              result.insert(a1);
              temp[a1] = k;
            }
          }
          Poset<maxN> b1 = poset.with_less(poset.n - 1, k);
          if (!result.contains(b1) && canTheLastElementBeReduced(b1)) {
            bool is_same = true;
            for (uint8_t i = 0; i < poset.n - 1; ++i) {
              for (uint8_t j = 0; j < poset.n - 1; ++j) {
                if (b1.is_less(i, j) != poset.is_less(i, j)) {
                  is_same = false;
                }
              }
            }
            if (is_same) {
              result.insert(b1);
              temp[b1] = k;
            }
          }
        }
      }
    }
    swap_init = temp;
  }

  std::unordered_set<Poset<maxN>> result_canonified;
  for (Poset<maxN> item : result) {
    normalizer.canonify_nauty(item);
    result_canonified.insert(item);
  }
  // prüfe auf shadowing -> weniger pot. Ergebnisse
  return result_canonified;
}

template <std::size_t maxN>
bool canTheLastElementBeReduced_withKInc(const Poset<maxN> &poset) {
  uint8_t less = 0;
  for (uint8_t k = 0; k < poset.size(); ++k) {
    if (poset.is_less(poset.size() - 1, k)) {
      ++less;
    }
  }
  return (poset.size() - 1) - poset.nth() < less;
}

// gibt ALLE closed, canonfified Posets zurück, die sich durch die Menge bilden lassen und das letzte wegreduziert
// werden kann
// wenn poset Größe (n, k) hat, dann kommt (n + 1, k); (n + 1, k + 1) zurück
template <std::size_t maxN>
std::unordered_set<Poset<maxN>> enlarge2(Normalizer<maxN> &normalizer,
                                         const std::unordered_set<Poset<maxN>> &setOfPosets) {
  std::unordered_set<Poset<maxN>> result;
  std::unordered_map<Poset<maxN>, int> swap_init;
  for (const Poset<maxN> &poset : setOfPosets) {
    Poset<maxN> temp{uint8_t(poset.n + uint8_t(1)), uint8_t(poset.nthSmallest + uint8_t(1))};
    for (uint8_t i = 0; i < poset.n; ++i) {
      for (uint8_t j = 0; j < poset.n; ++j) {
        temp.set_less(i, j, poset.is_less(i, j));
      }
    }
    swap_init[temp] = -1;
  }

  while (!swap_init.empty()) {
    std::unordered_map<Poset<maxN>, int> temp;
    for (const auto &[poset, number] : swap_init) {
      for (int k = number + 1; k < poset.n - 1; ++k) {  // auflösen???
        if (!poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k)) {
          Poset<maxN> a1 = poset.with_less(k, poset.n - 1);
          if (!result.contains(a1)) {
            bool is_same = true;  // INEFFIZIENT, canonify???
            for (uint8_t i = 0; i < poset.n - 1; ++i) {
              for (uint8_t j = 0; j < poset.n - 1; ++j) {
                if (a1.is_less(i, j) != poset.is_less(i, j)) {
                  is_same = false;
                }
              }
            }
            if (is_same) {
              result.insert(a1);
              temp[a1] = k;
            }
          }
          Poset<maxN> b1 = poset.with_less(poset.n - 1, k);
          if (!result.contains(b1)) {
            bool is_same = true;
            for (uint8_t i = 0; i < poset.n - 1; ++i) {
              for (uint8_t j = 0; j < poset.n - 1; ++j) {
                if (b1.is_less(i, j) != poset.is_less(i, j)) {
                  is_same = false;
                }
              }
            }
            if (is_same) {
              result.insert(b1);
              temp[b1] = k;
            }
          }
        }
      }
    }
    swap_init = temp;
  }

  std::unordered_set<Poset<maxN>> result_canonified;
  for (Poset<maxN> item : result) {
    if (canTheLastElementBeReduced_withKInc(item)) {
      normalizer.canonify_nauty(item);
      result_canonified.insert(item);
    }
  }
  return result_canonified;
}