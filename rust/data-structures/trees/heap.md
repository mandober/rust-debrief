# Heap
a heap is a specialized tree-based data structure that satisfies the heap property: if P is a parent node of C, then the key (the value) of P is either greater than or equal to (in a max heap) or less than or equal to (in a min heap) the key of C. The node at the "top" of the heap (with no parents) is called the root node.

The heap is one maximally efficient implementation of an abstract data type called a priority queue, and in fact priority queues are often referred to as "heaps", regardless of how they may be implemented. A common implementation of a heap is the binary heap, in which the tree is a binary tree. 

Heaps are usually implemented in an array (fixed size or dynamic array), and do not require pointers between elements. After an element is inserted into or deleted from a heap, the heap property may be violated and the heap must be balanced by internal operations.

Binary heaps may be represented in a very space-efficient way (as an implicit data structure) using an array alone. The first (or last) element will contain the root. The next two elements of the array contain its children. The next four contain the four children of the two child nodes, etc. Thus the children of the node at position n would be at positions 2n+1 and 2n+2 in the array. This allows moving up or down the tree by doing simple index computations. Balancing a heap is done by shift-up or shift-down operations (swapping elements which are out of order). As we can build a heap from an array without requiring extra memory (for the nodes, for example), heapsort can be used to sort an array in-place.

