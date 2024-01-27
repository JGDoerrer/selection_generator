#include "poset.h"

#include <bits/stdc++.h>

#include "cache_tree.h"
#include "util.h"

#define MAXN WORDSIZE
#include "../nauty2_8_8/nauty.h"

/// @brief constructs an empty Poset
/// @param n
/// @param nthSmallest
Poset::Poset(const uint8_t n, const uint8_t nthSmallest) : n(n), nthSmallest(nthSmallest), comparisonTable() {}

/// @return size of the Poset
uint8_t Poset::size() const { return this->n; }

/// @return the nthSmallest Element
uint8_t Poset::nth() const { return this->nthSmallest; }

bool Poset::get_index(uint8_t pos) const { return comparisonTable[pos]; }

void Poset::set_index(const uint8_t pos, const bool value) { comparisonTable[pos] = value; }

std::size_t Poset::getComparisonTableSize() const { return this->n * this->n; }

std::size_t Poset::toInternalPos(const uint16_t i, const uint16_t j) const { return i * this->n + j; }

/// @brief checks, whether it holds `arr[i] < arr[j]`, e.g. `is_less(i, j) == true` => `arr[i] < arr[j]`
//         Attention: `!is_less(i, j)` IMPLIES NOT `arr[i] > arr[j]`
/// @param i
/// @param j
/// @return
bool Poset::is_less(const uint16_t i, const uint16_t j) const { return this->get_index(this->toInternalPos(i, j)); }

void Poset::set_less(const uint16_t i, const uint16_t j, const bool value) {
  this->set_index(this->toInternalPos(i, j), value);
}

/// @brief
/// @param poset
/// @return true, if *this is a subset of `poset`
bool Poset::subset_of(const Poset &poset) const {
  return n == poset.n && nthSmallest == poset.nthSmallest && (~comparisonTable | poset.comparisonTable).all();
}

