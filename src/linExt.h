// copied from https://github.com/CodeCrafter47/sortinglowerbounds/blob/92865960ba465e4b6e068b400da82ff3f12af803/src/linExtCalculator.h
// TODO: adjust for selection
#pragma once

#include <bits/stdc++.h>
#include <x86intrin.h>

constexpr unsigned int MAXN = 15;

#define BRANCHLESS

struct alignas(8) UdSetItem32 {
  typedef uint32_t ValueType;
  typedef int32_t SignedType;
  ValueType downVal;
  ValueType upVal;
};

struct alignas(8) UdSetItemFull {
  typedef uint64_t ValueType;
  typedef int64_t SignedType;
  ValueType downVal;
  ValueType upVal;
};

template <class UdSetItem, typename linExtTableType, bool fixN, bool AVX32bit>
class LinearExtensionCalculatorInternal;

class LinearExtensionCalculator {
 public:
  std::array<std::array<uint64_t, MAXN>, MAXN> linExtTable;

 private:
  std::array<std::array<uint32_t, MAXN>, MAXN> linExtTable32;
  int C;

  LinearExtensionCalculatorInternal<UdSetItemFull, uint64_t, false, false>* internalCalcFull;
  LinearExtensionCalculatorInternal<UdSetItem32, uint32_t, true, true>* internalCalc32;

 public:
  LinearExtensionCalculator(unsigned int N, unsigned int cc);

  ~LinearExtensionCalculator();

  /**
   * Calculates the number of linear extensions on a graph, by reducing it to contain 2 singletons at max
   * and computes for the the missing values for t[j,k] in a faster way.
   * Uses calculateLinExtensions() for the reduced graph.
   *
   * @return e_p Number of linear extensions
   */
  uint64_t calculateLinExtensionsSingleton(PosetHandle& poset, unsigned int c, bool fillTable, bool overflowCheck);
};

template <class UdSetItem, typename linExtTableType, bool fixN, bool AVX32bit>
class LinearExtensionCalculatorInternal {
  UdSetItem* udSetVectorVal;
  uint32_t* udSetVectorSet;
  std::array<std::array<linExtTableType, MAXN>, MAXN>& linExtTable;

 public:
  uint64_t allocatedMemorySize = 0;

  LinearExtensionCalculatorInternal(int N, std::array<std::array<linExtTableType, MAXN>, MAXN>& linETab)
      : linExtTable(linETab) {}

  void setPointer(void* pointer, void* pointer2) {
    udSetVectorVal = (UdSetItem*)pointer;
    udSetVectorSet = (uint32_t*)pointer2;
  }

  void* getPointer() { return udSetVectorVal; }

  void* getPointer2() { return udSetVectorSet; }

