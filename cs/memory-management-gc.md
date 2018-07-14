# Garbage Collection


Garbage Collection (GC) is a form of automatic memory management whose objective is finding data objects that cannot be accessed in the future, and to reclaim the memory used by those objects. Garbage collection was invented by John McCarthy in 1959 to simplify memory management in Lisp.

Majority of programming languages require GC. Garbage collector is a component in the runtime of the language that autonomously and periodically attempts to reclaim memory, after identifying exactly which memory blocks are not needed anymore. 

The advantage of GC is that it frees the programmers from manually dealing with memory, making it easier to concentrate on the actual problem, rather then the details of implementation. It potentially reduces numerous memory-related problems such as dangling pointers, double free errors, memory leaks, etc.

Disadvantage of GC
- non-deterministic occurance
- consuming additional resources
- performance impacts
- potential stalls in program execution
- runtime overhead
- incompatibility with manual resource management

The disadvantage of GC is that it frees the programmers from manually dealing with memory, making it easier for harder-to-find bugs to skip through. For example, Rust might be considered a hard language, because it makes it very hard for a programer to easily violate memory safety. It is also probably the only mainstream language that requires the lifetime of objects is known at compile-time; similarly to annotating types, this sometimes mean that the programer needs to manually annotate lifetimes of objects in the source code and to carefully think about their aliasing and mutation. And these things are always present, in any language, it's just that many languages, especially dynamic ones, don't expose these issues, at least not in so many details.

The penalty for the convenience of not annotating object lifetime manually is an overhead in the runtime that can lead to decreased or uneven performance. GC process can take a significant proportion of total processing time in a program and, as a result, can have significant influence on performance.

The moment when the garbage is actually collected can be unpredictable, resulting in stalls scattered throughout a session. Unpredictable stalls can be unacceptable in real-time environments, in transaction processing, or in interactive programs.

A peer-reviewed paper came to the conclusion that GC needs five times the memory to compensate for this overhead and to perform as fast as explicit memory management.

Non-deterministic GC is incompatible with resource acquisition is initialization (RAII) based management of non-GC'ed resources. As a result, the need for explicit manual resource management (release/close) for non-GCed resources becomes transitive to composition.

That is: in a non-deterministic GC system, if a resource or a resource-like object requires manual resource management (release/close), and this object is used as "part of" another object, then the composed object will also become a resource-like object that itself requires manual resource management (release/close).


## GC implementations
The modern GC implementations try to minimize blocking _stop-the-world_ stalls by doing as much work as possible on a separate thread.

For example, marking unreachable garbage instances while the application process continues to run. In spite of these advancements, it is still very difficult to maintain large heaps (millions of objects).

Incremental, concurrent, and real-time garbage collectors address many of the problems, with varying trade-offs. 


## Tracing GC
The term "garbage collection" usually refers to tracing garbage collection because it is the most common GC strategy.

It consists of determining which objects are garbage by tracing which objects are reachable by a chain of references (from root objects) and considering the rest as garbage to be collected.

However, a large number of algorithms that is used in implementing tracing, has widely varying complexity and performance characteristics.


## Reference counting
Reference counting garbage collection is where each object has a count of the number of references to it.

Garbage is identified by having a reference count of zero. An object's reference count is incremented when a reference to it is created, and decremented when a reference is destroyed. When the count reaches zero, the object's memory is reclaimed.

As with manual memory management, and unlike tracing garbage collection, reference counting guarantees that objects are destroyed as soon as their last reference is destroyed, and usually only accesses memory which is either in CPU caches, in objects to be freed, or directly pointed by those, and thus tends to not have significant negative side effects on CPU cache and virtual memory operation.

There are a number of disadvantages to reference counting; this can generally be solved or mitigated by more sophisticated algorithms:


## Cycles

If two or more objects refer to each other, they can create a cycle whereby neither will be collected as their mutual references never let their reference counts become zero. Some garbage collection systems using reference counting (like the one in CPython) use specific cycle-detecting algorithms to deal with this issue.[8] Another strategy is to use weak references for the "backpointers" which create cycles. Under reference counting, a weak reference is similar to a weak reference under a tracing garbage collector. It is a special reference object whose existence does not increment the reference count of the referent object. Furthermore, a weak reference is safe in that when the referent object becomes garbage, any weak reference to it lapses, rather than being permitted to remain dangling, meaning that it turns into a predictable value, such as a null reference.

Space overhead (reference count)

