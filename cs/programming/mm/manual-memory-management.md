# Manual Memory Management

Garbage collection is always run, it constanly consumes computing resources, in deciding which memory to free, even though the programmer may have already known this information.

Manual memory management refers to the usage of manual instructions by the programmer to identify unneeded objects and to deallocate them i.e. to release the memory they occupy so it can be reclaimed by the system.

Some languages, in widespread use today, that require manual memory management are C++, C and Rust.

In manual memory allocation, object creation and destruction is managed by the programmer, by using appropriate functions.

In C, these are the functions like `malloc`, `calloc`, `realloc` and `free`, which have became almost synonymous with manual memory management. They force the programmer to keep track of which memory is still required, and who is responsible for freeing it (the caller or the callee). While this may be manageble for smaller progrems, it can become a source of bugs in larger applications.


C++ has `new` and `delete` for dealing with MM, but it also provides smart pointers that ease the memory management.

The RAII (resource acquisition is initialization) idiom that has originated in C++ automaticcally desposes of dinamically aquired resources when they fall out of scope. The defined object destructors (finalizers) enable arbitrary code to run just before an object is destroyed, allowing for clean up and manual release of resources.

Another approach is to use static program analysis and automated theorem proving to ensure that the program is free of memory errors. Rust is such a language - it encorporates many techniques, like a borrow checker, to ensure and guarantee memory safety. In fact, Rust is the only mainstream language that can guarantee memory safety.

