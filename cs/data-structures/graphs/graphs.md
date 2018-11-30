# Graph terms


## Graph elements:
- vertex, node
- edge

## Graph components:
* graph
  - directed
  - undirected
  - cyclic
  - acyclic
  - rooted
  - unrooted
* vertex
  - degree
* edge
  - saturated
  - walk
  - path
* path
  - augmenting
  - alternating
  - directed
  - cyclic
* tree
  - branching
  - spanning
* ﬂow 
  - blocking


- **Degree** the number of edges incident to a vertex.
- **Matching** a subset of edges that do not share a common vertex.
- **Leaves** Vertices of degree 1. Usually wrt trees.
- **Cycle** A path in which the start and end vertices are the same.
- **Tree** An acyclic graph, with `|V| − 1` edges.
- **Path** ordered sequence of edges such that any two consecutive edges
  are incident to a common vertex.
- **Walk** ordered sequence of, possibly repeating, edges such that any two consecutive edges are incident to a common vertex.


- Perfect matching: 
  matching in which every node is matched by an edge to another node.
- Augmenting path: 
  alternating path that can be used to augment the size of a matching.
- Branching tree: 
  spanning tree in a rooted graph, such that root has a path to each vertex.
- Minimum spanning tree: 
  spanning tree of minimum total weight
- DFS forest: 
  rooted forest formed by DFS
- Blocking ﬂow:
  ﬂow fn in which any directed path from s to t contains a saturated edge.
- Network ﬂow: 
  assignment of ﬂow values to the edges of a graph that satisfes ﬂow 
  conservation, skew symmetry, and capacity constraints
- s–t cut:
  partitioning of the vertex set into S and T such that `s ? S and t ? T`
- Topological order: 
  linear ordering of the edges of a DAG such that every edge in the graph goes 
  from left to right.


Types of graphs:
* Connected: graph with a path between each pair of vertices.
* Forest: An acyclic graph.
* Directed acyclic graph (DAG): directed graph with no cycles.
* Sparse graph: graph in which `|E| ? |V|2`.
* Eulerian graph: graph that has an Euler tour
* Biconnected: cannot be disconnected by removal of a single vertex.
* Bipartite: 
  graph in which the vertex set can be partitioned into two sets X and Y, such that each edge connects a node in X with a node in Y.
* Strongly connected: 
  directed graph in which there is a directed path in each direction between 
  each pair of vertices.

Graph related problems:
* Assignment problem: 
  finding a perfect matching of maximum (or minimum) total weight.
* Chinese postman problem: 
  minimum length tour that traverses each edge at least once.
* Euler tour problem: 
  traversal of the edges but visiting each edge exactly once.
* Traveling salesman problem: 
  minimum length tour that visits all of the vertices exactly once.
