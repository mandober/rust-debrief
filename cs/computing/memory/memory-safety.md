# Memory Safety

There are two common classes of memory safety:
1. Spatial memory safety    
   guarantees that one can not get out of bounds of the claimed memory blocks.
2. Temporal memory safety    
   ensures that it's impossible to dereference memory that has already been freed.


---

[Memory Safety in Rust: A Case Study with C | Will Crichton](http://willcrichton.net/notes/rust-memory-safety/)


In all programming that uses memory, we desire two program properties:

__Memory safety__ is the property of a program where memory pointers used always point to valid memory, i.e. allocated and of the correct type/size. Memory safety is a _correctness issue_ - a memory unsafe program may crash or produce nondeterministic output depending on the bug.

__Safe-for-space__ is the property of a program where memory does not leak, i.e. every allocated memory chunk can either be used (it is reachable from the root set of the program) or it will be deallocated eventually. A program may leak memory if it doesn't return memory it no longer needs (that is holding the objects that are no longer needed). This is a _performance issue_ because a leaky program may eventually deplate system of all memory.

In garbage-collected languages memory safety is guaranteed and maintained by the language's runtime.

Memory containment is guaranteed for _tracing_ garbage collectors (like Java), but not necessarily for _reference counting_ garbage collectors (like Python).

---

[Memory safety | Wikiwand](https://www.wikiwand.com/en/Memory_safety)


Memory safety is the state of being protected from various software bugs and security vulnerabilities when dealing with memory access, such as buffer overflows and dangling pointers.

For example, Java is said to be memory-safe because its runtime error detection checks array bounds and pointer dereferences. C and C++ allow arbitrary pointer arithmetic with pointers implemented as direct memory addresses with no provision for bounds checking, and thus are termed memory-unsafe. Rust is memory safe unless a programer explicitly opts into unsafe mode and takes the responsibility of guaranteeing memory safety onto herself (himself, irresponsibile asswipe, is unaccountable in such occasions).



## Types of memory errors
- Access errors: invalid R/W of a pointer
  - Buffer overflow: out-of-bound (OOB) writes can corrupt adjacent objects
  - Buffer over-read: OOB reads can reveal sensitive data or help beat ASLR
  - Race condition: concurrent R/W to shared memory
  - Invalid page fault: accessing a pointer outside the virtual memory space
  - Use after free: dereferencing a dangling pointer
- Uninitialized variables
  - Null pointer dereference: dereferencing an invalid pointer
  - Wild pointers: pointer is used prior to init to some known state.
- Memory leaks: when memory usage is tracked incorrectly
  - Stack exhaustion: when a program runs out of stack space (deep recursion)
  - Heap exhaustion: program tries to allocate more memory than available.
  - Double free: repeatedly calling free
  - Invalid free: passing an invalid address to free
  - Mismatched free: using a different allocator's free fn
  - Unwanted aliasing: the same memory location is allocated and modified for unrelated purposes
