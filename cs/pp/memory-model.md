# Memory Model


## Flat Memory Model

Flat memory model or linear memory model refers to a memory addressing paradigm in which "memory appears to the program as a single contiguous address space."[1] The CPU can directly (and linearly) address all of the available memory locations without having to resort to any sort of memory segmentation or paging schemes.

Memory management and address translation can still be implemented on top of a flat memory model in order to facilitate the operating system's functionality, resource protection, multitasking or to increase the memory capacity beyond the limits imposed by the processor's physical address space, but the key feature of a flat memory model is that the entire memory space is linear, sequential and contiguous from address zero to MaxBytes − 1.

---

[https://pcwalton.github.io/2013/05/20/safe-manual-memory-management.html](https://pcwalton.github.io/2013/05/20/safe-manual-memory-management.html)

Safe Manual Memory Management
May 20, 2013

There are a few different models in common use in industry languages:


## Unsafe manual memory management
These languages provide very fine-grained control over memory allocation; heap memory can be explicitly allocated and deallocated. The most important examples here are C and C++. The well-known downside of this approach is that memory safety violations can only be detected at runtime with a memory safety checker such as Valgrind or Address Sanitizer. Memory safety violations that go unchecked can lead to crashes at best and exploitable security vulnerabilities at worst.


## Full garbage collection
The majority of modern languages expose a memory model that falls into this category—the space is very diverse, ranging from Java to Go to JavaScript to Ruby to Haskell. In general, such languages place all allocations into the heap instead of the stack, although escape analysis and value types may be used to reduce the number of heap allocations. Periodically, a garbage collector scans all pointers on the stack and in the heap, judges unreachable objects dead, and reclaims them. This approach has the advantage of memory safety at compile time—the language arranges for there to be no dangling pointers, wild pointers, and so forth. The downsides, however, are:

The garbage collector may run at an inconvenient time. This can be mitigated by explicit control over when the GC runs, although if the garbage collector must collect multiple threads’ heaps at the same time, this may be difficult to synchronize. This can also be mitigated by using manual memory pooling and free lists, although pooling has undesirable safety properties—much like unsafe manual memory management, there is no static guarantee that objects allocated from a pool are returned properly or that an object is not reachable when returned to the pool. Incremental and concurrent garbage collectors help here, but they are not free, as they typically require write and/or read barriers, reducing throughput.

When it runs, the garbage collector must mark all pointers to discover which ones are live, reducing throughput of the application. Essentially, the GC must discover at runtime what a C++ (say) programmer knows at compile time. Not much can typically be done about this cost in fully garbage-collected languages, short of falling back to unsafe manual memory management. Pools don’t help much here, because the GC must still trace the pointers into the pool. Even pointers into the stack generally must be traced.

Garbage collection with value types and references—This category includes languages like C#. (I believe D falls into this category as well, although I may be mistaken.) These languages are essentially garbage-collected, but they include value types which are guaranteed to be stack-allocated if in local variables. Additionally, and most importantly, they include reference parameters (and sometimes reference locals), which allow stack-allocated values to be temporarily aliased when calling another function. Effective use of value types can reduce marking and sweeping time. In general, this system is an effective addition to a garbage-collected system, allowing a good measure of additional control without much cost in complexity and no cost in memory safety. It is not, however, typically sufficient to write programs without using the garbage collector at all; the system is too simple to statically encode anything other than the most basic memory management patterns.

## Where does Rust fit in? 

Actually, it fits into a category all to itself among industry languages (although one shared by various research languages, like Cyclone). Rust offers safe manual memory management (although some have objected to the term “manual” here). It extends the system described above as “garbage collection with value types and references” in two important ways:

You can allocate memory that will not be traced by the garbage collector, and free it manually if you choose. This is the feature of Rust known as “unique pointers”. Rust will automatically free memory that is uniquely owned when its owning pointer goes out of scope. It’s also easy to write a function that acts exactly as free does, so you can precisely choose when your objects die. Unique pointers are not traced by the GC (unless they point to a type that transitively contains a garbage-collected pointer), so they are an effective way to cut down marking times.

You can return references and place references into data structures. Like other references, these references are not traced by the garbage collector. As long as the references follow a stack discipline, meaning that they point to memory that was allocated by one of the callers of the current function, the compiler allows them to be placed anywhere. This adds a great deal of expressiveness over the reference parameter approach, and it enables a large number of programs to be written without using the garbage collector at all.

In terms of safety and performance, safe manual memory management is having your cake and eating it too. You get memory safety like a garbage-collected language, but control like that of unsafe manual memory management. But this system has its downsides as well—most importantly, complexity of implementation and interface. Learning to use references and unique pointers poses a significant learning curve. But, once the system is learned, it’s remarkably flexible, with an attractive combination of performance and safety.