# Allocating Memory

4 kinds of memory allocation:
- in processor registers
- static
- on the stack
- on the heap

In C and C++ language, static allocation is that of the global variables and of the variables declared using the static keyword; the stack allocation is the one used for all non-static local variables, and also for function arguments; while heap allocation is the one used by invoking the `malloc` function of the C language standard library, or the `new` operator of the C++ language.

Virtual-memory systems are modern multitasking and multithreading systems that offer offers a virtual address space to each of the running processes. The OS, running in a privileged (protected or kernel) mode, can access any part of memory; it manages the memory, assigning portions of memory to the running processes and scheduling their memory access.

Every process has a distinct, virtual address space, which the OS maps to the physical address space when needed. A process sees the (virtual) memory as a contiguous array of bytes entirely available to it.

When a process requests some memory, the OS immediately notifies the process (by returning a pointer to the allocated memory) that the memory is reserved (although it's not), so allocation request happens very fast. When the process tries to access that memory, the page fault will trigger the OS to intercept that call and actually map the memory (virtual to physical). In the subsequent attempts to access that same memory block, the process will access the mapped (real) memory. In case the memory address the process wanted to access is not in the address space of that process, an addressing error called segmentation fault will be triggered.

In the C language, there is `malloc`, `calloc` and `realloc` functions to allocate a buffer in the heap, and the `free` function to deallocate a previously allocated buffer. In the C++, there is `new` and the `new[]` operators to allocate respectively an object or an array of objects in
the heap, and the `delete` and `delete[]` operators to deallocate what has been allocate