## Variants: 
- https://www.wikiwand.com/en/Heap_(data_structure)
- [2–3 heap](https://www.wikiwand.com/en/2%E2%80%933_heap)
- [B-heap](https://www.wikiwand.com/en/B-heap)
- [Beap](https://www.wikiwand.com/en/Beap)
- [Binary heap](https://www.wikiwand.com/en/Binary_heap)
- [Binomial heap](https://www.wikiwand.com/en/Binomial_heap)
- [Brodal queue](https://www.wikiwand.com/en/Brodal_queue)
- [D-ary heap](https://www.wikiwand.com/en/D-ary_heap)
- [Fibonacci heap](https://www.wikiwand.com/en/Fibonacci_heap)
- [Leftist tree](https://www.wikiwand.com/en/Leftist_tree)
- [Pairing heap](https://www.wikiwand.com/en/Pairing_heap)
- [Radix heap](https://www.wikiwand.com/en/Radix_heap)
- [Randomized meldable heap](https://www.wikiwand.com/en/Randomized_meldable_heap)
- [Skew heap](https://www.wikiwand.com/en/Skew_heap)
- [Soft heap](https://www.wikiwand.com/en/Soft_heap)
- [Ternary heap](https://www.wikiwand.com/en/Ternary_heap)
- [Treap](https://www.wikiwand.com/en/Treap)
- [Weak heap](https://www.wikiwand.com/en/Weak_heap)

## Applications
The heap data structure has many applications.
* [Heapsort][1]: One of the best sorting methods being in-place and with no quadratic worst-case scenarios.
* [Selection algorithms][2]: A heap allows access to the min or max element in constant time, and other selections (such as median or kth-element) can be done in sub-linear time on data that is in a heap.[[16]][3]
* [Graph algorithms][4]: By using heaps as internal traversal data structures, run time will be reduced by polynomial order. Examples of such problems are [Prim's minimal-spanning-tree algorithm][5] and [Dijkstra's shortest-path algorithm][6].
* [Priority Queue][7]: A priority queue is an abstract concept like "a list" or "a map"; just as a list can be implemented with a linked list or an array, a priority queue can be implemented with a heap or a variety of other methods.
* [Order statistics][8]: The Heap data structure can be used to efficiently find the kth smallest (or largest) element in an array.

[1]: https://www.wikiwand.com/en/Heapsort "Heapsort"
[2]: https://www.wikiwand.com/en/Selection_algorithm ""
[3]: https://www.wikiwand.com/en/#citenote21
[4]: https://www.wikiwand.com/en/List_of_algorithms#Graph_algorithms "List of algorithms"
[5]: https://www.wikiwand.com/en/Prim%27s_algorithm "Prim's algorithm"
[6]: https://www.wikiwand.com/en/Dijkstra%27s_algorithm ""
[7]: https://www.wikiwand.com/en/Priority_Queue "Priority Queue"
[8]: https://www.wikiwand.com/en/Order_statistics "Order statistics"



## Implementations
* The [C++ Standard Library][1] provides the `make_heap`, `push_heap` and `pop_heap` algorithms for heaps (usually implemented as binary heaps), which operate on arbitrary random access [iterators][2]. It treats the iterators as a reference to an array, and uses the array-to-heap conversion. It also provides the container adaptor `priority_queue`, which wraps these facilities in a container-like class. However, there is no standard support for the decrease/increase-key operation.
* The [Boost C++ libraries][3] include a heaps library. Unlike the STL, it supports decrease and increase operations, and supports additional types of heap: specifically, it supports _d_-ary, binomial, Fibonacci, pairing and skew heaps.
* There is a [generic heap implementation][4] for [C][5] and [C++][6] with [D-ary heap][7] and [B-heap][8] support. It provides an STL-like API.
* The standard library of the [D programming language][9] includes [`std.container.BinaryHeap`][10], which is implemented in terms of D's [ranges][11]. Instances can be constructed from any [random-access range][12]. `BinaryHeap` exposes an [input range interface][13] that allows iteration with D's built-in `foreach` statements and integration with the range-based API of the [`std.algorithm` package][14].
* The [Java][15] platform (since version 1.5) provides a binary heap implementation with the class [`java.util.PriorityQueue][16]` in the [Java Collections Framework][17]. This class implements by default a min-heap; to implement a max-heap, programmer should write a custom comparator. There is no support for the decrease/increase-key operation.
* [Python][18] has a [`heapq`][19] module that implements a priority queue using a binary heap.
* [PHP][20] has both max-heap (`SplMaxHeap`) and min-heap (`SplMinHeap`) as of version 5.3 in the Standard PHP Library.
* [Perl][21] has implementations of binary, binomial, and Fibonacci heaps in the [`Heap`][22] distribution available on [CPAN][23].
* The [Go][24] language contains a [`heap`][25] package with heap algorithms that operate on an arbitrary type that satisfies a given interface.
* Apple's [Core Foundation][26] library contains a [`CFBinaryHeap`][27] structure.
* [Pharo][28] has an implementation of a heap in the Collections-Sequenceable package along with a set of test cases. A heap is used in the implementation of the timer event loop.
* The [Rust][29] programming language has a binary max-heap implementation, [`BinaryHeap`][30], in the `collections` module of its standard library.

[1]: https://www.wikiwand.com/en/C%2B%2B_Standard_Library "C++ Standard Library"
[2]: https://www.wikiwand.com/en/Iterator ""
[3]: https://www.wikiwand.com/en/Boost_(C%2B%2B_libraries) ""
[4]: https://github.com/valyala/gheap
[5]: https://www.wikiwand.com/en/C_(programming_language) "C (programming language)"
[6]: https://www.wikiwand.com/en/C%2B%2B "C++"
[7]: https://www.wikiwand.com/en/D-ary_heap "D-ary heap"
[8]: https://www.wikiwand.com/en/B-heap "B-heap"
[9]: https://www.wikiwand.com/en/D_(programming_language) "D (programming language)"
[10]: https://dlang.org/phobos/std_container_binaryheap.html
[11]: https://tour.dlang.org/tour/en/basics/ranges
[12]: https://dlang.org/phobos/std_range_primitives.html#isRandomAccessRange
[13]: https://dlang.org/phobos/std_range_primitives.html#isInputRange
[14]: https://dlang.org/phobos/std_algorithm.html
[15]: https://www.wikiwand.com/en/Java_(programming_language) "Java (programming language)"
[16]: https://www.wikiwand.com//docs.oracle.com/javase/9/docs/api/java/util/PriorityQueue.html
[17]: https://www.wikiwand.com/en/Java_Collections_Framework "Java Collections Framework"
[18]: https://www.wikiwand.com/en/Python_(programming_language) "Python (programming language)"
[19]: https://docs.python.org/library/heapq.html
[20]: https://www.wikiwand.com/en/PHP "PHP"
[21]: https://www.wikiwand.com/en/Perl "Perl"
[22]: https://metacpan.org/module/Heap
[23]: https://www.wikiwand.com/en/CPAN "CPAN"
[24]: https://www.wikiwand.com/en/Go_(programming_language) "Go (programming language)"
[25]: http://golang.org/pkg/container/heap/
[26]: https://www.wikiwand.com/en/Core_Foundation "Core Foundation"
[27]: https://developer.apple.com/library/mac/#documentation/CoreFoundation/Reference/CFBinaryHeapRef/Reference/reference.html
[28]: https://www.wikiwand.com/en/Pharo "Pharo"
[29]: https://www.wikiwand.com/en/Rust_(programming_language) "Rust (programming language)"
[30]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