  template <bool overflowCheck>
  uint64_t calculateLinExtensionsNew(PosetObj& poset, bool fillTable, unsigned int nn = 0) {
    std::array<std::array<linExtTableType, MAXN>, MAXN>& t = linExtTable;
    const unsigned int n = fixN ? NCT::N : nn;
    if constexpr (fixN) assert(nn == 0);

    for (int i = 0; i < MAXN; i++) {
      for (int j = 0; j < MAXN; j++) {
        t[i][j] = 0;
      }
    }

    uint32_t inVertexMask[MAXN];
    for (int i = 0; i < n; i++) {
      inVertexMask[i] = 0;
      for (int j = n - 1; j >= 0; j--) {
        inVertexMask[i] <<= 1;
        int edge = poset.isEdge(j, i);
        inVertexMask[i] |= edge;
      }
    }

    // compute out vertex mask
    uint32_t outVertexMask[MAXN];

    for (int i = 0; i < n; i++) {
      outVertexMask[i] = 0;
      for (int j = n - 1; j >= 0; j--) {
        outVertexMask[i] <<= 1;
        int edge = poset.isEdge(i, j);
        outVertexMask[i] |= edge;
      }
    }

    const uint32_t set1 = (uint32_t(1) << n) - 1;  // set1 is all ones
    const uint32_t set0 = 0;                       // empty set

    // fill downset vector
    udSetVectorSet[0] = set0;
    udSetVectorVal[set0].downVal = 1;
    int lastEnd = 1;
    uint32_t endNode_mask = 1;
    int writeIndex = 1;
    for (int endNode = 0; endNode < n; endNode++, endNode_mask <<= 1) {
      if (lastEnd >= allocatedMemorySize / 2) {
        std::cout << "lastEnd: " << lastEnd << ", allocatedMemorySize: " << allocatedMemorySize << std::endl;
        assert(false);
      }

      if constexpr (overflowCheck) {
        constexpr uint32_t limit = std::numeric_limits<uint32_t>::max() / MAXN;
        uint32_t downSet = udSetVectorSet[lastEnd - 1];
        if (udSetVectorVal[downSet].downVal > limit) {
          return 0;
        }
      }

      for (int j = 0; j < lastEnd; j++) {
        if ((udSetVectorSet[j] | inVertexMask[endNode]) == udSetVectorSet[j]) {  // if downset
          udSetVectorSet[writeIndex] = udSetVectorSet[j] | endNode_mask;
          uint32_t curSet = udSetVectorSet[writeIndex];
          udSetVectorVal[curSet].downVal = udSetVectorVal[udSetVectorSet[j]].downVal;

          int i = __builtin_ctz(curSet);
          uint32_t curSetShift = curSet >> (i + 1);
          while (curSetShift) {
            uint32_t preCurSet = curSet & (~(uint32_t(1) << i));
            assert(preCurSet < curSet);

            if (!(preCurSet & outVertexMask[i])) {
              udSetVectorVal[curSet].downVal += udSetVectorVal[preCurSet].downVal;
            }

            int shift = __builtin_ctz(curSetShift) + 1;
            curSetShift >>= shift;
            i += shift;
          }

          writeIndex++;
        }
      }
      lastEnd = writeIndex;
    }

    int numsets = lastEnd;
    int lastSet = numsets - 1;
    assert(udSetVectorSet[lastSet] == set1);

    Stats::addVal<AVMSTAT::NDownSets>(numsets);

    if (!fillTable) {
      return udSetVectorVal[set1].downVal;
    }

    udSetVectorVal[set1].upVal = 1;
    for (int curWriteIndex = lastSet - 1; curWriteIndex >= 0; curWriteIndex--) {
      uint32_t curSet = udSetVectorSet[curWriteIndex];
      udSetVectorVal[curSet].upVal = 0;

      uint32_t curSetShift = (~curSet) & set1;
      int i = -1;
      while (curSetShift) {
        int shift = __builtin_ctz(curSetShift);
        curSetShift >>= shift + 1;
        i += shift + 1;
        uint32_t preCurSet = curSet | (uint32_t(1) << i);

        if ((curSet | inVertexMask[i]) == curSet) {  // if preCurSet is down set
          udSetVectorVal[curSet].upVal += udSetVectorVal[preCurSet].upVal;

          // calculation of t[j,k]
          typename UdSetItem::ValueType d_v = udSetVectorVal[curSet].downVal;  //   table[idx][0];
          typename UdSetItem::ValueType u_w = udSetVectorVal[preCurSet].upVal;
          typename UdSetItem::ValueType product = d_v * u_w;

          {
#ifdef BRANCHLESS
            uint32_t bstringShift = ~(preCurSet >> (i + 1));
            for (int k = i + 1; k < n; k++, bstringShift >>= 1) {  // only fill upper triangle of t
                                                                   // checks if u_k not in W
              t[i][k] += product & (-((typename UdSetItem::SignedType)bstringShift & 1));
            }
#else
            uint32_t k_mask = uint32_t(1) << (i + 1);
            for (int k = i + 1; k < n; k++, k_mask <<= 1) {  // only fill upper triangle of t
                                                             // checks if u_k not in W
              if (!(preCurSet & k_mask)) {
                t[i][k] += product;
              }
            }
#endif
          }
        }
      }
    }

    uint64_t e_p = udSetVectorVal[set1].downVal;
    return e_p;
  }
};

template <bool fixN, typename T>
void fillFullTable(std::array<std::array<uint64_t, MAXN>, MAXN>& target, std::array<std::array<T, MAXN>, MAXN>& source,
                   uint64_t e_p, unsigned int nn = 0) {
  const unsigned int n = fixN ? NCT::N : nn;
  std::array<std::array<uint64_t, MAXN>, MAXN>& t = target;
  for (int i = 1; i < n; i++) {  // calculate lower triangle
    for (int j = 0; j < i; j++) {
      t[j][i] = source[j][i];
      t[i][j] = e_p - source[j][i];
      DEBUG_ASSERT((T)t[j][i] == source[j][i]);
    }
  }
}

