# Manual Memory Management

[Manual memory management](https://www.wikiwand.com/en/Manual_memory_management)


Manual memory management refers to the usage of manual instructions by the programmer to identify and deallocate unused objects (called garbage).

Languages in widespread use today that require manual memory management are C++, C and Rust.

Object creation i.e. determining when an object needs to be created is unproblematic, even trivial, the fundamental issue is with object destruction – determining when an object is no longer needed, when the memory it occupies can be returned to the heap (free store).




In manual memory allocation, this is also specified manually by the programmer; via functions such as free() in C, or the delete operator in C++ – this contrasts with automatic destruction of objects held in automatic variables, notably (non-static) local variables of functions, which are destroyed at the end of their scope in C and C++.
