# Collections

General purpose programming data structures were provided by the `collections` crate until its deprecation in Rust version 1.20, when they became a part of the `alloc` crate, accessed as the standard library's module `std::collections`.

Rust's collections can be grouped into four major categories:
- Sequences: Vec `vec`, VecDeque `vec_deque`, LinkedList `linked_list`
- Maps: HashMap `btree_map`, BTreeMap `btree_map`
- Sets: HashSet, BTreeSet `btree_set`
- Misc: BinaryHeap `binary_heap`


https://doc.rust-lang.org/alloc/vec/
https://doc.rust-lang.org/src/alloc/vec.rs.html



Available data structures:
- `binary_heap` A priority queue implemented with a binary heap.
- `btree_map` A map based on a B-Tree.
- `btree_set` A set based on a B-Tree.
- `linked_list` A doubly-linked list with owned nodes.
- `vec_deque` A double-ended queue implemented with a growable ring buffer.
- `vec` a contiguous growable array type with heap-allocated contents, `Vec<T>`.
- `range` [LAB] Range syntax.


- `string` an UTF-8 encoded, growable string.