Reference counting requires space to be allocated for each object to store its reference count. The count may be stored adjacent to the object's memory or in a side table somewhere else, but in either case, every single reference-counted object requires additional storage for its reference count. Memory space with the size of an unsigned pointer is commonly used for this task, meaning that 32 or 64 bits of reference count storage must be allocated for each object. On some systems, it may be possible to mitigate this overhead by using a tagged pointer to store the reference count in unused areas of the object's memory. Often, an architecture does not actually allow programs to access the full range of memory addresses that could be stored in its native pointer size; certain number of high bits in the address is either ignored or required to be zero. If an object reliably has a pointer at a certain location, the reference count can be stored in the unused bits of the pointer. For example, each object in Objective-C has a pointer to its class at the beginning of its memory; on the ARM64 architecture using iOS 7, 19 unused bits of this class pointer are used to store the object's reference count.

Speed overhead (increment/decrement)

In naive implementations, each assignment of a reference and each reference falling out of scope often require modifications of one or more reference counters. However, in the common case, when a reference is copied from an outer scope variable into an inner scope variable, such that the lifetime of the inner variable is bounded by the lifetime of the outer one, the reference incrementing can be eliminated. The outer variable "owns" the reference. In the programming language C++, this technique is readily implemented and demonstrated with the use of const references. Reference counting in C++ is usually implemented using "smart pointers"[11] whose constructors, destructors and assignment operators manage the references. A smart pointer can be passed by reference to a function, which avoids the need to copy-construct a new smart pointer (which would increase the reference count on entry into the function and decrease it on exit). Instead the function receives a reference to the smart pointer which is produced inexpensively.

Requires atomicity

When used in a multithreaded environment, these modifications (increment and decrement) may need to be atomic operations such as compare-and-swap, at least for any objects which are shared, or potentially shared among multiple threads. Atomic operations are expensive on a multiprocessor, and even more expensive if they have to be emulated with software algorithms. It is possible to avoid this issue by adding per-thread or per-CPU reference counts and only accessing the global reference count when the local reference counts become or are no longer zero (or, alternatively, using a binary tree of reference counts, or even giving up deterministic destruction in exchange for not having a global reference count at all), but this adds significant memory overhead and thus tends to be only useful in special cases (it is used, for example, in the reference counting of Linux kernel modules).

Not real-time

Naive implementations of reference counting do not in general provide real-time behavior, because any pointer assignment can potentially cause a number of objects bounded only by total allocated memory size to be recursively freed while the thread is unable to perform other work. It is possible to avoid this issue by delegating the freeing of objects whose reference count dropped to zero to other threads, at the cost of extra overhead.

Escape analysis

Escape analysis can be used to convert heap allocations to stack allocations, thus reducing the amount of work needed to be done by the garbage collector. This is done using a compile-time analysis to determine whether an object allocated within a function is not accessible outside of it (i.e. escape) to other functions or threads. In such a case the object may be allocated directly on the thread stack and released when the function returns, reducing its potential garbage collection overhead.

Availability

Generally speaking, higher-level programming languages are more likely to have garbage collection as a standard feature. In some languages that do not have built in garbage collection, it can be added through a library, as with the Boehm garbage collector for C and C++.


Most functional programming languages, such as ML, Haskell, and APL, have garbage collection built in. Lisp is especially notable as both the first functional programming language and the first language to introduce garbage collection.

Other dynamic languages, such as Ruby and Julia (but not Perl 5 or PHP before version 5.3, which both use reference counting), JavaScript and ECMAScript also tend to use GC. Object-oriented programming languages such as Smalltalk and Java usually provide integrated garbage collection. Notable exceptions are C++ and Delphi which have destructors.



Compile-time use

Compile-time garbage collection is a form of static analysis allowing memory to be reused and reclaimed based on invariants known during compilation. This form of garbage collection has been studied in the Mercury programming language,[23] and it saw greater usage with the introduction of LLVM's automatic reference counter (ARC) into Apple's ecosystem (iOS and OS X) in 2011.

Real-time systems

Incremental, concurrent, and real-time garbage collectors have been developed, such as Baker's algorithm or Lieberman's algorithm.

In Baker's algorithm, the allocation is done in either half of a single region of memory. When it becomes half full, a garbage collection is performed which moves the live objects into the other half and the remaining objects are implicitly deallocated. The running program (the 'mutator') has to check that any object it references is in the correct half, and if not move it across, while a background task is finding all of the objects.

Generational garbage collection schemes are based on the empirical observation that most objects die young. In generational garbage collection two or more allocation regions (generations) are kept, which are kept separate based on object's age. New objects are created in the "young" generation that is regularly collected, and when a generation is full, the objects that are still referenced from older regions are copied into the next oldest generation. Occasionally a full scan is performed.

Some high-level language computer architectures include hardware support for real-time garbage collection.

Most implementations of real-time garbage collectors use tracing. Such real-time garbage collectors meet hard real-time constraints when used with a real-time operating system.