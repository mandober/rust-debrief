# Binary Search Tree

A binary tree is a type of tree where each node has left and right subtrees. All values in left subtree are lower then the root node, and all the values in the right subtree are greater than the value in the root node.







# BST is rooted

The tree has height (depth) that excludes the root (if there's only the root node: the height is 0; the root node is always at level 0).

h = height
Nmax/L = max Nodes per level  = 2^h
Nmax   = max Nodes in the tree = 2^(h+1) - 1
leftChild < Parent > rightChild



Algo   | Avg      | Worst
-------|----------|------
Space  | O(n)     | O(n)
Search | O(log n) | O(n)
Insert | O(log n) | O(n)
Delete | O(log n) | O(n)


## Add value
- start from the root node: tree.root
  - is tree.root ?
    - if no: add new node with val
    - if yes: let node = tree.root
  - compare val to node.val:
    - if eq return node
    - if lt go left: let node = node.left
    - if gt go right: let node = node.right


### Remove
- the node has no child => simply remove that node
- the node just has a left child => the left child of the removing node will take it's position on the tree
- the node has right child, and the right child does not have any left child => the right child of the node will take the position of the removing node on the tree
- the node has right child, and the right child also has left child => the most left child of the right child will be removed (removing this node will cause a recursive algorithm!) and take the position of the removing node.


# Binary Search Tree

A Binary Search Tree (BST) is a binary tree in which each vertex has only up to 2 children that satisfies BST property: All vertices in the left subtree of a vertex must hold a value smaller than its own and all vertices in the right subtree of a vertex must hold a value larger than its own (we have assumption that all values are distinct integers in this visualization and small tweak is needed to cater for duplicates/non integer).

An Adelson-Velskii Landis (AVL) tree is a self-balancing BST that maintains it's height to be O(log N) when having N vertices in the AVL tree.

BST (and especially balanced BST like AVL Tree) is an efficient data structure to implement a certain kind of Table (or Map) Abstract Data Type (ADT).

A Table ADT must support at least the following three operations as efficient as possible:
- Search(v) — determine if v exists in the ADT or not,
- Insert(v) — insert v into the ADT,
- Remove(v) — remove v from the ADT.

If we use unsorted array/vector to implement Table ADT, it can be inefficient:
- Search(v) runs in O(N), as we may end up exploring all N elements of the ADT if v actually does not exist,
- Insert(v) can be implemented in O(1), just put v at the back of the array,
- Remove(v) runs in O(N) too as we have to first search for v which is already O(N) and later close the resulting gap after deletion — also in O(N).

If we use sorted array/vector to implement Table ADT, we can improve the Search(v) performance but weakens the Insert(v) performance:
- Search(v) can now be implemented in O(log N), as we can now use binary search on the sorted array,
- Insert(v) now runs in O(N) as we need to implement an insertion-sort like strategy to make the array remains sorted,
- Remove(v) runs in O(N) because even if Search(v) runs in O(log N), we still need to close the gap after deletion — which is in O(N).

The goal is to impove basic Table ADT operations (Search, Insert, Remove, etc.) so they run in O(log N) time by exploring BST and balanced BST (AVL Tree) data structure.





