# Memory Safety

[Memory safety | Wikiwand](https://www.wikiwand.com/en/Memory_safety)

[Memory Safety in Rust: A Case Study with C | Will Crichton](http://willcrichton.net/notes/rust-memory-safety/)


In all programming that uses memory, we desire two program properties:

__Memory safety__ is the property of a program where memory pointers used always point to valid memory, i.e. allocated and of the correct type/size. Memory safety is a _correctness issue_ - a memory unsafe program may crash or produce nondeterministic output depending on the bug.

__Safe-for-space__ is the property of a program where memory does not leak, i.e. every allocated memory chunk can either be used (it is reachable from the root set of the program) or it will be deallocated eventually. A program may leak memory if it doesn't return memory it no longer needs (that is holding the objects that are no longer needed). This is a _performance issue_ because a leaky program may eventually deplate system of all memory.

In garbage-collected languages memory safety is guaranteed and maintained by the language's runtime.

Memory containment is guaranteed for _tracing_ garbage collectors (like Java), but not necessarily for _reference counting_ garbage collectors (like Python).

