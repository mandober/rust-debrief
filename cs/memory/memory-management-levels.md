# Memory management levels

Memory Management concens tree levels:
- Hardware level
- OS level
- Application level


## Hardware level

At the hardware level, memory management is concerned with the electronic devices that actually store data. This includes all kinds of computer memory like SRAM, DRAM, SSD, Hard disk and other storage.


## OS level

In the early days of computing, programs would have direct, sole, unrestricted access to the entire physical memory and could write data anywhere just by specifying the _physical memory address_. Today, programs have illusion they have direct, sole, unrestricted access to memory, when, not only is the OS solely in charge of it, but the medium itself is not even real, being abstracted by the _virtual memory system_. 

The OS is in charge of all memory: it manages the virtual memory system, controlls the allocation of virtual memory to programs, mapping of virtual to physical memory, and reclaming memory from programs that fail to return it.

The OS manages the logical division of memory: the _stack_, the _heap_, _static_ blocks, etc.

Each process has its own virtual address space, which in 32-bit mode is always a 4GB block of memory addresses. These virtual addresses are mapped to physical memory by page tables, which are maintained by the OS (kerenl) and consulted by the CPU.


## Application level

The essential requirement of memory management is to provide ways to _dynamically allocate_ portions of memory to programs at their request, and release these resources when they are no longer needed, so they can be reused.

Knowing when an object needs to be created doesn't present a challange, but determining when an object is no longer needed, proved to be one of the fundamental issues in computing, so much so that different approaches to solving this would seperate, yet again, programing languages into two extremes: those that provide automatic memory management and those that require manual labor.

Dynamic memory management contrasts with automatic destruction of objects held in automatic variables, notably functions' local variables, which are destroyed at the end of their scope, when their containg stack frame is discarded.

Manual memory management requires the programmer to annotate objects to be deallocated, so that the memory they occupy is returned to the free store (this issue is only related to the free store i.e. the heap, the stack got off scot-free. this time). Nearly everything computer programmers do requires them to consider how to manage memory.

Garbage collection (GC) is automatic memory management aimed at identifying objects that cannot be accessed in the future, in order to destroy them and reclaim resources.

Majority of popular languages require garbage collection. Even when a language is designed with manual memory management in mind, it can have garbage collection implemented. Some languages allow both management methods at the same time, others allow users to choose between the available methods. Systems might also use a combination of approaches, or they could employ other techniques such as stack only allocation and region inference.
