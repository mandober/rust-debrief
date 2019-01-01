
# algorithm

## online algorithm
In computer science, an online algorithm is one that can process its input piece-by-piece in a serial fashion, i.e., in the order that the input is fed to the algorithm, without having the entire input available from the start. In contrast, an offline algorithm is given the whole problem data from the beginning and is required to output an answer which solves the problem at hand.


# Data Structures

# std
- struct
- enum
- union
- Tuple
- Array
- Vector
- VectDeque
- HashMap
- HashSet
- DLL
- BTreeMap
- BTreeSet
- BinaryHeap

# ADT
- Stack
- Queue
- Deque
- Priority queue
- Set
- Map

# Hash
- Dictionary
- Associated array

# Contiguous sequences
- Array
- Judy arrays
- Vector


# Lists
- singly-linked list
- doubly-linked list
- doubly-linked XOR list
- circular list
- unrolled list
- skip lists


# Trees
- binary tree
- black and white tree
- AVL tree: https://www.wikiwand.com/en/AVL_tree
- vEB tree (Van Emde Boas tree)
- Red-black tree: https://www.wikiwand.com/en/Red-black_tree
- radix trees
- trie


# Graphs
- Graph




## Van Emde Boas tree
https://www.wikiwand.com/en/Van_Emde_Boas_tree

A `vEB tree` (a *Van Emde Boas* tree or *Van Emde Boas priority queue*) is a tree data structure which implements an *associative array* with *m-bit integer keys*. It performs all operations in *O(log m)* time i.e. *O(log log M)* time, where `M = 2^m` is the maximum number of elements that can be stored in the tree. The `M` is not to be confused with the actual number of elements stored in the tree, by which the performance of other tree data-structures is often measured. The vEB tree has good space efficiency when it contains a large number of elements. It was invented by a team led by Dutch computer scientist Peter van Emde Boas in 1975.

Space: O(M)
Search: O(log log M)
Insert: O(log log M)
Delete: O(log log M)


## Bidirectional map (hash bag)
a bidirectional map, or hash bag, is an associative data structure in which the (key,value) pairs form a one-to-one correspondence. Thus the binary relation is functional in each direction: value can also act as a key to key. A pair (a,b) thus provides a unique coupling between a and b so that b can be found when a is used as a key and a can be found when b is used as a key.


## Multimap
https://www.wikiwand.com/en/Multimap

a multimap (sometimes also multihash or multidict) is a generalization of a map or associative array abstract data type in which more than one value may be associated with and returned for a given key. Both map and multimap are particular cases of containers (for example, see C++ Standard Template Library containers). Often the multimap is implemented as a map with lists or sets as the map values. 

In a student enrollment system, where students may be enrolled in multiple classes simultaneously, there might be an association for each enrollment of a student in a course, where the key is the student ID and the value is the course ID. If a student is enrolled in three courses, there will be three associations containing the same key. The index of a book may report any number of references for a given index term, and thus may be coded as a multimap from index terms to any number of reference locations or pages. Querystrings may have multiple values associated with a single field. This is commonly generated when a web form allows multiple check boxes or selections to be chosen in response to a single form element.


## Array
https://www.wikiwand.com/en/Array_data_structure

an array data structure, or simply an array, is a data structure consisting of a collection of elements (values or variables), each identified by at least one array index or key. An array is stored so that the position of each element can be computed from its index tuple by a mathematical formula. The simplest type of data structure is a linear array, also called one-dimensional array.


## Tree
https://www.wikiwand.com/en/Tree_(data_structure)

a tree is a widely used abstract data type (ADT), or data structure implementing this ADT, that simulates a hierarchical tree structure, with a root value and subtrees of children with a parent node, represented as a set of linked nodes.

A tree data structure can be defined recursively (locally) as a collection of nodes (starting at a root node), where each node is a data structure consisting of a value, together with a list of references to nodes (the "children"), with the constraints that no reference is duplicated, and none points to the root.

Alternatively, a tree can be defined abstractly as a whole (globally) as an ordered tree, with a value assigned to each node. Both these perspectives are useful: while a tree can be analyzed mathematically as a whole, when actually represented as a data structure it is usually represented and worked with separately by node (rather than as a set of nodes and an adjacency list of edges between nodes, as one may represent a digraph, for instance). For example, looking at a tree as a whole, one can talk about "the parent node" of a given node, but in general as a data structure a given node only contains the list of its children, but does not contain a reference to its parent (if any).


