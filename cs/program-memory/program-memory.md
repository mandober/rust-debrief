# Program memory

When a *program* is loaded from a storage medium into memory, it becomes a *process*. If configured to allow multi-instancing, the same program can be strated several times, yielding multiple instances of the same program, with each contained in its own separate and independent process.

A process is comprised of *threads*. At least one, main, thread, is created per process, and a process can create additional auxillary threads.

Each process "thinks" it's working with the entire and physical memory, and that it is the only process currently running in the system. In fact, the OS manipulates processes and presents them with a false view of the world. In reality, the OS is juggling a multitude of proccesses and the memory it ever assigns to a process is always virtual.

Loading a program creates a particular layout in memory for that program, which consists of several distinct segments (the segments here are ranges of memory addresses):

- `.text`  read-only code segment with executable instructions
- `.data`  initialized globals and statics; retain their address across calls
- `.bss`   uninitialized variables (global and static) are initialized to zero
- `.stack` the call stack
- the heap is also available being the shared pool of memory (free store)

When run:
- a process runs isolated from the others by means of virtual memory
- every process has its own segments, its own stack and heap.
- each threds has its own stack.
The heap area is shared by all threads, shared libraries, and dynamically loaded modules in a process.


categorized into two sections: read-only and read-write

`.text`: **code segment** (text segment) is a read-only segment with a fixed size  containing program's executable instructions.

`.data`: **data segment** contains any global or static variables which have a pre-defined value; they have a stable address and can be modified. The values of these variables are initially typically stored in .text, but during the program's start-up routine they are copied into .data segment.

`.bss`: **BSS segment** is for uninitialized data: all global and static variables that are initialized to zero or that don't have explicit initialization are stored here.

The **heap** area commonly begins after the `.bss` segment and grows toward higher addresses.

The heap area is managed by allocator functions, which usually have the similar name as the ones in C: `malloc`, `calloc`, `realloc`, `free`.

The overall size of heap can be adjusted with `brk` and `sbrk` system calls.

(use of brk/sbrk and a single heap area is not required to fulfill the contract of malloc, calloc, realloc, free)
they may also be implemented using mmap/munmap to reserve/unreserve potentially non-contiguous regions of virtual memory into the process' virtual address space).

The heap area is shared by all threads, shared libraries, and dynamically loaded modules in a process.



`.stack`: **Stack** area contains the program stack, typically located in the higher parts of memory.

The specal register (rsp @x64, esp @x86) called the stack pointer tracks the top of the stack; it is adjusted each time a value is pushed onto the stack.

The set of values pushed for one function call is termed a stack frame.
A stack frame consists at minimum of a return address.
Automatic variables are also stack allocated.

With large address spaces and virtual memory techniques, they tend to be placed more freely, but they still typically grow in a converging direction. On the standard x86 architecture, the stack grows toward address zero, meaning that more recent items, deeper in the call chain, are at numerically lower addresses and closer to the heap.


## Data segment

Historically, to be able to support memory address spaces larger than the native size of their internal address register, CPUs used segmentation by storing the offsets to certain memory areas.

The Intel 8086 family provided 4 segments:
- the code segment, `.text`, executable instructions
- the data segment, `.data`
- the stack segment
- the extra segment

The name `.bss` is used by many compilers and linkers for a part of the `.data` segment containing statically-allocated variables that are not explicitly initialized to any value. Typically only the length of the bss section, but no data, is stored in the object file.

Each segment was placed at a specific location in memory by the software being executed and all instructions that operated on the data within those segments were performed relative to the start of that segment.

A **static variable** is a variable that has been allocated "statically", meaning that its lifetime (extent) is the entire run of the program. This is in contrast to shorter-lived automatic variables, whose storage is allocated and deallocated on the call stack; and in contrast to objects, whose storage is dynamically allocated and deallocated in heap memory.

Variable lifetime is contrasted with scope (where a variable can be used): "global" and "local" refer to scope, not lifetime, but scope often implies lifetime. In many languages, global variables are always static, but in some languages they are dynamic, while local variables are generally automatic, but may be static. 

- static memory allocation is the allocation of memory at compile time, before the associated program is executed, unlike
- dynamic memory allocation or automatic memory allocation where memory is allocated as required at run time.

In terms of scope and extent, static variables have extent the entire run of the program, but may have more limited scope. A basic distinction is between a static global variable, which has global scope and thus is in context throughout the program, and a static local variable, which has local scope. A static local variable is different from a local variable as a static local variable is initialized only once no matter how many times the function in which it resides is called and its value is retained and accessible through many calls to the function in which it is declared, e.g. to be used as a count variable.
