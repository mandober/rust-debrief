# cs.ds Glossary

## AVL tree
The first balanced BST. Has a balance requirement that each node's left and right subtrees have height differing by one.

## Balanced BST
BST with a balance requirement that guarantees $$O(log N)$$ depth of the tree; insertions and deletions must maintain the balance requirement.

## Binary heap
An implementation of a priority queue that uses only a simple array and supports insertion and deleteMin in logarithmic time.

## Binary search
An algorithm to search a sorted array in O(log N) time by searching in the middle and recursing.

## Binary search tree (BST)
An implementation of an ordered sequence collection that generally uses two links per node; without balancing, performance can be linear in the worst case but logarithmic on average.

## B-tree
A multiway search tree ofen used for database systems.

## Complete binary tree
A binary tree in which all levels are full, except possibly for the last level that is left-filled with no missing nodes. Used in the binary heap.

## Container
Stores a collection of items.

## Cuckoo hash table
A hash table implementation that uses two tables and two hash functions and supports constant worst-case access.

## Double-ended queue
A sequence container in which operations are restricted to the front and back.

## Fibonacci heap
An implementation of a priority queue that provides O(1) amortized cost for decreaseKey.

## Hash function
Maps items to array indices, with the property that if x = y, then hash(x) = hash(y) and if x≠y, then the collision probability that hash(x) = hash(y) should be approximately 1/M.

## Hash table
An implementation of an unordered sequence collection that typically provides constant time search and update on average.

## Heap order
In a priority queue that allows access to the minimum item, heap order means that every node's value is at least as large as its parent's value.

## Lefist heap
Te frst efcient mergeable priority queue.

## Linear probing
A hash table implementation that uses only a simple table and tries array slots sequentially starting from the hash value position until an empty slot is found.

## Linked list
A sequence container in which the items are linked; can be singly linked, storing a link to the next item, or doubly linked, storing links to both the previous and next items.

## Map
Stores key value pairs.

## Pairing heap
An implementation of a priority queue that provides o(log N) amortized cost for decreaseKey,uses two links per node, does not require balance information, and performs very well in practice.

## Path compression
In the union/fnd data structure, the process of changing the parent of every node on a fnd path to the root.

## Perfect hash table
A hash table scheme that supports constant time access by using hash tables to resolve hash table collisions.

## Primary clustering
A phenomena in linear probing in which keys with diﬀerent hash values attempt to resolve to similar alternate locations, potentially resulting in poor performance.

## Priority queue
A container in which only the minimum can be accessed and removed.

## Quadratic probing
A hash table implementation that uses only a simple table and tries slots sequentially starting from the hash value position plus i2 (starting with i = 0), until an empty slot is found.

## Queue
A sequence container in which insertions are restricted to the back and access and removal is restricted to the front.

## Red–black tree
A balanced search tree currently used in both the C++ and Java library.

## Rotation
A process by which parent/child relations among a few nodes are changed, while retaining binary search tree order. Examples include single and double rotations for AVL trees and a zig–zig rotation for splay trees.

## Search tree
An implementation of an ordered sequence collection that generally uses either binary trees or multiway trees.

## Separate chaining hash table
A hash table scheme in which collisions are resolved by singly linked lists.

## Sequence container
Stores items in a linear order, with items inserted at specifed positions.

## Skip list
An ordered container that uses linked lists with multiple forward pointers per node. A concurrent version is implemented as part of the Java library.

## Splay tree
A binary search tree that maintains no balance information but that has O(log N) amortized cost per operation.

## Splaying
The process in splay trees by which a node is rotated toward the root using zig, zig–zig, or zig–zag rotations.

## Stack
A sequence container in which operations are restricted to one end.

## Union/fnd data structure
Maintains a partition of {1, 2, …, N} under a sequence of union and find operations at only slightly more than constant cost per fnd.

## Universal hash function
A collection of hash functions such that for any specifc x ≠ y, only O(1/M) of the hash functions in the collection yield hash(x) = hash(y).