## hash table
https://www.wikiwand.com/en/Hash_table
https://www.wikiwand.com/en/Hash_function

A hash table (hash map) is a data structure which implements an associative array abstract data type, a structure that can map keys to values. A hash table uses a hash function to compute an index into an array of buckets or slots, from which the desired value can be found.

Ideally, the hash function will assign each key to a unique bucket, but most hash table designs employ an imperfect hash function, which might cause hash collisions where the hash function generates the same index for more than one key. Such collisions must be accommodated in some way.

In a well-dimensioned hash table, the average cost (number of instructions) for each lookup is independent of the number of elements stored in the table. Many hash table designs also allow arbitrary insertions and deletions of key-value pairs, at (amortized) constant average cost per operation.

In many situations, hash tables turn out to be on average more efficient than search trees or any other table lookup structure. For this reason, they are widely used in many kinds of computer software, particularly for associative arrays, database indexing, caches, and sets.



## amortized analysis
https://www.wikiwand.com/en/Amortized_analysis
amortized analysis is a method for analyzing a given algorithm's complexity, or how much of a resource, especially time or memory, it takes to execute. The motivation for amortized analysis is that looking at the worst-case run time per operation can be too pessimistic.

While certain operations for a given algorithm may have a significant cost in resources, other operations may not be as costly. Amortized analysis considers both the costly and less costly operations together over the whole series of operations of the algorithm. This may include accounting for different types of input, length of the input, and other factors that affect its performance.



## Judy array
https://www.wikiwand.com/en/Judy_array

A Judy array is a data structure implementing a type of associative array with high performance and low memory usage. Unlike most other key-value stores, Judy arrays use no hashing, leverage compression on their keys (which may be integers or strings), and can efficiently represent sparse data, that is, they may have large ranges of unassigned indices without greatly increasing memory usage or processing time. They are designed to remain efficient even on structures with sizes in the peta-element range, with performance scaling on the order of `O(log256n)`. Roughly speaking, Judy arrays are highly optimized 256-ary radix trees.

Judy trees are usually faster than AVL trees, B-trees, hash tables and skip lists because they are highly optimized to maximize usage of the CPU cache. In addition, they require no tree balancing and no hashing algorithm is used.

The Judy array was invented by Douglas Baskins and named after his sister.

Memory allocation: Judy arrays are dynamic and can grow or shrink as elements are added to, or removed from, the array. The memory used by Judy arrays is nearly proportional to the number of elements in the Judy array.
Speed: Judy arrays are designed to minimize the number of expensive cache-line fills from RAM, and so the algorithm contains much complex logic to avoid cache misses as often as possible. Due to these cache optimizations, Judy arrays are fast, especially for very large datasets. On data sets that are sequential or nearly sequential, Judy arrays can even outperform hash tables, since, unlike hash tables, the internal tree structure of Judy arrays maintains the ordering of the keys.
Drawbacks: Judy arrays are extremely complicated. The smallest implementations are thousands of lines of code.

- [Main Judy arrays site](http://judy.sourceforge.net/)
- [How Judy arrays work and why they are so fast](http://judy.sourceforge.net/downloads/10minutes.htm)
- [A complete technical description of Judy arrays](http://judy.sourceforge.net/application/shop_interm.pdf)
- [An independent performance comparison of Judy to H](http://www.nothings.org/computer/judy/)
- [A compact implementation of Judy arrays in 1250 li](http://code.google.com/p/judyarray)


## Binary Heap
https://www.wikiwand.com/en/Binary_heap

A binary heap is a heap data structure that takes the form of a binary tree. Binary heaps are a common way of implementing priority queues. The binary heap was introduced by J. W. J. Williams in 1964, as a data structure for heapsort.

A binary heap is defined as a binary tree with two additional constraints:
1. Shape property: a binary heap is a complete binary tree; that is, all levels of the tree, except possibly the last one (deepest) are fully filled, and, if the last level of the tree is not complete, the nodes of that level are filled from left to right.
2. Heap property: the key stored in each node is either greater than or equal to (≥) or less than or equal to (≤) the keys in the node's children, according to some total order.
