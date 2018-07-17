# Memory



[The Periodic Table of Rust Types](http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/)

[Rust container cheat sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p)





## Managing Memory

Rust doesn't use garbage collector, prefering manual memory management assisted with RAII concept.

Manual memory management requires the programer to annotate the lifetimes.


The memory ocupied by a Rust program is split into two distinct areas: the heap and the sack. Simply put, the stack contains primitive variables, while the heap stores complex types; a heap can grow until the available memory is exhausted. The stack is faster, but may not grow without limits. Every binding in Rust is on the stack, but those bindings may refer to things in the heap, and elsewhere.



---

http://www.electronicdesign.com/blog/reflections-rust

In general, there is an owner of an object and references can be borrowed. An object cannot be released if there are borrowed references to it. All references have a lifetime, but they can often be inferred based on Rust language rules. There are also explicit lifetime parameters that can be used in various areas in the code, such as in function signatures.
