# Collections

- Data structures
- module `std::collections` (re-export from `alloc` crate by `std`):
- Sequences:
  - `VecDeque<T>`, double-ended queue implemented with a growable ring buffer.
  - `LinkedList<T>`
- Maps: `HashMap<K, V, S = RandomState>`, `BTreeMap<K, V>`
- Sets: `HashSet<K, S = RandomState>`, `BTreeSet<K>`
- Misc: `BinaryHeap<T>`


## Modules
- `hash_map` impl with linear probing and Robin Hood bucket stealing.
- `hash_set` implemented as a `HashMap` where the value is `()`.
- `btree_map` map based on a B-Tree.
- `btree_set` set based on a B-Tree.
- `linked_list` doubly-linked list with owned nodes.
- `vec_deque` double-ended queue implemented with a growable ring buffer.
- `binary_heap` priority queue impl with a binary heap.
- `range` [LAB] Range syntax.

## Structs
- `HashMap` impl with linear probing and Robin Hood bucket stealing.
- `HashSet` impl as a `HashMap` where the value is `().`
- `BTreeMap` map based on a B-Tree.
- `BTreeSet` set based on a B-Tree.
- `LinkedList` doubly-linked list with owned nodes.
- `VecDeque` double-ended queue implemented with a growable ring buffer.
- `BinaryHeap` priority queue implemented with a binary heap.

## Enums
- `Bound` endpoint of a range of keys.




---

## Module `std::collections::binary_heap`

**Structs**
BinaryHeap	A priority queue implemented with a binary heap.
Drain	A draining iterator over the elements of a BinaryHeap.
IntoIter	An owning iterator over the elements of a BinaryHeap.
Iter	An iterator over the elements of a BinaryHeap.
PeekMut	Structure wrapping a mutable reference to the greatest item on a BinaryHeap.
BinaryHeapPlace	[Experimental]


## Module `std::collections::btree_map`

A map based on a B-Tree.

**Structs**
BTreeMap	A map based on a B-Tree.
IntoIter	An owning iterator over the entries of a BTreeMap.
Iter	An iterator over the entries of a BTreeMap.
IterMut	A mutable iterator over the entries of a BTreeMap.
Keys	An iterator over the keys of a BTreeMap.
OccupiedEntry	A view into an occupied entry in a BTreeMap. It is part of the Entry enum.
Range	An iterator over a sub-range of entries in a BTreeMap.
RangeMut	A mutable iterator over a sub-range of entries in a BTreeMap.
VacantEntry	A view into a vacant entry in a BTreeMap. It is part of the Entry enum.
Values	An iterator over the values of a BTreeMap.
ValuesMut	A mutable iterator over the values of a BTreeMap.

**Enums**
Entry	A view into a single entry in a map, which may either be vacant or occupied.


## Module `std::collections::btree_set`

A set based on a B-Tree.

**Structs**:
BTreeSet	A set based on a B-Tree.
Difference	A lazy iterator producing elements in the difference of BTreeSets.
Intersection	A lazy iterator producing elements in the intersection of BTreeSets.
IntoIter	An owning iterator over the items of a BTreeSet.
Iter	An iterator over the items of a BTreeSet.
Range	An iterator over a sub-range of items in a BTreeSet.
SymmetricDifference	A lazy iterator producing elements in the symmetric difference of BTreeSets.
Union	A lazy iterator producing elements in the union of BTreeSets.


## Module `std::collections::hash_map`

A hash map implemented with linear probing and Robin Hood bucket stealing.

**Structs**
DefaultHasher	The default Hasher used by RandomState.
Drain	A draining iterator over the entries of a HashMap.
HashMap	A hash map implemented with linear probing and Robin Hood bucket stealing.
IntoIter	An owning iterator over the entries of a HashMap.
Iter	An iterator over the entries of a HashMap.
IterMut	A mutable iterator over the entries of a HashMap.
Keys	An iterator over the keys of a HashMap.
OccupiedEntry	A view into an occupied entry in a HashMap. It is part of the Entry enum.
RandomState	RandomState is the default state for HashMap types.
VacantEntry	A view into a vacant entry in a HashMap. It is part of the Entry enum.
Values	An iterator over the values of a HashMap.
ValuesMut	A mutable iterator over the values of a HashMap.
EntryPlace	[Experimental] A place for insertion to a Entry.

**Enums**
Entry	A view into a single entry in a map, which may either be vacant or occupied.


## Module `std::collections::hash_set`

A hash set implemented as a HashMap where the value is ().

**Structs**
Difference	A lazy iterator producing elements in the difference of HashSets.
Drain	A draining iterator over the items of a HashSet.
HashSet	A hash set implemented as a HashMap where the value is ().
Intersection	A lazy iterator producing elements in the intersection of HashSets.
IntoIter	An owning iterator over the items of a HashSet.
Iter	An iterator over the items of a HashSet.
SymmetricDifference	A lazy iterator producing elements in the symmetric difference of HashSets.
Union	A lazy iterator producing elements in the union of HashSets.


## Module `std::collections::linked_list`

A doubly-linked list with owned nodes.

The LinkedList allows pushing and popping elements at either end in constant time.

Almost always it is better to use Vec or VecDeque instead of LinkedList. In general, array-based containers are faster, more memory efficient and make better use of CPU cache.

**Structs**
IntoIter	An owning iterator over the elements of a LinkedList.
Iter	An iterator over the elements of a LinkedList.
IterMut	A mutable iterator over the elements of a LinkedList.
LinkedList	A doubly-linked list with owned nodes.
BackPlace	[Experimental] A place for insertion at the back of a LinkedList.
DrainFilter	[Experimental] An iterator produced by calling drain_filter on LinkedList.
FrontPlace	[Experimental] A place for insertion at the front of a LinkedList.


## Module `std::collections::range`

This is a nightly-only experimental API. (collections_range [#30877](https://github.com/rust-lang/rust/issues/30877))

Range syntax.

**Traits**
`RangeArgument` [LAB] RangeArgument is implby Rust's built-in range types, produced by range syntax like `..`, `a..`, `..b` or `c..d`.



## Module `std::collections::vec_deque`

**Structs**
Drain	A draining iterator over the elements of a VecDeque.
IntoIter	An owning iterator over the elements of a VecDeque.
Iter	An iterator over the elements of a VecDeque.
IterMut	A mutable iterator over the elements of a VecDeque.
VecDeque	A double-ended queue implemented with a growable ring buffer.
PlaceBack	[Experimental] A place for insertion at the back of a VecDeque.
PlaceFront	[Experimental] A place for insertion at the front of a VecDeque.