template <bool fixN>
void fillFullTable(std::array<std::array<uint64_t, MAXN>, MAXN>& t, uint64_t e_p, unsigned int nn = 0) {
  const unsigned int n = fixN ? NCT::N : nn;

  for (int i = 1; i < n; i++) {  // calculate lower triangle
    for (int j = 0; j < i; j++) {
      t[i][j] = e_p - t[j][i];
    }
  }
}

LinearExtensionCalculator::LinearExtensionCalculator(unsigned int N, unsigned int cc) : linExtTable(), C(cc) {
  assert(N == NCT::N);

  internalCalcFull = new LinearExtensionCalculatorInternal<UdSetItemFull, uint64_t, false, false>(N, linExtTable);
  internalCalc32 = new LinearExtensionCalculatorInternal<UdSetItem32, uint32_t, true, true>(N, linExtTable32);

  size_t newTempSize = pow(1.74, N + 4);

  void* pointer = std::malloc(sizeof(UdSetItemFull) * (1ULL << MAXN));
  void* pointer2 = std::malloc(sizeof(uint32_t) * newTempSize);
  internalCalcFull->allocatedMemorySize = newTempSize;
  internalCalc32->allocatedMemorySize = newTempSize;

  internalCalcFull->setPointer(pointer, pointer2);
  internalCalc32->setPointer(pointer, pointer2);
}

uint64_t LinearExtensionCalculator::calculateLinExtensionsSingleton(PosetHandle& poset, unsigned int c, bool fillTable,
                                                                    bool overflowCheck) {
  int n = NCT::N;

  if (poset.GetnumSingletons() <= 1) {
    uint64_t e_p;
    if (overflowCheck && C - c < 27) {
      e_p = internalCalc32->calculateLinExtensionsNew<true>(*poset, fillTable);
      if (e_p == 0) {
        e_p = internalCalcFull->calculateLinExtensionsNew<false>(*poset, fillTable,
                                                                 n);  // sets linExtensions and linExtTable
        if (fillTable) {
          fillFullTable<false>(linExtTable, e_p, n);
        }
      } else if (fillTable) {
        fillFullTable<true>(linExtTable, linExtTable32, e_p);
      }
    } else if (!overflowCheck && C - c < 32) {
      e_p = internalCalc32->calculateLinExtensionsNew<false>(*poset, fillTable);  // sets linExtensions and linExtTable
      if (fillTable) fillFullTable<true>(linExtTable, linExtTable32, e_p);
    } else {
      e_p = internalCalcFull->calculateLinExtensionsNew<false>(*poset, fillTable,
                                                               n);  // sets linExtensions and linExtTable
      if (fillTable) fillFullTable<false>(linExtTable, e_p, n);
    }

    return e_p;
  } else {
    unsigned int reduced_n = NCT::N - poset.GetnumSingletons() + 1;

    uint64_t e_p = internalCalcFull->calculateLinExtensionsNew<false>(*poset, fillTable, reduced_n);
    if (fillTable) fillFullTable<false>(linExtTable, e_p, reduced_n);

    int k = poset.GetnumSingletons();
    uint64_t fac = fallingfactorial(n, reduced_n);  // one singleton remains --> +1
    e_p *= fac;                                     // adjust values with factorial
    std::array<std::array<uint64_t, MAXN>, MAXN>& t_reduced = linExtTable;
    for (unsigned int i = 0; i < reduced_n; i++) {
      for (unsigned int j = 0; j < reduced_n; j++) t_reduced[i][j] *= fac;
    }

    int lastIdx = reduced_n - 1;
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        if (i <= n - k && j <= n - k) {  // includes the one singeltons
          continue;                      //  new_t[i][j] = t_reduced[i][j]; //copy entry
        } else if (i < n - k && j > n - k) {
          t_reduced[i][j] = t_reduced[i][lastIdx];  // last entry of the row
          continue;
        } else if (i > n - k && j < n - k) {
          t_reduced[i][j] = t_reduced[lastIdx][j];  // last entry of the column
          continue;
        } else if (i >= n - k && j >= n - k) {
          if (i != j) {
            t_reduced[i][j] = e_p / 2;  // compare two singletons
          } else {
            t_reduced[i][j] = 0;
          }
        }
      }
    }
    return e_p;
  }
}

LinearExtensionCalculator::~LinearExtensionCalculator() {
  free(this->internalCalcFull->getPointer());
  free(this->internalCalcFull->getPointer2());
  delete this->internalCalcFull;
  delete this->internalCalc32;
}