#pragma once
#include <bits/stdc++.h>
#include <cxxabi.h>
#include <execinfo.h>

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

inline void print_time(const std::chrono::_V2::steady_clock::time_point &startPoint,
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

  friend std::ostream &operator<<(std::ostream &os, const StopWatch &watch);
};

std::ostream &operator<<(std::ostream &os, const std::chrono::nanoseconds &nanos) {
  os << (std::chrono::duration_cast<std::chrono::milliseconds>(nanos).count() / 1000.0) << "s";
  return os;
}

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::chrono::duration<T> &duration) {
  os << (std::chrono::duration_cast<std::chrono::milliseconds>(duration).count() / 1000.0) << "s";
  return os;
}

std::ostream &operator<<(std::ostream &os, const StopWatch &watch) {
  const auto endPoint = std::chrono::steady_clock::now();
  os << endPoint - watch.startPoint;
  return os;
}

// template<>
// struct std::hash<uint16_t> {
//   size_t operator()(const uint16_t value) const { return value; }
// };

// template <typename T, typename G>
// struct std::hash<std::pair<T, G>> {
//   size_t operator()(const std::pair<T, G> &pair) const { return std::hash<T>(pair.first) ^ std::hash<G>(pair.second);
//   }
// };

template <>
struct std::hash<std::pair<uint16_t, uint16_t>> {
  size_t operator()(const std::pair<uint16_t, uint16_t> &pair) const { return pair.first ^ pair.second; }
};

void printStackTrace() {
  const int maxFrames = 20;  // You can adjust this based on your needs
  void *callstack[maxFrames];
  int frames = backtrace(callstack, maxFrames);
  char **symbols = backtrace_symbols(callstack, frames);

  std::cout << "Stack Trace:" << std::endl;
  for (int i = 0; i < frames; ++i) {
    // Demangle C++ function names
    size_t sz = 256;
    char *function = static_cast<char *>(malloc(sz));
    if (function == nullptr) {
      continue;
    }
    int status;
    char *demangled = abi::__cxa_demangle(symbols[i], function, &sz, &status);
    if (status == 0) {
      function = demangled;  // Use demangled name if available
    }

    std::cout << "[" << i << "] " << function << std::endl;
    free(function);
  }

  free(symbols);
}
