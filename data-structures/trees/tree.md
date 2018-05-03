# Tree

A tree is a data structure made up of nodes (vertices) and edges without having any cycle.

The tree with no nodes is called the null or empty tree. A tree that is not empty consists of a root node and potentially many levels of additional nodes that form a hierarchy.

The tree data structure is a constrained directed graph, where constrains are:
- a node has at most a single parent
- no node in the tree point to the root
- every node (other than the root) must have exactly one parent
- the root must have no parents

Although, given a list of nodes, and for each node a list of references to its children, one cannot tell if this structure is a tree without analyzing its global structure and that it is in fact topologically a tree.


## Terminology used in trees

- Root: The top node in a tree.
- Child: A node directly connected to another node when moving away from the Root.
- Parent: The converse notion of a child.
- Siblings: A group of nodes with the same parent.
- Descendant: A node reachable by repeated proceeding from parent to child.
- Ancestor: A node reachable by repeated proceeding from child to parent.
- Leaf: (less commonly called External node) A node with no children.
- Branch (Internal node): A node with at least one child.
- Degree: The number of subtrees of a node.
- Edge: The connection between one node and another.
- Path: A sequence of nodes and edges connecting a node with a descendant.
- Level: The level of a node is defined by 1 + (the number of connections between the node and the root).
- Height of node: The height of a node is the number of edges on the longest path between that node and a leaf.
- Height of tree: The height of a tree is the height of its root node.
- Depth: The depth of a node is the number of edges from the tree's root node to the node.
- Forest: a set of `n >= 0` disjoint trees.


In type theory, the abstract tree type `T` with values of some type `E` is defined, using the abstract forest type `F` (list of trees), by the functions:

```
value: T → E
children: T → F
nil: () → F
node: E × F → T
```

with the axioms:

```
value(node(e, f)) = e
children(node(e, f)) = f
```

In terms of type theory, a tree is an inductive type defined by the constructors nil (empty forest) and node (tree with root node with given value and children).


- [Data Trees as a Means of Presenting Complex Data A](http://www.community-of-knowledge.de/beitrag/data-trees-as-a-means-of-presenting-complex-data-analysis/)
- [Description](https://xlinux.nist.gov/dads/HTML/tree.html)
- [Dictionary of Algorithms and Data Structures](https://www.wikiwand.com/en/Dictionary_of_Algorithms_and_Data_Structures)
- [data.tree](http://www.ipub.com/data.tree)
- [WormWeb.org: Interactive Visualization of the C. e](http://wormweb.org/celllineage)
- [Binary Trees by L. Allison](http://www.allisons.org/ll/AlgDS/Tree/)




## Complete Tree
A balanced tree in which the distance from the root to any leaf is either h or h-1.



---

https://www.wikiwand.com/en/Tree_traversal