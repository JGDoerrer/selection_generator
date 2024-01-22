#include <boost/graph/adjacency_matrix.hpp>
#include <boost/graph/vf2_sub_graph_iso.hpp>
#include <iostream>

struct Graph : public boost::adjacency_matrix<boost::directedS> {
  using Impl = boost::adjacency_matrix<boost::directedS>;
  // using Impl::Impl;
  // using Impl::operator=;
};

namespace boost {
template <>
struct graph_traits<Graph> : graph_traits<Graph::Impl> {
  struct traversal_category : boost::bidirectional_graph_tag, Graph::Impl::traversal_category {};
};

// O(2N)
auto degree(Graph::vertex_descriptor u, Graph const& g) { return size(out_edges(u, g)) + size(in_edges(u, g)); }
};  // namespace boost

int main() {
  Graph g_small(4); // not working with 5?!

  add_edge(0, 1, g_small);
  add_edge(1, 2, g_small);
  add_edge(2, 0, g_small);

  Graph g_large(5);

  add_edge(0, 1, g_large);
  add_edge(1, 2, g_large);
  add_edge(2, 3, g_large);
  add_edge(3, 4, g_large);
  add_edge(4, 2, g_large);

  bool is_iso = false;
  vf2_subgraph_iso(g_small, g_large, [&](auto, auto) {
    is_iso = true;
    return false;
  });

  std::cout << "ex. subgraph isomorphism: " << is_iso << std::endl;
}