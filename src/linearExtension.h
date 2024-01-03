#include <bits/stdc++.h>

#include "poset.h"

std::unordered_set<int> to_set(uint16_t value) {
  std::unordered_set<int> result;
  for (uint8_t i = 0; i < 16; ++i, value >>= 1) {
    if (value & 0b1) {
      result.insert(i);
    }
  }
  return result;
}

template <std::size_t maxN>
bool is_down_set(const uint16_t D, const uint16_t U, const Poset<maxN> &poset) {
  const std::size_t n = poset.size();

  if ((D | U) != U) {
    return false;
  }
  for (uint8_t x = 0; x < n; ++x) {
    for (uint8_t y = 0; y < n; ++y) {
      if (((D >> x) & 0b1) && ((U >> y) & 0b1) && ((1 << y) & ~D) && poset.is_less(y, x)) {
        return false;
      }
    }
  }

  return true;
}

template <std::size_t maxN>
std::vector<uint16_t> get_down_sets(const uint16_t U, const Poset<maxN> &poset) {
  const std::size_t n = poset.size();

  std::vector<uint16_t> result;
  result.push_back(0);
  for (uint8_t i = 0; i < n; ++i) {
    std::vector<uint16_t> temp;
    for (uint16_t item : result) {
      item <<= 1;
      temp.push_back(item);
      item |= 1;
      temp.push_back(item);
    }
    std::swap(result, temp);
  }

  std::vector<uint16_t> downsets;
  for (const uint16_t D : result) {
    if (is_down_set(D, U, poset)) {
      downsets.push_back(D);
    }
  }
  return downsets;
}

void dfs(const int n, const std::vector<std::vector<int>> &adj, const int v, std::vector<int> &d1,
         std::vector<bool> &visited) {
  visited[v] = true;
  d1[v] = (n == v) ? 1 : 0;
  for (const int w : adj[v]) {
    if (!visited[w]) {
      dfs(n, adj, w, d1, visited);
    }
    d1[v] += d1[w];
  }
}

template <std::size_t maxN>
std::pair<int, std::vector<std::vector<uint64_t>>> get_linear_extensions(const Poset<maxN> &poset) {
  const std::size_t n = poset.size();
  assert(n <= 8 * sizeof(uint16_t) - 1);

  const uint16_t biggest_value = (1 << n);
  const uint16_t U = biggest_value - 1;
  const std::vector<uint16_t> downsets = get_down_sets(U, poset);

  std::vector<std::vector<int>> adj(biggest_value), adj_inv(biggest_value);
  for (const uint16_t D1 : downsets) {
    for (uint8_t x = 0; x < n; ++x) {
      if (((U >> x) & 0b1) && !((D1 >> x) & 0b1)) {
        const uint16_t D2 = D1 | (1 << x);
        if (is_down_set(D2, U, poset)) {
          adj_inv[D1].push_back(D2);
          adj[D2].push_back(D1);
        }
      }
    }
  }

  std::vector<int> d1(biggest_value);
  std::vector<bool> visited(biggest_value);
  dfs(0, adj, U, d1, visited);

  std::vector<int> u1(biggest_value);
  std::vector<bool> visited2(biggest_value);
  dfs(U, adj_inv, 0, u1, visited2);

  std::vector<std::vector<uint64_t>> table(n, std::vector<uint64_t>(n, 0));
  for (int j = 0; j < n; ++j) {
    for (int k = 0; k < n; ++k) {
      if (j != k)
        for (int v = 0; v < biggest_value; ++v) {
          const int w = v | (1 << j);
          if (std::find(adj_inv[v].begin(), adj_inv[v].end(), w) != adj_inv[v].end() && !((1 << k) & w)) {
            table[j][k] += d1[v] * u1[w];
          }
        }
    }
  }
  return {d1[U], table};
}