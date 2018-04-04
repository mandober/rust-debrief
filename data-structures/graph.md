# Graphs

A graph is an ADT that is meant to implement the (undirected or directed) graph concepts from mathematics, specifically the field of graph theory.

A graph data structure consists of a finite (and possibly mutable) set of vertices (nodes, points) together with a set of unordered pairs of these vertices for an undirected graph or a set of ordered pairs for a directed graph.

These pairs are known as edges (arcs, lines) for an undirected graph and as arrows (directed edges, directed arcs, directed lines) for a directed graph.

The vertices may be part of the graph structure, or may be external entities represented by integer indices or references.

A graph data structure may also associate to each edge some edge value, such as a symbolic label or a numeric attribute (cost, capacity, length, etc.).


## Operations

The basic operations provided by a graph (G) usually include:
- `adjacent(G, x, y)`: tests whether there's an edge from the vertex x to the vertex y;
- `neighbors(G, x)`: lists all vertices y such that there is an edge from the vertex x to the vertex y;
- `add_vertex(G, x)`: adds the vertex x, if it is not there;
- `remove_vertex(G, x)`: removes the vertex x, if it is there;
- `add_edge(G, x, y)`: adds the edge from the vertex x to the vertex y, if it is not there;
- `remove_edge(G, x, y)`: removes the edge from the vertex x to the vertex y, if it is there;
- `get_vertex_value(G, x)`: returns the value associated with the vertex x;
- `set_vertex_value(G, x, v)`: sets the value associated with the vertex x to v.

If edges have associated values:
- `get_edge_value(G, x, y)`: returns the value associated with the edge (x, y);
- `set_edge_value(G, x, y, v)`: sets the value associated with the edge (x, y) to v.

Algorithms in graphs include finding a path between two nodes, finding the shortest path between two nodes, determining cycles in the graph (a cycle is a non-empty path from a node to itself), finding a path that reaches all nodes (the famous "traveling salesman problem"), and so on. Sometimes the nodes or arcs of a graph have weights or costs associated with them, and we are interested in finding the cheapest path.


## Representations

Different data structures for the representation of graphs are used in practice:

### Adjacency list
Vertices are stored as records or objects, and every vertex stores a list of adjacent vertices. This data structure allows the storage of additional data on the vertices. Additional data can be stored if edges are also stored as objects, in which case each vertex stores its incident edges and each edge stores its incident vertices.

Adjacency lists are generally preferred because they efficiently represent sparse graphs. An adjacency matrix is preferred if the graph is dense, that is the number of edges is close to the number of vertices squared, or if one must be able to quickly look up if there is an edge connecting two vertices.

Slow to remove vertices and edges, because it needs to find all vertices or edges.

### Adjacency matrix
A two-dimensional matrix, in which the rows represent source vertices and columns represent destination vertices. Data on edges and vertices must be stored externally. Only the cost for one edge can be stored between each pair of vertices.

Slow to add or remove vertices, because matrix must be resized/copied.

### Incidence matrix
A two-dimensional Boolean matrix, in which the rows represent the vertices and columns represent the edges. The entries indicate whether the vertex at a row is incident to the edge at a column.

Slow to add or remove vertices and edges, because matrix must be resized/copied.


---

* [Graph traversal][1] for graph walking strategies
* [Graph database][2] for graph (data structure) persistency
* [Graph rewriting][3] for rule based transformations of graphs (graph data structures)
* [Graph drawing software][4] for software, systems, and providers of systems for drawing graphs

[1]: https://www.wikiwand.com/en/Graph_traversal "Graph traversal"
[2]: https://www.wikiwand.com/en/Graph_database "Graph database"
[3]: https://www.wikiwand.com/en/Graph_rewriting "Graph rewriting"
[4]: https://www.wikiwand.com/en/Graph_drawing_software "Graph drawing software"
