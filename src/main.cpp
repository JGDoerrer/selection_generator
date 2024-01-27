#include <bits/stdc++.h>
#include <cxxabi.h>
#include <execinfo.h>

#include "cache_set.h"
#include "poset.h"
#include "util.h"

template <class T>
struct is_map {
  static constexpr bool value = false;
};

template <class Key, class Value>
struct is_map<std::map<Key, Value>> {
  static constexpr bool value = true;
};

template <class T>
struct is_stack {
  static constexpr bool value = false;
};

template <class T>
struct is_stack<std::stack<T>> {
  static constexpr bool value = true;
};

template <class T>
struct is_queue {
  static constexpr bool value = false;
};

template <class T>
struct is_queue<std::queue<T>> {
  static constexpr bool value = true;
};

template <class T>
struct is_pair {
  static constexpr bool value = false;
};

template <class F, class G>
struct is_pair<std::pair<F, G>> {
  static constexpr bool value = true;
};

template <class T>
struct is_set {
  static constexpr bool value = false;
};

template <class T>
struct is_set<std::set<T>> {
  static constexpr bool value = true;
};

template <class T>
struct is_set<std::unordered_set<T>> {
  static constexpr bool value = true;
};

template <typename T>
std::ostream &print_container(std::ostream &os, T container) {
  if constexpr (is_map<T>::value) {
    os << '[';
    bool isNotFirst = false;
    for (const auto &[k, v] : container) {
      if (isNotFirst) os << ", ";
      isNotFirst = true;
      os << k << ": " << v;
    }
    os << ']';
  } else if constexpr (is_stack<T>::value || is_queue<T>::value) {
    os << '<';
    bool isNotFirst = false;
    while (!container.empty()) {
      if (isNotFirst) os << ", ";
      isNotFirst = true;
      if constexpr (is_stack<T>::value)
        os << container.top();
      else if constexpr (is_queue<T>::value)
        os << container.front();
      container.pop();
    }
    os << '>';
  } else if constexpr (is_pair<T>::value)
    os << '(' << container.first << ", " << container.second << ')';
  else  // copy(container.begin(), container.end(), ostream_iterator<typename
        // T::value_type>(os, ", "));
  {
    if constexpr (is_set<T>::value)
      os << '{';
    else
      os << '[';
    bool isNotFirst = false;
    for (const auto &data : container) {
      if (isNotFirst) os << ", ";
      isNotFirst = true;
      os << data;
    }
    if constexpr (is_set<T>::value)
      os << '}';
    else
      os << ']';
  }
  return os;
}

template <typename F, typename G>
std::ostream &operator<<(std::ostream &os, const std::map<F, G> &container) {
  return print_container(os, container);
}

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::stack<T> &container) {
  return print_container(os, container);
}

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::queue<T> &container) {
  return print_container(os, container);
}

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::vector<T> &container) {
  return print_container(os, container);
}

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::list<T> &container) {
  return print_container(os, container);
}

template <typename F, typename G>
std::ostream &operator<<(std::ostream &os, const std::pair<F, G> &container) {
  return print_container(os, container);
}

template <typename F, typename G>
std::ostream &operator<<(std::ostream &os, const std::set<F, G> &container) {
  return print_container(os, container);
}

template <typename F, typename G>
std::ostream &operator<<(std::ostream &os, const std::unordered_set<F, G> &container) {
  return print_container(os, container);
}

std::ostream &operator<<(std::ostream &os, const std::chrono::nanoseconds &nanos) {
  os << (std::chrono::duration_cast<std::chrono::milliseconds>(nanos).count() / 1000.0) << "s";
  return os;
}

enum SearchResult : uint8_t { FoundSolution, NoSolution, Unknown };

