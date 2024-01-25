#pragma once
#include <bits/stdc++.h>

#include "util.h"

class Poset {
 private:
  uint8_t n;
  uint8_t nthSmallest;

 public:
  std::bitset<MAX_N * MAX_N> comparisonTable;

  std::size_t getComparisonTableSize() const;

  std::size_t toInternalPos(const uint16_t i, const uint16_t j) const;

  void set_less(const uint16_t i, const uint16_t j, const bool value);

  Poset &add_and_close_recursive(const uint16_t i, const uint16_t j);

  Poset &reduce_n();

  Poset(const uint8_t n, const uint8_t nthSmallest);

  Poset(const Poset &poset);

  uint8_t size() const;

  uint8_t nth() const;

  bool is_less(const uint16_t i, const uint16_t j) const;

  Poset &add_less(const uint16_t i, const uint16_t j);

  Poset with_less(const uint16_t i, const uint16_t j) const;

  void calculate_relations(uint8_t less[], uint8_t greater[]) const;

  bool is_redundant(const uint16_t i, const uint16_t j) const;

  std::unordered_set<Poset> remove_less(const uint16_t i, const uint16_t j,
                                        const std::function<bool(const Poset &)> &test) const;

  Poset &dual();

  bool subset_of(const Poset &poset) const;

  void swap(const uint16_t i1, const uint16_t j1, const uint16_t i2, const uint16_t j2);

  void swap(const uint16_t i, const uint16_t j);

 private:
  std::array<int, MAX_N> canonify_nauty_indicies() const;

 public:
  Poset &canonify();

  Poset &normalize();

  bool operator==(const Poset &poset) const;

  std::size_t hash() const;

  bool can_reduce_element_greater(const uint8_t element) const;

  void enlarge_n(std::unordered_set<Poset> &result) const;

  bool can_reduce_element_less(const uint8_t element) const;

  void enlarge_nk(std::unordered_set<Poset> &result) const;

  static std::unordered_set<Poset> enlarge(const std::unordered_set<Poset> &setOfPosets, const int n, const int k);

  static std::unordered_set<Poset> filter(const std::unordered_set<Poset> &unfiltered);

  friend std::ostream &operator<<(std::ostream &os, const Poset &poset);
};

template <>
struct std::hash<Poset> {
  std::size_t operator()(const Poset &poset) const { return poset.hash(); }
};