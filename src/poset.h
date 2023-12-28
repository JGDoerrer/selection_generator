#pragma once
#include <bits/stdc++.h>

#include "normalizer.h"
// =============
#include "cache.h"

template <std::size_t maxN>
class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;

 public:
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

  inline Poset<maxN> &reduce_n() {
    uint8_t less[this->n];
    uint8_t greater[this->n];
    this->calculate_relations(greater, less);  // TODO: MACHT DAS SINN???

    // can the element be ignored, because it is too large/small
    uint8_t new_indices[this->n];
    uint8_t n_less_dropped = 0;

    // maps the old indices to the new ones
    uint8_t new_n = 0;
    uint8_t b = this->n - 1;

    for (int i = 0; i < this->n; ++i) {
      if (this->nthSmallest < greater[i]) {
        new_indices[b--] = i;
      } else if ((this->n - 1) - this->nthSmallest < less[i]) {
        ++n_less_dropped;
        new_indices[b--] = i;
      } else {
        new_indices[new_n++] = i;
      }
    }

    if (new_n != this->n) {
      const Poset<maxN> oldPoset(*this);
      this->n = new_n;
      this->nthSmallest -= n_less_dropped;
      this->comparisonTable.reset();
      for (uint8_t i = 0; i < new_n; ++i) {
        for (uint8_t j = 0; j < new_n; ++j) {
          this->set_less(i, j, oldPoset.is_less(new_indices[i], new_indices[j]));
        }
      }

      if (this->n <= 2 * this->nthSmallest) {
        this->dual();
      }
    }
    return *this;
  }

 public:
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
    poset_initial.canonify(normalizer);
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
          poset_norm.canonify(normalizer);
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

  inline Poset<maxN> &canonify(Normalizer<maxN> &normalizer) {
    if constexpr (false) {
      uint8_t less[this->n];
      uint8_t greater[this->n];
      this->calculate_relations(less, greater);

      std::vector<std::pair<uint64_t, uint8_t>> in_out_degree(this->n);
      for (uint8_t i = 0; i < this->n; ++i) {
        in_out_degree[i] = {uint64_t(maxN) * uint64_t(greater[i]) + uint64_t(less[i]), i};
      }

      std::sort(in_out_degree.begin(), in_out_degree.end());

      uint8_t duplicats = 0;
      for (uint8_t i = 1; i < this->n; ++i) {
        if (in_out_degree[i - 1].first == in_out_degree[i].first) {
          ++duplicats;
        }
      }

      if (0 == duplicats) {
        const Poset<maxN> oldPoset(*this);
        for (uint8_t i = 0; i < this->n; ++i) {
          for (uint8_t j = 0; j < this->n; ++j) {
            this->set_less(i, j, oldPoset.is_less(in_out_degree[i].second, in_out_degree[j].second));
          }
        }
        return *this;
      }
    }

    return normalizer.canonify_nauty(*this);
  }

  inline Poset<maxN> &normalize(Normalizer<maxN> &normalizer) { return this->reduce_n().canonify(normalizer); }

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
  friend std::unordered_set<Poset<maxN2>> enlarge_n(Normalizer<maxN2> &normalizer,
                                                    const std::unordered_set<Poset<maxN2>> &setOfPosets);

  template <std::size_t maxN2>
  friend std::unordered_set<Poset<maxN2>> enlarge_nk(Normalizer<maxN2> &normalizer,
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
inline bool can_reduce_element_greater(const Poset<maxN> &poset, const uint8_t element) {
  uint8_t greater = 0;
  for (uint8_t k = 0; k < poset.size(); ++k) {
    if (poset.is_less(k, element)) {
      ++greater;
    }
  }
  return poset.nth() < greater;
}

template <std::size_t maxN>
inline bool can_reduce_element_less(const Poset<maxN> &poset, const uint8_t element) {
  uint8_t less = 0;
  for (uint8_t k = 0; k < poset.size(); ++k) {
    if (poset.is_less(element, k)) {
      ++less;
    }
  }
  return (poset.size() - 1) - poset.nth() < less;
}

template <std::size_t maxN>
std::unordered_set<Poset<maxN>> filter(const std::unordered_set<Poset<maxN>> &unfiltered) {
  std::unordered_set<Poset<maxN>> filtered;
  for (const Poset<maxN> &item : unfiltered) {
    bool found = false;
    for (const Poset<maxN> &temp : unfiltered) {
      if (temp != item && temp.subset_of(item)) {
        found = false;
        break;
      }
    }
    if (!found) {
      filtered.insert(item);
    }
  }
  return filtered;
}

// gibt ALLE closed, canonfified Posets zurück, die sich durch die Menge bilden lassen und das letzte wegreduziert
// werden kann
template <std::size_t maxN>
std::unordered_set<Poset<maxN>> enlarge_n(Normalizer<maxN> &normalizer,
                                          const std::unordered_set<Poset<maxN>> &setOfPosets) {
  std::unordered_map<Poset<maxN>, int> swap_init;
  for (const Poset<maxN> &poset : setOfPosets) {
    Poset<maxN> temp{uint8_t(poset.size() + uint8_t(1)), poset.nth()};
    for (uint8_t i = 0; i < poset.size(); ++i) {
      for (uint8_t j = 0; j < poset.size(); ++j) {
        temp.set_less(i, j, poset.is_less(i, j));
      }
    }
    swap_init[temp] = -1;
  }

  std::unordered_set<Poset<maxN>> result;
  while (!swap_init.empty()) {
    std::unordered_map<Poset<maxN>, int> temp;
    for (const auto &[poset, number] : swap_init) {
      for (int k = number + 1; k < poset.size() - 1; ++k) {
        if (!poset.is_less(k, poset.size() - 1) && !poset.is_less(poset.size() - 1, k)) {
          const Poset<maxN> new_poset = poset.with_less(k, poset.size() - 1);
          // TODO: ist der can_reduce_... Test hier korrekt?
          if (!result.contains(new_poset) && can_reduce_element_greater(new_poset, new_poset.size() - 1)) {
            result.insert(new_poset);
            temp[new_poset] = k;
          }
        }
      }
    }
    swap_init = temp;
  }

  std::unordered_set<Poset<maxN>> result_canonified;
  for (Poset<maxN> item : result) {
    // oder eher hier?
    item.canonify(normalizer);
    result_canonified.insert(item);
  }
  // TODO: prüfe auf shadowing -> potentiell weniger Ergebnisse
  return result_canonified;
}

// gibt ALLE closed, canonfified Posets zurück, die sich durch die Menge bilden lassen und das letzte wegreduziert
// werden kann
// wenn poset Größe (n, k) hat, dann return (n + 1, k); (n + 1, k + 1)
template <std::size_t maxN>
std::unordered_set<Poset<maxN>> enlarge_nk(Normalizer<maxN> &normalizer,
                                           const std::unordered_set<Poset<maxN>> &setOfPosets) {
  std::unordered_map<Poset<maxN>, int> swap_init;
  for (const Poset<maxN> &poset : setOfPosets) {
    Poset<maxN> temp{uint8_t(poset.size() + uint8_t(1)), uint8_t(poset.nth() + uint8_t(1))};
    for (uint8_t i = 0; i < poset.size(); ++i) {
      for (uint8_t j = 0; j < poset.size(); ++j) {
        temp.set_less(i, j, poset.is_less(i, j));
      }
    }
    swap_init[temp] = -1;
  }

  std::unordered_set<Poset<maxN>> result;
  while (!swap_init.empty()) {
    std::unordered_map<Poset<maxN>, int> temp;
    for (const auto &[poset, number] : swap_init) {
      for (int k = number + 1; k < poset.size() - 1; ++k) {
        if (!poset.is_less(k, poset.size() - 1) && !poset.is_less(poset.size() - 1, k)) {
          Poset<maxN> new_poset = poset.with_less(poset.size() - 1, k);
          if (!result.contains(new_poset)) {
            result.insert(new_poset);
            temp[new_poset] = k;
          }
        }
      }
    }
    swap_init = temp;
  }

  std::unordered_set<Poset<maxN>> result_canonified;
  for (Poset<maxN> item : result) {
    if (can_reduce_element_less(item, item.size() - 1)) {
      item.canonify(normalizer);
      result_canonified.insert(item);
    }
  }
  return result_canonified;
}

template <std::size_t maxN>
std::unordered_set<Poset<maxN>> enlarge(Normalizer<maxN> &normalizer,
                                        const std::unordered_set<Poset<maxN>> &setOfPosets, const int n, const int k) {
  assert(2 * k < n);

  std::unordered_set<Poset<maxN>> tempSet = setOfPosets;
  PosetSet<maxN> cache;
  while (0 != tempSet.size()) {
    std::unordered_set<Poset<maxN>> remainder1, remainder2;
    // tempSet = filter(tempSet);
    for (auto item : tempSet) {
      assert(item.size() <= n);
      assert(item.nth() < item.size());
      if (item.size() < n) {
        if (item.nth() == k) {
          remainder1.insert(item);
        } else if (item.nth() < k) {
          remainder2.insert(item);
        }
      } else if (item.size() == n && item.nth() == k) {
        cache.insert(item);
      }
    }

    tempSet = enlarge_n(normalizer, remainder1);
    tempSet.merge(enlarge_nk(normalizer, remainder2));
  }

  return cache.entries(n, k, false);
}