Poset &Poset::add_and_close_recursive(const uint16_t i, const uint16_t j) {
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

/// @brief adds i < j to the poset
/// @param i
/// @param j
/// @return
Poset &Poset::add_less(const uint16_t i, const uint16_t j) {
  return this->add_and_close_recursive(i, j);  // faster than iterative
}

/// @brief
/// @param i
/// @param j
/// @return retunrs a clone of the poset, with i < j added
Poset Poset::with_less(const uint16_t i, const uint16_t j) const { return Poset{*this}.add_less(i, j); }

/// @brief returns how many elements are less or greater than it
/// @param less wenn `less[3] = 2`, dann gibt es {a, b} mit a != b, is_less(a, 3) und is_less(b, 3)
/// @param greater
void Poset::calculate_relations(uint8_t less[], uint8_t greater[]) const {
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

void Poset::swap(const uint16_t i1, const uint16_t j1, const uint16_t i2, const uint16_t j2) {
  const bool temp = this->is_less(i1, j1);
  this->set_less(i1, j1, this->is_less(i2, j2));
  this->set_less(i2, j2, temp);
}

void Poset::swap(const uint16_t i, const uint16_t j) {
  for (uint8_t k = 0; k < this->n; ++k) {
    this->swap(i, k, j, k);
  }
  for (uint8_t k = 0; k < this->n; ++k) {
    this->swap(k, i, k, j);
  }
}

// invariant after method: 2 * i < n
Poset &Poset::dual() {
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

Poset &Poset::reduce_n() {
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
    const Poset oldPoset(*this);
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

std::array<int, MAX_N> Poset::canonify_nauty_indicies() const {
  constexpr static int m = SETWORDSNEEDED(MAX_N);
  constexpr static int ms = SETWORDSNEEDED(MAX_N) * MAX_N;

  if constexpr (debug) {
    assert(MAX_N <= WORDSIZE);
    nauty_check(WORDSIZE, m, MAX_N, NAUTYVERSIONID);
  }

  graph g[ms];

  EMPTYGRAPH(g, m, this->n);
  for (uint16_t i = 0; i < this->n; ++i) {
    for (uint16_t j = 0; j < this->n; ++j) {
      if (this->is_less(i, j)) {
        ADDONEARC(g, i, j, m);
      }
    }
  }

  std::array<int, MAX_N> lab, ptn, orbits;
  for (uint8_t i = 0; i < this->n; ++i) {
    lab[i] = i;
    ptn[i] = 0;  // hier 0 oder 1?
  }
  ptn[this->n - 1] = 0;

  graph result[ms];
  EMPTYGRAPH(result, m, this->n);

  DEFAULTOPTIONS_GRAPH(options);
  options.getcanon = TRUE;
  options.digraph = TRUE;

  statsblk stats;
  densenauty(g, lab.data(), ptn.data(), orbits.data(), &options, &stats, m, this->n, result);
  assert(stats.errstatus == 0);

  return lab;
}

Poset &Poset::canonify() {
  constexpr bool ONLY_NAUTY = false;

  const Poset oldPoset(*this);
  std::array<int, MAX_N> new_indices;
  if constexpr (ONLY_NAUTY) {
    new_indices = this->canonify_nauty_indicies();
  } else {
    uint8_t less[this->n];
    uint8_t greater[this->n];
    this->calculate_relations(less, greater);

    std::vector<uint64_t> in_out_degree(this->n);
    for (uint8_t i = 0; i < this->n; ++i) {
      in_out_degree[i] = uint64_t(MAX_N) * uint64_t(greater[i]) + uint64_t(less[i]);
    }

    std::vector<uint64_t> hash = in_out_degree;
    for (int q = 0; q < 2; ++q) {
      uint64_t sum_hash[MAX_N];
      for (int k = 0; k < MAX_N; ++k) {
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
        hash[i] = sum_hash[i] * (MAX_N * MAX_N) + in_out_degree[i];
      }
    }

    std::iota(new_indices.begin(), new_indices.begin() + this->n, 0);

    std::stable_sort(new_indices.begin(), new_indices.begin() + this->n, [&](const int a, const int b) {
      return in_out_degree[a] < in_out_degree[b] || (in_out_degree[a] == in_out_degree[b] && hash[a] < hash[b]);
    });

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
      new_indices = this->canonify_nauty_indicies();

      std::stable_sort(new_indices.begin(), new_indices.begin() + this->n, [&](const int a, const int b) {
        return in_out_degree[a] < in_out_degree[b] || (in_out_degree[a] == in_out_degree[b] && hash[a] < hash[b]);
      });
    }
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

Poset &Poset::normalize() { return this->reduce_n().canonify(); }

/// @brief
/// @param i
/// @param j
/// @return true, if comparison (i, j) is redundant
bool Poset::is_redundant(const uint16_t i, const uint16_t j) const {
  for (uint16_t k = 0; k < this->n; ++k) {
    if (this->is_less(i, k) && this->is_less(k, j)) {
      return true;
    }
  }
  return false;
}

/// @brief *this is a closed Poset
/// @param i
/// @param j
/// @return Menge an Posets M, wobei für alle m in M gilt: m ist vollständig, normalisiert, dedupliziert,
//          durch Aufruf von `m.add_less(i, j)` erhält man wieder `*this`, durch Aufruf von `m.add_less(j, i)` erfüllt
//          Kriterium `test` und keine unnötigen Vergleiche gespeichert haben
std::unordered_set<Poset> Poset::remove_less(const uint16_t i, const uint16_t j,
                                             const std::function<bool(const Poset &)> &test) const {
  // assert(this->is_closed());  // check if input closed

  std::unordered_set<Poset> result;
  if (!this->is_less(i, j) || this->is_redundant(i, j)) {
    return result;
  }

  Poset poset_initial = *this;
  poset_initial.set_less(i, j, false);

  Poset poset_check = poset_initial.with_less(j, i);
  poset_check.normalize();
  if (!test(poset_check)) {
    return result;
  }

  std::queue<Poset> queue{};
  queue.push(poset_initial);

  poset_initial.canonify();
  result.insert(poset_initial);

  while (!queue.empty()) {
    Poset poset = queue.front();
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

        Poset poset_next = poset;
        poset_next.set_less(i1, j1, false);

        // wenn durch hinzufügen des ursprünglich entfernten Vergleichs, nicht wieder das ursprüngliche Poset
        // entsteht, brich ab
        if (*this != poset_next.with_less(i, j)) {
          continue;
        }

        Poset poset_check = poset_next.with_less(j, i);
        poset_check.normalize();
        if (!test(poset_check)) {
          continue;
        }

        // wenn das poset normalisert schon in result ist, abbruch
        Poset poset_norm = poset_next;
        poset_norm.canonify();
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

std::unordered_set<Poset> Poset::filter(const std::unordered_set<Poset> &unfiltered) {
  // TODO: OPTIMIERUNG BUCKETS NACH ANZAHL EINSEN IN POSET
  // TODO: OPtimierung: betrachte element nicht mehr, wenn schon in filtered -> n^2 Schritte zu n^2/2 Schritte
  std::unordered_set<Poset> filtered;
  for (const Poset &item : unfiltered) {
    bool found = false;
    for (const Poset &temp : unfiltered) {
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

bool Poset::can_reduce_element_greater(const uint8_t element) const {
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
void Poset::enlarge_n(std::unordered_set<Poset> &result) const {
  Poset temp{uint8_t(this->n + uint8_t(1)), this->nthSmallest};
  for (uint8_t i = 0; i < this->n; ++i) {
    for (uint8_t j = 0; j < this->n; ++j) {
      temp.set_less(i, j, this->is_less(i, j));
    }
  }

  std::unordered_set<Poset> unfiltered;
  std::stack<std::pair<Poset, int>> swap_init;
  swap_init.push({temp, -1});
  while (!swap_init.empty()) {
    const auto [poset, number] = swap_init.top();
    swap_init.pop();

    for (int k = number + 1; k < poset.n - 1; ++k) {
      if (!poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k)) {
        Poset new_poset = poset.with_less(k, poset.n - 1);
        swap_init.push({new_poset, k});
        if (new_poset.can_reduce_element_greater(poset.n - 1)) {
          unfiltered.insert(new_poset);
        }
      }
    }
  }

  for (Poset item : Poset::filter(unfiltered)) {
    item.canonify();
    result.insert(item);
  }
}

bool Poset::can_reduce_element_less(const uint8_t element) const {
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
void Poset::enlarge_nk(std::unordered_set<Poset> &result) const {
  Poset temp{uint8_t(this->n + uint8_t(1)), uint8_t(this->nthSmallest + uint8_t(1))};
  for (uint8_t i = 0; i < this->n; ++i) {
    for (uint8_t j = 0; j < this->n; ++j) {
      temp.set_less(i, j, this->is_less(i, j));
    }
  }

  std::unordered_set<Poset> unfiltered;
  std::stack<std::pair<Poset, int>> swap_init;
  swap_init.push({temp, -1});
  while (!swap_init.empty()) {
    const auto [poset, number] = swap_init.top();
    swap_init.pop();

    for (int k = number + 1; k < poset.n - 1; ++k) {
      if (!poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k)) {
        Poset new_poset = poset.with_less(poset.n - 1, k);
        swap_init.push({new_poset, k});
        if (new_poset.can_reduce_element_less(poset.n - 1)) {
          unfiltered.insert(new_poset);
        }
      }
    }
  }

  for (Poset item : Poset::filter(unfiltered)) {
    item.canonify();
    result.insert(item);
  }
}

std::unordered_set<Poset> Poset::enlarge(const std::unordered_set<Poset> &setOfPosets, const int n, const int k) {
  assert(2 * k < n);

  CacheTreeFixed<true> tempSet[n + 1][k + 1];
  for (const Poset &item : setOfPosets) {
    if (item.n <= n && item.nthSmallest <= k) {
      tempSet[item.n][item.nthSmallest].insert(item);
    }
  }
  for (int n0 = 0; n0 < n; ++n0) {
    std::unordered_set<Poset> result;

    for (int k0 = 0; k0 < k; ++k0) {
      for (auto item : tempSet[n0][k0].entries(n0, k0)) {
        item.enlarge_nk(result);
      }
      // tempSet[n0][k0].reset();
    }

    for (auto item : tempSet[n0][k].entries(n0, k)) {
      item.enlarge_n(result);
    }

    for (const Poset &item : result) {
      tempSet[item.n][item.nthSmallest].insert(item);
    }
    // tempSet[n0][k].reset();
  }

  return tempSet[n][k].entries(n, k);
}

std::ostream &operator<<(std::ostream &os, const Poset &poset) {
  os << "n = " << (uint16_t)poset.n << ", nthSmallest = " << (uint16_t)poset.nthSmallest;
  for (uint16_t i = 0; i < poset.n; ++i) {
    os << '\n';
    for (uint16_t j = 0; j < poset.n; ++j) {
      os << poset.is_less(i, j) << " ";
    }
  }
  return os;
}

/// @brief
/// @param poset
/// @return
bool Poset::operator==(const Poset &poset) const {
  return n == poset.n && nthSmallest == poset.nthSmallest && comparisonTable == poset.comparisonTable;
}

/// @brief
/// @return hash of poset
std::size_t Poset::hash() const {
  const std::hash<std::bitset<MAX_N * MAX_N>> hash1;
  return ((std::size_t)n << (std::size_t)4) ^ nthSmallest ^ hash1(comparisonTable);
}
