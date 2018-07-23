# Memory



[The Periodic Table of Rust Types](http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/)

[Rust container cheat sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p)




`main` is the fn: when it returns, the program is also over; it only remains to return the error (status) code to the caller (shell). So the extent of main fn is the extent of the whole program; the extent of main fn is the extent of the program, practically the `'static` lifetime. All construction done strictly within main fn itself, which is using the stack will have a stable address. main fn is in the main thread, which, like any thread, has its own stack, but unlike user-created threads, the main's stack size cannot be specified (it's already running). Just before the main's stack base address is the address of the caller (shell) that ran the program. 


## Managing Memory

Rust doesn't use garbage collector, prefering manual memory management assisted with RAII concept.

Manual memory management requires the programer to annotate the lifetimes.

The memory ocupied by a Rust program is split into two distinct areas: the heap and the sack. Simply put, the stack contains primitive variables, while the heap stores complex types; a heap can grow until the available memory is exhausted. The stack is faster, but may not grow without limits. Every binding in Rust is on the stack, but those bindings may refer to things in the heap, and elsewhere.



In general, there is an owner of an object and references can be borrowed. An object cannot be released if there are borrowed references to it. All references have a lifetime, but they can often be inferred based on Rust language rules. There are also explicit lifetime parameters that can be used in various areas in the code, such as in function signatures.
