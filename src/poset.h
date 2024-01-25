#pragma once
#include <bits/stdc++.h>

#include "util.h"

class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;

 public:
  std::bitset<MAX_N * MAX_N> comparisonTable;

  // constructor
  Poset(const uint8_t n, const uint8_t nthSmallest);

  // getter
  uint8_t size() const;

  uint8_t nth() const;

  bool get_index(uint8_t pos) const;

  void set_index(const uint8_t pos, const bool value);

  std::size_t getComparisonTableSize() const;

  std::size_t toInternalPos(const uint16_t i, const uint16_t j) const;

  bool is_less(const uint16_t i, const uint16_t j) const;

  void set_less(const uint16_t i, const uint16_t j, const bool value);

  bool subset_of(const Poset &poset) const;

  // add
  Poset &add_and_close_recursive(const uint16_t i, const uint16_t j);

  Poset &add_less(const uint16_t i, const uint16_t j);

  Poset with_less(const uint16_t i, const uint16_t j) const;

  // reduce
  void calculate_relations(uint8_t less[], uint8_t greater[]) const;

  void swap(const uint16_t i1, const uint16_t j1, const uint16_t i2, const uint16_t j2);

  void swap(const uint16_t i, const uint16_t j);

  Poset &dual();

  Poset &reduce_n();

  // canonify
  std::array<int, MAX_N> canonify_nauty_indicies() const;

  Poset &canonify();

  // normalize
  Poset &normalize();

  // remove less
  bool is_redundant(const uint16_t i, const uint16_t j) const;

  std::unordered_set<Poset> remove_less(const uint16_t i, const uint16_t j,
                                        const std::function<bool(const Poset &)> &test) const;

  // enlarge
  bool can_reduce_element_greater(const uint8_t element) const;

  void enlarge_n(std::unordered_set<Poset> &result) const;

  bool can_reduce_element_less(const uint8_t element) const;

  void enlarge_nk(std::unordered_set<Poset> &result) const;

  static std::unordered_set<Poset> filter(const std::unordered_set<Poset> &unfiltered);

  static std::unordered_set<Poset> enlarge(const std::unordered_set<Poset> &setOfPosets, const int n, const int k);

  // debug
  friend std::ostream &operator<<(std::ostream &os, const Poset &poset);

  bool operator==(const Poset &poset) const;

  std::size_t hash() const;
};

template <>
struct std::hash<Poset> {
  std::size_t operator()(const Poset &poset) const { return poset.hash(); }
};