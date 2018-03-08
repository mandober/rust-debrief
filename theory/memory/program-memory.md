# Program memory

- `.text`: ro code segment with executable instructions
- `.data`: globals and statics, rw, retain their address across calls
- `.bss`: uninitialized data, globals and statics initialized to zero
- the heap: shared pool of free memory
- the stack: program's own stack


Program memory can be largely categorized into two sections: read-only (ro) and read-write (rw). This distinction grew from early computer systems, but the idea that some portions of the program's memory should not be modified was retained.

**Text**: The code segment (text, text segment) is where a portion of an object file, or the corresponding section of the program's virtual address space that contains executable instructions, is stored and is generally read-only and fixed size.

**Data**: The `.data` segment contains any global or static variables which have a pre-defined value and can be modified: any variable not declared within a function, but defined as static, so it retains its address across calls. 

The values for these variables are initially stored within the read-only memory (typically within `.text`) and are copied into the `.data` segment during the start-up routine of the program. Example in C:

```c
int val = 3;
char string[] = "Hello World";
```


**BSS**: The BSS segment, also known as uninitialized data, is usually adjacent to the data segment. The `.bss` segment contains all global variables and static variables that are *initialized to zero* or do not have explicit initialization in source code. For instance, a variable defined as `static int i` would be contained in the BSS segment.


**Heap**: The heap area commonly begins at the end of the `.bss` and `.data` segments and grows to larger addresses from there. The heap area is managed by `malloc`, `calloc`, `realloc` and `free`, which may use the `brk` and `sbrk` system calls to adjust its size (note that the use of brk/sbrk and a single "heap area" is not required to fulfill the contract of malloc, calloc, realloc, free; they may also be implemented using mmap/munmap to reserve/unreserve potentially non-contiguous regions of virtual memory into the process' virtual address space). The heap area is shared by all threads, shared libraries, and dynamically loaded modules in a process.


**Stack**: The stack area contains the program stack, typically located in the higher parts of memory. A "stack pointer" register tracks the top of the stack; it is adjusted each time a value is "pushed" onto the stack. The set of values pushed for one function call is termed a "stack frame". A stack frame consists at minimum of a return address. Automatic variables are also stack allocated. 

Typical layout of a legacy program's memory:   
`^| text | data | bss | heap ->              <- stack |$`

The stack area is traditionally adjoined to the heap area and they grew towards each other; when the stack pointer meets the heap pointer, free memory is exhausted.

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
