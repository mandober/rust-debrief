# Memory management

<!-- TOC -->

- [Memory address space](#memory-address-space)
- [Data segment](#data-segment)
- [Program memory](#program-memory)
- [Processor register](#processor-register)
- [The stack and the heap](#the-stack-and-the-heap)
- [calling convention](#calling-convention)
- [Call site processing](#call-site-processing)
- [Subroutine entry processing](#subroutine-entry-processing)
- [Return processing](#return-processing)
- [Unwinding](#unwinding)
- [garbage collection](#garbage-collection)

<!-- /TOC -->



https://www.wikiwand.com/en/Shared_memory
https://www.wikiwand.com/en/Memory_segmentation
https://www.wikiwand.com/en/Code_segment
https://www.wikiwand.com/en/Data_segment
https://www.wikiwand.com/en/Heap_overflow
https://www.wikiwand.com/en/.bss
https://www.wikiwand.com/en/Static_variable
https://www.wikiwand.com/en/Stack-based_memory_allocation
https://www.wikiwand.com/en/Memory-mapped_file
https://www.wikiwand.com/en/Library_(computing)#/Shared_libraries
https://www.wikiwand.com/en/Pointer_(computer_programming)




## Memory address space
A process usually has in its address space chunks of memory of two types:
- Machine code that includes:
  - program's own code (.text segment)
  - shared libraries
- Data code that includes:
  - initialized data (.data segment)
  - uninitialized, but allocated variables (.bss)
  - run-time stack
  - heap
  - shared memory and memory mapped files.

Some parts of address space may be not mapped at all.





## Processor register

Processor register is a quickly accessible location available to a computer's central processing unit (CPU). Registers usually consist of a small amount of fast storage. Computers load data from memory into registers where they can manipulate it. Manipulated data is then often stored back to main memory, either by the same instruction or by a subsequent one. Registers are normally at the top of the memory hierarchy, and provide the fastest way to access data. Allocating frequently used variables to registers can be critical to a program's performance; this register allocation is performed either by a compiler in the code generation phase.


## The stack and the heap
The stack and the heap are parts of memory that is available to your code to use at runtime, but they are structured in different ways.

The stack and the heap are abstractions that help you determine when to allocate and deallocate memory.

The stack is very fast, and is where memory is allocated in Rust by default.
But the allocation is local to a function call, and is limited in size.

The heap, on the other hand, is slower, and is explicitly allocated by your program. But it’s effectively unlimited in size, and is globally accessible.

This meaning of heap, which allocates arbitrary-sized blocks of memory in arbitrary order, is quite different from the heap data structure.

Rust stack allocates by default, which means that basic values go on the stack.

When a function gets called, some memory gets allocated for all of its local variables and some other information. This is called a *stack frame*. When the function exits, its stack frame gets deallocated. Allocation and deallocation happens automatically.

Stack allocation is very fast. Since we know all the local variables we have ahead of time, we can grab the memory all at once. And since we'll throw them all away at the same time as well, we can get rid of it very fast too. The downside is that we can't keep values around if we need them for longer than a single function. 

~ ~ ~

The stack is fast because of the way it accesses the data: it never has to search for a place to put new data or a place to get data from because that place is always the top. Another property that makes the stack fast is that all data on the stack must take up a known, fixed size.

For data with a size unknown to us at compile time or a size that might change, we can store data on the heap instead. 

The heap is less organized: when we put data on the heap, we ask for some amount of space. The operating system finds an empty spot somewhere in the heap that is big enough, marks it as being in use, and returns to us a pointer to that location. This process is called allocating on the heap, and sometimes we abbreviate the phrase as just allocating.

Pushing values onto the stack is not considered allocating. Because the pointer is a known, fixed size, we can store the pointer on the stack, but when we want the actual data, we have to follow the pointer.

Contemporary processors are faster if they jump around less in memory. 
A processor can do its job better if it works on data that’s close to other data rather than farther away. Allocating a large amount of space on the heap can also take time.

When our code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so we don’t run out of space are all problems that ownership addresses.



## calling convention
*calling convention* is an implementation-level (low-level) scheme for how functions receive parameters from their caller and how they return a result. Differences in various implementations include where parameters, return values, return addresses and scope links are placed (register, stack, mix of both, etc.), and how the tasks of preparing for a function call and restoring the environment afterward are divided between the caller and the callee.

## Call site processing
Usually the call stack manipulation needed at the site of a call to a subroutine is minimal (which is good since there can be many call sites for each subroutine to be called). The values for the actual arguments are evaluated at the call site, since they are specific to the particular call, and either pushed onto the stack or placed into registers, as determined by the used calling convention. The actual call instruction, such as *branch and link*, is then typically executed to transfer control to the code of the target subroutine.

## Subroutine entry processing
In the called subroutine, the first code executed is usually termed the subroutine prologue, since it does the necessary housekeeping before the code for the statements of the routine is begun.

The prologue will commonly save the return address left in a register by the call instruction by pushing the value onto the call stack. Similarly, the current stack pointer and/or frame pointer values may be pushed. Alternatively, some instruction set architectures automatically provide comparable functionality as part of the action of the call instruction itself, and in such an environment the prologue does not need to do this.

If frame pointers are being used, the prologue will typically set the new value of the frame pointer register from the stack pointer. Space on the stack for local variables can then be allocated by incrementally changing the stack pointer.

## Return processing
When a subroutine is ready to return, it executes an epilogue that undoes the steps of the prologue. This will typically restore saved register values (such as the frame pointer value) from the stack frame, pop the entire stack frame off the stack by changing the stack pointer value, and finally branch to the instruction at the return address. Under many calling conventions the items popped off the stack by the epilogue include the original argument values, in which case there usually are no further stack manipulations that need to be done by the caller. With some calling conventions, however, it is the caller's responsibility to remove the arguments from the stack after the return.

## Unwinding
Returning from the called function will pop the top frame off of the stack, perhaps leaving a return value. The more general act of popping one or more frames off the stack to resume execution elsewhere in the program is called stack unwinding and must be performed when non-local control structures are used, such as those used for exception handling. In this case, the stack frame of a function contains one or more entries specifying exception handlers. When an exception is thrown, the stack is unwound until a handler is found that is prepared to handle (catch) the type of the thrown exception.

Some languages have other control structures that require general unwinding. Pascal allows a global goto statement to transfer control out of a nested function and into a previously invoked outer function. This operation requires the stack to be unwound, removing as many stack frames as necessary to restore the proper context to transfer control to the target statement within the enclosing outer function. Similarly, C has the setjmp and longjmp functions that act as non-local gotos. Common Lisp allows control of what happens when the stack is unwound by using the unwind-protect special operator.

When applying a continuation, the stack is (logically) unwound and then rewound with the stack of the continuation. This is not the only way to implement continuations; for example, using multiple, explicit stacks, application of a continuation can simply activate its stack and wind a value to be passed. The Scheme programming language allows arbitrary thunks to be executed in specified points on "unwinding" or "rewinding" of the control stack when a continuation is invoked.



## garbage collection
garbage collection (GC) is a form of automatic memory management. The garbage collector, or just collector, attempts to reclaim garbage, or memory occupied by objects that are no longer in use by the program.
Garbage collection is often portrayed as the opposite of manual memory management, which requires the programmer to specify which objects to deallocate and return to the memory system. However, many systems use a combination of approaches, including other techniques such as stack allocation and region inference. Like other memory management techniques, garbage collection may take a significant proportion of total processing time in a program and, as a result, can have significant influence on performance. With good implementations, enough memory, and depending on application, garbage collection can be faster than manual memory management, while the opposite can also be true and has often been the case in the past with sub-optimal GC algorithms.

Garbage collection is a strategy for automatically detecting memory allocated to objects that are no longer usable in a program, and returning that allocated memory to a pool of free memory locations. This method is in contrast to "manual" memory management where a programmer explicitly codes memory requests and memory releases in the program. While automatic garbage has the advantages of reducing programmer workload and preventing certain kinds of memory allocation bugs, garbage collection does require memory resources of its own, and can compete with the application program for processor time.



https://www.wikiwand.com/en/Execution_(computing)

https://www.wikiwand.com/en/Call_stack

https://www.wikiwand.com/en/Manual_memory_management

https://www.wikiwand.com/en/C_dynamic_memory_allocation

https://www.wikiwand.com/en/Memory_management#/ALLOCATION

https://www.wikiwand.com/en/Memory_management