std::tuple<std::optional<int>, std::chrono::nanoseconds, std::chrono::nanoseconds> startSearchBackward(
    CacheSetSingle<true> &poset_cache, const uint8_t n, const uint8_t nthSmallest, const int maxComparisons) {
  std::chrono::nanoseconds duration_build_posets_total{}, duration_test_posets_total{};
  std::unordered_set<Poset> source{{Poset(1, 0)}};
  for (int k = 1; k < maxComparisons; ++k) {
    std::chrono::nanoseconds duration_build_posets{}, duration_test_posets{};
    const auto start = std::chrono::high_resolution_clock::now();
    const auto source_new = Poset::enlarge(source, n, nthSmallest);
    const auto mid = std::chrono::high_resolution_clock::now();
    duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    std::unordered_set<Poset> destination;
    for (const Poset &item : source_new) {
      for (uint8_t i = 0; i < n; ++i) {
        for (uint8_t j = 0; j < n; ++j) {
          if (item.is_less(i, j)) {
            for (const Poset &predecessor : item.remove_less(
                     i, j, [&poset_cache, k](const Poset &poset) { return poset_cache.check(poset, k - 1); })) {
              if (predecessor == Poset{(uint8_t)n, (uint8_t)nthSmallest}) {
                duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
                duration_test_posets_total += duration_test_posets;
                std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in "
                          << duration_build_posets << " ~ " << duration_test_posets
                          << " | total cached: " << poset_cache.size() << " (found solution)" << std::endl;
                return {k, duration_build_posets_total, duration_test_posets_total};
              }

              Poset predecessor_normalized = predecessor;
              predecessor_normalized.normalize();
              if (poset_cache.check(predecessor_normalized, k - 1)) {
                continue;
              }

              destination.insert(predecessor_normalized);
              poset_cache.insert(predecessor_normalized, k);
            }
          }
        }
      }
    }
    duration_test_posets = std::chrono::high_resolution_clock::now() - mid;
    duration_test_posets_total += duration_test_posets;

    std::cout << "# " << k << ": " << source.size() << " => " << source_new.size() << " in " << duration_build_posets
              << " ~ " << duration_test_posets << " | total cached: " << poset_cache.size() << std::endl;

    source = destination;
  }

  return {std::nullopt, duration_build_posets_total, duration_test_posets_total};
}

int main() {
  std::cout.setf(std::ios::fixed, std::ios::floatfield);
  std::cout.precision(3);

  if constexpr (false) {
    CacheSetSingle<true> poset_cache;
    poset_cache.insert(Poset(1, 0), 0);

    const int n = 9;
    const int nthSmallest = 4;

    const auto &[comparisons, durationGeneratePosets, durationSearch] =
        startSearchBackward(poset_cache, n, nthSmallest, n * n);

    if (comparisons.has_value()) {
      std::cout << "\rtime '" << durationGeneratePosets << " + " << durationSearch << " = "
                << durationGeneratePosets + durationSearch << "': n = " << n << ", i = " << nthSmallest
                << ", comparisons: " << comparisons.value() << std::endl;
      if (comparisons != min_n_comparisons[n][nthSmallest]) {
        std::cerr << "Error: got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
                  << std::endl;
        exit(0);
      }
    } else {
      std::cerr << "Error: got 'nothing' but expected " << min_n_comparisons[n][nthSmallest] << std::endl;
      exit(0);
    }
  } else {
    constexpr size_t nBound = 0;

    CacheSetSingle<true> poset_cache;
    poset_cache.insert(Poset(1, 0), 0);

    for (int n = 2; n < 15; ++n) {
      for (int nthSmallest = 0; nthSmallest < (n + 1) / 2; ++nthSmallest) {
        const auto &[comparisons, durationGeneratePosets, durationSearch] =
            startSearchBackward(poset_cache, n, nthSmallest, n * n);

        if (comparisons.has_value()) {
          if (n >= nBound) {
            std::cout << "\rtime '" << durationGeneratePosets << " + " << durationSearch << " = "
                      << durationGeneratePosets + durationSearch << "': n = " << n << ", i = " << nthSmallest
                      << ", comparisons: " << comparisons.value() << std::endl;
          }
          if (comparisons != min_n_comparisons[n][nthSmallest]) {
            std::cerr << "Error: got " << comparisons.value() << ", but expected " << min_n_comparisons[n][nthSmallest]
                      << std::endl;
            exit(0);
          }
        } else {
          std::cerr << "Error: got 'nothing' but expected " << min_n_comparisons[n][nthSmallest] << std::endl;
          exit(0);
        }
      }
      if (n >= nBound) std::cout << std::endl;
    }
  }
}