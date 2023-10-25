#include <bits/stdc++.h>

#include "util.h"
using namespace std;

enum State : uint8_t {
  Unknown,
  Lesser,
  GreaterEqual
  // LesserEqual,
  // Equal,
  // Greater
};

class Poset {
 private:
  const int n;
  const int nthSmallest;
  int comparisonsDone = 0;
  bool comparisonTable[maxN * maxN];

  inline void setTrue(const int i, const int j) {
    comparisonTable[i * n + j] = true;
  }

  inline bool get(const int i, const int j) const {
    return comparisonTable[i * n + j];
  }

  void addComparisonTransitiv(const int i, const int j) {
    setTrue(i, j);

    for (int k = 0; k < n; ++k) {
      if (get(j, k) && !get(i, k)) {
        addComparisonTransitiv(i, k);
      }
      if (get(k, i) && !get(k, j)) {
        addComparisonTransitiv(k, j);
      }
    }
  }

 public:
  Poset(const int n, const int nthSmallest)
      : n(n), nthSmallest(nthSmallest) {
    for (int i = 0; i < n * n; ++i) {
      comparisonTable[i] = false;
    }
  };

  ~Poset(){};

  int size() const { return n; }

  int nthSmallest1() const { return nthSmallest; }

  // true => arr[i] < arr[j]
  bool is(int i, int j) const { return get(i, j); }

  // after func it holds: arr[i] < arr[j]
  void addComparison(const int i, const int j) {
    ++comparisonsDone;
    addComparisonTransitiv(i, j);
  }

  // can one determine n smallest element with current comparisons
  bool canDetermineNSmallest() const {
    for (int k = 0; k < n; ++k) {  // is arr[k] medium?
      int smaller = 0, equals = 0, bigger = 0, idk = 0;
      for (int i = 0; i < n; ++i) {
        if (get(i, k) && get(k, i)) {
          ++equals;
        } else if (get(i, k)) {
          ++smaller;
        } else if (get(k, i)) {
          ++bigger;
        } else {
          ++idk;
        }
      }
      if (idk == 1 && smaller <= nthSmallest &&
          bigger <= (n - 1) - nthSmallest) {
        return true;
      }
    }
    return false;
  }

  bool operator==(const Poset &tb) const {
    if (n != tb.n || nthSmallest != nthSmallest) {
      return false;
    }
    for (int i = 0; i < n * n; ++i) {
      if (comparisonTable[i] != tb.comparisonTable[i]) {
        return false;
      }
    }
    if (comparisonsDone != tb.comparisonsDone) {
      return false;
    }
    return true;
  }

  size_t hash() const {
    size_t result = std::hash<int>{}(n) ^ std::hash<int>{}(nthSmallest);
    result ^= std::hash<int>{}(comparisonsDone);
    for (int i = 0; i < n * n; ++i) {
      if (comparisonTable[i]) {
        result = result ^ std::hash<int>{}(i);
      }
    }
    // ((hash<string>()(k.first) ^ (hash<string>()(k.second) << 1)) >> 1) ^
    // (hash<int>()(k.third) << 1);
    return result;
  }
};

template <>
struct std::hash<Poset> {
  size_t operator()(const Poset &poset) const { return poset.hash(); }
};