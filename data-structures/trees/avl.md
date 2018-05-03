# AVL tree

An Adelson-Velskii Landis (AVL) tree is a self-balancing BST that maintains it's height to be `O(log N)` when having `N` vertices in the AVL tree.

BST (and especially balanced BST like AVL Tree) is an efficient data structure to implement a certain kind of Table (or Map) Abstract Data Type (ADT).

A Table ADT supports at least 3 operations:
- Search(v)
- Insert(v)
- Remove(v)

If we use unsorted array to implement Table ADT, it can be inefficient:
- Search(v) runs in O(N), as we may end up exploring all N elements of the ADT if v actually does not exist,
- Insert(v) can be implemented in O(1), just put v at the back of the array,
- Remove(v) runs in O(N) too as we have to first search for v which is already O(N) and later close the resulting gap after deletion — also in O(N).

If we use sorted array to implement Table ADT, we can improve the Search(v) performance but weakens the Insert(v) performance:
- Search(v) can now be implemented in O(log N), as we can now use binary search on the sorted array,
- Insert(v) now runs in O(N) as we need to implement an insertion-sort like strategy to make the array remains sorted,
- Remove(v) runs in O(N) because even if Search(v) runs in O(log N), we still need to close the gap after deletion — which is in O(N).

The goal is to impove basic Table ADT operations (Search, Insert, Remove, etc.) so they run in O(log N) time by exploring BST and balanced BST (AVL Tree) data structure.

