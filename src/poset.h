#pragma once
#include <bits/stdc++.h>

#include "normalizer.h"
// =============
#include "cache_Tree.h"

template <std::size_t maxN>
class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;

 public:
  bool rec(const Poset<maxN> &poset, std::vector<int> &new_indices, std::vector<bool> &visited, const int n, int k,
           int a, int b) const {
    if (a > b) {
      return false;
    }
    if (-1 != new_indices[k]) {
      int na = a, nb = b;
      bool maybe = true;
      for (uint8_t q = 0; q < k; ++q) {
        if (this->is_less(k, q)) {
          --na;
        }
        if (new_indices[k] != new_indices[q]) {
          if (poset.is_less(new_indices[k], new_indices[q])) {
            --nb;
          }
          if ((this->is_less(k, q) && !poset.is_less(new_indices[k], new_indices[q])) ||
              (this->is_less(q, k) && !poset.is_less(new_indices[q], new_indices[k]))) {
            maybe = false;
            break;
          }
        }
      }

      if (maybe && (k + 1 == n || rec(poset, new_indices, visited, n, k + 1, na, nb))) {
        return true;
      }
    } else {
      for (int i = 0; i < n; ++i) {
        if (!visited[i]) {
          visited[i] = true;
          new_indices[k] = i;

          int na = a, nb = b;
          bool maybe = true;
          for (uint8_t q = 0; q < k; ++q) {
            if (this->is_less(k, q)) {
              --na;
            }
            if (new_indices[k] != new_indices[q]) {
              if (poset.is_less(new_indices[k], new_indices[q])) {
                --nb;
              }
              if ((this->is_less(k, q) && !poset.is_less(new_indices[k], new_indices[q])) ||
                  (this->is_less(q, k) && !poset.is_less(new_indices[q], new_indices[k]))) {
                maybe = false;
                break;
              }
            }
          }

          if (maybe && (k + 1 == n || rec(poset, new_indices, visited, n, k + 1, na, nb))) {
            return true;
          }
          new_indices[k] = -1;
          visited[i] = false;
        }
      }
    }
    return false;
  }

  // is *this subset of poset?
  // Frage: exisitert eine Permutation new_indicies, sodass Teilmenge?
  // new: 96.148s': n = 8, i = 3, (cache_l: 5535, cache_u: 2387, noSol: 0, bruteForce: 416), cache = 683
  // old: 0.089s': n = 8, i = 3, (cache_l: 11590, cache_u: 3508, noSol: 0, bruteForce: 778), cache = 1081
  bool subsetBruteForce(const Poset &poset) const {
    // eigentlich nicht nötig
    if (n != poset.n || nthSmallest != poset.nthSmallest) {
      return false;
    }

    // Graph graph_this(n), graph_poset(n + 1); // TODO: why is this only working with n + 1???
    // for (uint8_t i = 0; i < n; ++i) {
    //   for (uint8_t j = 0; j < n; ++j) {
    //     if (this->is_less(i, j)) {
    //       add_edge(i, j, graph_this);
    //     }
    //     if (poset.is_less(i, j)) {
    //       add_edge(i, j, graph_poset);
    //     }
    //   }
    // }

    // bool is_iso = false;
    // auto callback = [&](auto f, auto) {
    //   is_iso = true;
    //   return false;
    // };

    // vf2_subgraph_iso(graph_this, graph_poset, callback);

    std::vector<int> new_indices(poset.n, -1);
    std::vector<bool> visited(poset.n, false);
    // for (int i = 0; i < poset.n; ++i) {
    //   for (int j = 0; j < poset.n; ++j) {
    //     bool is_same = true;
    //     for (int k = 0; k < poset.n; ++k) {
    //       if (this->is_less(i, k) != poset.is_less(j, k)) {
    //         is_same = false;
    //         break;
    //       }
    //     }
    //     if (is_same && -1 == new_indices[i] && !visited[j]) {
    //       new_indices[i] = j;
    //       visited[j] = true;
    //     }
    //   }
    // }

    bool sol = rec(poset, new_indices, visited, poset.n, 0, this->count(), poset.count());
    // assert(sol == is_iso);
    return sol;
  }

  std::bitset<maxN * maxN> comparisonTable;

  inline std::size_t getComparisonTableSize() const { return this->n * this->n; }

  inline std::size_t toInternalPos(const uint16_t i, const uint16_t j) const { return i * this->n + j; }

 private:
  inline void set_less(const uint16_t i, const uint16_t j, const bool value) {
    this->comparisonTable[toInternalPos(i, j)] = value;
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
  inline bool is_less(const uint16_t i, const uint16_t j) const {
    // 0 != n && i != j &&
    return this->comparisonTable[toInternalPos(i, j)];
  }

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
    return false;
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
  /// @return Menge an Posets M, wobei für alle m in M gilt: m ist vollständig, normalisiert, dedupliziert,
  //          durch Aufruf von `m.add_less(i, j)` erhält man wieder `*this`, durch Aufruf von `m.add_less(j, i)` erfüllt
  //          Kriterium `test` und keine unnötigen Vergleiche gespeichert haben
  inline std::unordered_set<Poset<maxN>> remove_less(Normalizer<maxN> &normalizer, const uint16_t i, const uint16_t j,
                                                     const std::function<bool(const Poset<maxN> &)> &test) const {
    std::unordered_set<Poset<maxN>> result;
    if (!this->is_less(i, j) || this->is_redundant(i, j)) {
      return result;
    }

    Poset<maxN> poset_initial = *this;
    poset_initial.set_less(i, j, false);

    Poset<maxN> poset_check = poset_initial.with_less(j, i);
    poset_check.normalize(normalizer);
    if (!test(poset_check)) {
      return result;
    }

    std::queue<Poset<maxN>> queue{};
    queue.push(poset_initial);

    poset_initial.canonify(normalizer);
    result.insert(poset_initial);

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
          if (i1 == j1 || abs((int)j - (int)i) >= abs((int)j1 - (int)i1) || !poset.is_less(i1, j1) ||
              poset.is_redundant(i1, j1)) {
            continue;
          }

          Poset<maxN> poset_next = poset;
          poset_next.set_less(i1, j1, false);

          // wenn durch hinzufügen des ursprünglich entfernten Vergleichs, nicht wieder das ursprüngliche Poset
          // entsteht, brich ab
          if (*this != poset_next.with_less(i, j)) {
            continue;
          }

          Poset<maxN> poset_check = poset_next.with_less(j, i);
          poset_check.normalize(normalizer);
          if (!test(poset_check)) {
            continue;
          }

          // wenn das poset normalisert schon in result ist, abbruch
          Poset<maxN> poset_norm = poset_next;
          poset_norm.canonify(normalizer);
          if (result.contains(poset_norm)) {
            continue;
          }
          result.insert(poset_norm);

          // mache sonst in den nächsten Schritten mit dem Poset weiter und versuche noch mehr Vergleiche zu entfernen
          queue.push(poset_next);
        }
      }
    }

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

  inline void swap(const uint16_t i1, const uint16_t j1, const uint16_t i2, const uint16_t j2) {
    const bool temp = this->is_less(i1, j1);
    this->set_less(i1, j1, this->is_less(i2, j2));
    this->set_less(i2, j2, temp);
  }

  inline void swap(const uint16_t i, const uint16_t j) {
    for (uint8_t k = 0; k < this->n; ++k) {
      this->swap(i, k, j, k);
    }
    for (uint8_t k = 0; k < this->n; ++k) {
      this->swap(k, i, k, j);
    }
  }

  inline Poset<maxN> &canonify(Normalizer<maxN> &normalizer) {
    // static std::array<std::array<int, maxN>, maxN> table;
    // static int debug = 0;

    // for (uint8_t i = 0; i < this->n; ++i) {
    //   for (uint8_t j = 0; j < this->n; ++j) {
    //     if (this->is_less(i, j)) {
    //       ++table[i][j];
    //     }
    //   }
    // }
    // ++debug;
    // if (debug % 1000000 == 0) {
    //   int max = 0;
    //   for (uint8_t i = 0; i < this->n; ++i) {
    //     for (uint8_t j = 0; j < this->n; ++j) {
    //       max = std::max(max, table[i][j]);
    //     }
    //   }
    //   for (uint8_t i = 0; i < this->n; ++i) {
    //     for (uint8_t j = 0; j < this->n; ++j) {
    //       std::cout << (int)std::ceil(1000 * table[i][j] / float(max)) << "\t ";
    //     }
    //     std::cout << std::endl;
    //   }
    // }

    constexpr bool ONLY_NAUTY = false;
    if constexpr (ONLY_NAUTY) {
      return normalizer.canonify_nauty(*this);
    } else {
      uint8_t less[this->n];
      uint8_t greater[this->n];
      this->calculate_relations(less, greater);

      std::vector<uint64_t> in_out_degree(this->n);
      for (uint8_t i = 0; i < this->n; ++i) {
        in_out_degree[i] = uint64_t(maxN) * uint64_t(greater[i]) + uint64_t(less[i]);
      }

      std::vector<uint64_t> hash = in_out_degree;
      for (int q = 0; q < 2; ++q) {
        uint64_t sum_hash[maxN];
        for (int k = 0; k < maxN; ++k) {
          sum_hash[k] = 0;
        }

        for (size_t i = 0; i < this->n; ++i) {
          uint64_t sum = hash[i];

          for (size_t j = 0; j < this->n; ++j) {
            if (i != j && (this->is_less(i, j) || this->is_less(j, i))) {
              sum ^= hash[j];
            }
          }

          sum_hash[i] = sum;
        }

        for (size_t i = 0; i < this->n; ++i) {
          hash[i] = sum_hash[i] * (maxN * maxN) + in_out_degree[i];
        }
      }

      std::vector<int> new_indices(this->n);
      std::iota(new_indices.begin(), new_indices.end(), 0);

      std::stable_sort(new_indices.begin(), new_indices.end(), [&](const int a, const int b) {
        return in_out_degree[a] < in_out_degree[b] || (in_out_degree[a] == in_out_degree[b] && hash[a] < hash[b]);
      });

      const Poset<maxN> oldPoset(*this);
      bool isUnique = true;
      for (uint8_t i = 1; i < this->n; ++i) {
        if (in_out_degree[new_indices[i - 1]] == in_out_degree[new_indices[i]] &&
            hash[new_indices[i - 1]] == hash[new_indices[i]]) {
          this->swap(new_indices[i - 1], new_indices[i]);
          if (*this != oldPoset) {
            isUnique = false;
            break;
          }
        }
      }

      if (!isUnique) {
        new_indices = normalizer.canonify_nauty_indicies(*this);

        std::stable_sort(new_indices.begin(), new_indices.end(), [&](const int a, const int b) {
          return in_out_degree[a] < in_out_degree[b] || (in_out_degree[a] == in_out_degree[b] && hash[a] < hash[b]);
        });
      }

      for (uint8_t i = 0; i < this->n; ++i) {
        for (uint8_t j = 0; j < this->n; ++j) {
          this->set_less(i, j, oldPoset.is_less(new_indices[i], new_indices[j]));
        }
      }

      for (uint8_t i = 0; i < this->n; ++i) {
        for (uint8_t j = i + 1; j < this->n; ++j) {
          assert(!this->is_less(i, j));
        }
      }

      return *this;
    }
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

  inline bool can_reduce_element_greater(const uint8_t element) const {
    uint8_t greater = 0;
    for (uint8_t k = 0; k < this->n; ++k) {
      if (this->is_less(k, element)) {
        ++greater;
      }
    }
    return this->nthSmallest < greater;
  }

  // gibt ALLE closed, canonfified Posets zurück, die sich durch die Menge bilden lassen und das letzte wegreduziert
  // werden kann
  void enlarge_n(Normalizer<maxN> &normalizer, std::unordered_set<Poset<maxN>> &result) const {
    Poset<maxN> temp{uint8_t(this->n + uint8_t(1)), this->nthSmallest};
    for (uint8_t i = 0; i < this->n; ++i) {
      for (uint8_t j = 0; j < this->n; ++j) {
        temp.set_less(i, j, this->is_less(i, j));
      }
    }

    std::unordered_set<Poset<maxN>> unfiltered;
    std::stack<std::pair<Poset<maxN>, int>> swap_init;
    swap_init.push({temp, -1});
    while (!swap_init.empty()) {
      const auto [poset, number] = swap_init.top();
      swap_init.pop();

      for (int k = number + 1; k < poset.n - 1; ++k) {
        if (!poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k)) {
          Poset<maxN> new_poset = poset.with_less(k, poset.n - 1);
          swap_init.push({new_poset, k});
          if (new_poset.can_reduce_element_greater(poset.n - 1)) {
            unfiltered.insert(new_poset);
          }
        }
      }
    }

    for (Poset<maxN> item : Poset::filter(unfiltered)) {
      item.canonify(normalizer);
      result.insert(item);
    }
  }

  inline bool can_reduce_element_less(const uint8_t element) const {
    uint8_t less = 0;
    for (uint8_t k = 0; k < this->n; ++k) {
      if (this->is_less(element, k)) {
        ++less;
      }
    }
    return (this->n - 1) - this->nthSmallest < less;
  }

  // gibt ALLE closed, canonfified Posets zurück, die sich durch die Menge bilden lassen und das letzte wegreduziert
  // werden kann
  // wenn poset Größe (n, k) hat, dann return (n + 1, k); (n + 1, k + 1)
  void enlarge_nk(Normalizer<maxN> &normalizer, std::unordered_set<Poset<maxN>> &result) const {
    Poset<maxN> temp{uint8_t(this->n + uint8_t(1)), uint8_t(this->nthSmallest + uint8_t(1))};
    for (uint8_t i = 0; i < this->n; ++i) {
      for (uint8_t j = 0; j < this->n; ++j) {
        temp.set_less(i, j, this->is_less(i, j));
      }
    }

    std::unordered_set<Poset<maxN>> unfiltered;
    std::stack<std::pair<Poset<maxN>, int>> swap_init;
    swap_init.push({temp, -1});
    while (!swap_init.empty()) {
      const auto [poset, number] = swap_init.top();
      swap_init.pop();

      for (int k = number + 1; k < poset.n - 1; ++k) {
        if (!poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k)) {
          Poset<maxN> new_poset = poset.with_less(poset.n - 1, k);
          swap_init.push({new_poset, k});
          if (new_poset.can_reduce_element_less(poset.n - 1)) {
            unfiltered.insert(new_poset);
          }
        }
      }
    }

    for (Poset<maxN> item : Poset::filter(unfiltered)) {
      item.canonify(normalizer);
      result.insert(item);
    }
  }

  static std::unordered_set<Poset<maxN>> enlarge(Normalizer<maxN> &normalizer,
                                                 const std::unordered_set<Poset<maxN>> &setOfPosets, const int n,
                                                 const int k) {
    assert(2 * k < n);

    PosetSet<maxN> tempSet[n + 1][k + 1];
    for (const Poset<maxN> &item : setOfPosets) {
      if (item.n <= n && item.nthSmallest <= k) {
        tempSet[item.n][item.nthSmallest].insert(item, false);
      }
    }
    for (int n0 = 0; n0 < n; ++n0) {
      std::unordered_set<Poset<maxN>> result;

      for (int k0 = 0; k0 < k; ++k0) {
        for (auto item : tempSet[n0][k0].entries(n0, k0, false)) {
          item.enlarge_nk(normalizer, result);
        }
        tempSet[n0][k0].reset();
      }

      for (auto item : tempSet[n0][k].entries(n0, k, false)) {
        item.enlarge_n(normalizer, result);
      }

      for (const Poset<maxN> &item : result) {
        tempSet[item.n][item.nthSmallest].insert(item, false);
      }
      tempSet[n0][k].reset();
    }

    return filter2(tempSet[n][k].entries(n, k, false));
  }

  static std::unordered_set<Poset<maxN>> filter(const std::unordered_set<Poset<maxN>> &unfiltered) {
    // TODO: OPTIMIERUNG BUCKETS NACH ANZAHL EINSEN IN POSET
    // TODO: OPtimierung: betrachte element nicht mehr, wenn schon in filtered -> n^2 Schritte zu n^2/2 Schritte
    std::unordered_set<Poset<maxN>> filtered;
    for (const Poset<maxN> &item : unfiltered) {
      bool found = false;
      for (const Poset<maxN> &temp : unfiltered) {
        if (temp != item && temp.subset_of(item)) {
          found = true;
          break;
        }
      }
      if (!found) {
        filtered.insert(item);
      }
    }
    return filtered;
  }

  static std::unordered_set<Poset<maxN>> filter2(const std::unordered_set<Poset<maxN>> &unfiltered) {
    // TODO: OPTIMIERUNG BUCKETS NACH ANZAHL EINSEN IN POSET
    // TODO: OPtimierung: betrachte element nicht mehr, wenn schon in filtered -> n^2 Schritte zu n^2/2 Schritte
    std::unordered_set<Poset<maxN>> filtered;
    for (const Poset<maxN> &item : unfiltered) {
      bool found = false;
      for (const Poset<maxN> &temp : unfiltered) {
        if (temp != item && temp.subsetBruteForce(item)) {
          found = true;
          break;
        }
      }
      if (!found) {
        filtered.insert(item);
      }
    }
    return filtered;
  }
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
