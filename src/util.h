#pragma once
#include <bits/stdc++.h>
using namespace std;

template <class T>
struct is_map {
  static constexpr bool value = false;
};

template <class Key, class Value>
struct is_map<map<Key, Value>> {
  static constexpr bool value = true;
};

template <class T>
struct is_stack {
  static constexpr bool value = false;
};

template <class T>
struct is_stack<stack<T>> {
  static constexpr bool value = true;
};

template <class T>
struct is_queue {
  static constexpr bool value = false;
};

template <class T>
struct is_queue<queue<T>> {
  static constexpr bool value = true;
};

template <class T>
struct is_pair {
  static constexpr bool value = false;
};

template <class F, class G>
struct is_pair<pair<F, G>> {
  static constexpr bool value = true;
};

template <class T>
struct is_set {
  static constexpr bool value = false;
};

template <class T>
struct is_set<set<T>> {
  static constexpr bool value = true;
};

template <typename T>
ostream &printContainer(ostream &os, T container) {
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
ostream &operator<<(ostream &os, const map<F, G> &container) {
  return printContainer(os, container);
}

template <typename T>
ostream &operator<<(ostream &os, const stack<T> &container) {
  return printContainer(os, container);
}

template <typename T>
ostream &operator<<(ostream &os, const queue<T> &container) {
  return printContainer(os, container);
}

template <typename T>
ostream &operator<<(ostream &os, const vector<T> &container) {
  return printContainer(os, container);
}

template <typename T>
ostream &operator<<(ostream &os, const list<T> &container) {
  return printContainer(os, container);
}

template <typename F, typename G>
ostream &operator<<(ostream &os, const pair<F, G> &container) {
  return printContainer(os, container);
}

template <typename F, typename G>
ostream &operator<<(ostream &os, const set<F, G> &container) {
  return printContainer(os, container);
}

template <typename T>
size_t minIndex(const vector<T> &vector) {
  size_t minIndex = 0;
  for (size_t i = 1; i < vector.size(); ++i) {
    if (vector[i] < vector[minIndex]) minIndex = i;
  }
  return minIndex;
}

template <typename T>
size_t maxIndex(const vector<T> &vector) {
  size_t maxIndex = 0;
  for (size_t i = 1; i < vector.size(); ++i) {
    if (vector[maxIndex] < vector[i]) maxIndex = i;
  }
  return maxIndex;
}

template <typename T>
T min(const vector<T> &vector) {
  return vector[minIndex(vector)];
}

template <typename T>
T max(const vector<T> &vector) {
  return vector[maxIndex(vector)];
}

inline void printTime(const std::chrono::_V2::steady_clock::time_point &startPoint,
                      const std::chrono::_V2::steady_clock::time_point &endPoint) {
  std::cout << (std::chrono::duration_cast<std::chrono::milliseconds>(endPoint - startPoint).count() / 1000.0)
            << " seconds";
}

inline void measure(const std::string &message, const std::function<void()> &function) {
  auto start = std::chrono::steady_clock::now();
  function();
  auto end = std::chrono::steady_clock::now();
  auto time = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
  std::cout << "time '" << message << "': " << (time / 1000.0) << " seconds";
}

class StopWatch {
 private:
  const std::chrono::_V2::steady_clock::time_point startPoint;

 public:
  StopWatch() : startPoint(std::chrono::steady_clock::now()) {}

  friend ostream &operator<<(ostream &os, const StopWatch &watch);
};

ostream &operator<<(ostream &os, const StopWatch &watch) {
  const auto endPoint = std::chrono::steady_clock::now();
  os << (chrono::duration_cast<chrono::milliseconds>(endPoint - watch.startPoint).count() / 1000.0) << "s";
  return os;
}