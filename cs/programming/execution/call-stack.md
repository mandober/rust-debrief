# Call stack

<!-- TOC -->

- [Properties](#properties)
- [Call stack](#call-stack)
- [Stack frames](#stack-frames)
- [Structure](#structure)
- [Security](#security)

<!-- /TOC -->

A stack is an abstract data type that serves as a collection of elements, with two principal operations: *push*, which adds an element to the collection, and *pop*, which removes the most recently added element. The order in which elements come off a stack gives rise to its alternative name, LIFO (last in, first out). The name "stack" comes from the analogy to a set of physical items stacked on top of each other, which makes it very easy to put or get an item off the top.


## Properties
Considered as a linear data structure, the push and pop operations occur only at one end of the structure, referred to as the top of the stack. This makes the stack extremely fast as there is only one place for data to be written or read.

This linearity means the needed data is laid out in sequence, so it is readily available.

Additionally, CPUs have special instructions and registers for efficient and effective stack manipulation.

Another property that makes the stack fast is that all data on the stack must take up a known, fixed size.


Another feature is that memory on the stack is automatically, and very efficiently, reclaimed when the function exits, which can be convenient for the programmer if the data is no longer required. If however, the data needs to be kept in some form, then it must be copied from the stack before the function exits. Therefore, stack based allocation is suitable for temporary data or data which is no longer required after the creating function exits.


## Call stack
The primary purpose of the stack is to keep track of the point to which each active function should return control when it finishes executing. This kind of stack is called the **call stack**, but it is also known as an execution stack, program stack, control stack, run-time stack or machine stack.

An **active function** is one that has been called, but is yet to complete execution after which control should be handed back to the point of call.

When a function is called, the address of the instruction at which the calling function can later resume needs to be saved somewhere:
- the caller pushes the return address onto the stack, and the called function, when it finishes, pops the return address off the call stack and transfers control to that address.
- if a called function calls yet another function, it will push another return address onto the call stack, and so on, with the information stacking up and unstacking as the program runs.

Adding a function's entry to the call stack is sometimes called **winding**. Conversely, removing entries is **unwinding** the stack.

There is usually one call stack per each thread of a process.

Using a stack to save the return address has important advantages over alternative **calling conventions**. One is that each thread has its own stack, so the function can be *reentrant*. Another benefit is that recursion is automatically supported: when a function calls itself recursively, a return address needs to be stored for each activation of the function so that it can later be used to return from the function activation.

Excessively deep or infinite recursion is the most common cause of *stack overflow*, in which a function calls itself so many times that it consumes all the stack space. In contrast, *stack underflow* can occur when an item is read off the empty stack. 

More precisely, if a pop operation on the stack causes the stack pointer to move past the origin of the stack, a stack underflow occurs. If a push operation causes the stack pointer to increment or decrement beyond the maximum extent of the stack, a stack overflow occurs.

## Stack frames
A call stack is composed of *stack frames* that contain state information. The stack frame at the top of the stack is for the currently executing function. The stack frame may also include:
- the return address back to the function's caller
- parameters passed to the function; generally if there are only a few small parameters, processor registers will be used to pass the values, but if there are more, they can be laid out in the call stack.
- space for the local variables of the function
- operands for deeply nested arithmetic or logical operations that cannot fit in the registers
- enclosing function context
- pointer to current instance


## Structure
A typical stack is an area of computer memory with a fixed origin and a variable size. Initially the size of the stack is zero. A stack pointer, usually in the form of a hardware register, points to the most recently referenced location on the stack; when the stack has a size of zero, the stack pointer points to the origin of the stack.

The two operations applicable to all stacks are:
- a push operation, in which a data item is placed at the location pointed to by the stack pointer, and the address in the stack pointer is adjusted by the size of the data item;
- a pop or pull operation: a data item at the current location pointed to by the stack pointer is removed, and the stack pointer is adjusted by the size of the data item.

There are many variations on the basic principle of stack operations. Every stack has a fixed location in memory at which it begins. As data items are added to the stack, the stack pointer is displaced to indicate the current extent of the stack, which expands away from the origin.

Stack pointers may point to the origin of a stack or to a limited range of addresses either above or below the origin (depending on the direction in which the stack grows); however, the stack pointer cannot cross the origin of the stack. In other words, if the origin of the stack is at address 1000 and the stack grows downwards (towards addresses 999, 998, and so on), the stack pointer must never be incremented beyond 1000 (to 1001, 1002, etc.). If a pop operation on the stack causes the stack pointer to move past the origin of the stack, a stack underflow occurs. If a push operation causes the stack pointer to increment or decrement beyond the maximum extent of the stack, a stack overflow occurs.


Stacks are often visualized growing from the bottom up (like real-world stacks). They may also be visualized growing from left to right, so that "topmost" becomes "rightmost", or even growing from top to bottom. The important feature is that the bottom of the stack is in a fixed position. The illustration in this section is an example of a top-to-bottom growth visualization: the top (28) is the stack "bottom", since the stack "top" is where items are pushed or popped from.

A stack is usually represented in computers by a block of memory cells, with the "bottom" at a fixed location, and the stack pointer holding the address of the current "top" cell in the stack. The top and bottom terminology are used irrespective of whether the stack actually grows towards lower memory addresses or towards higher memory addresses.

Pushing an item on to the stack adjusts the stack pointer by the size of the item (either decrementing or incrementing, depending on the direction in which the stack grows in memory), pointing it to the next cell, and copies the new top item to the stack area. Depending again on the exact implementation, at the end of a push operation, the stack pointer may point to the next unused location in the stack, or it may point to the topmost item in the stack. If the stack points to the current topmost item, the stack pointer will be updated before a new item is pushed onto the stack; if it points to the next available location in the stack, it will be updated after the new item is pushed onto the stack.

Popping the stack is simply the inverse of pushing. The topmost item in the stack is removed and the stack pointer is updated, in the opposite order of that used in the push operation.

Many CISC-type CPU designs, including the x86, Z80 and 6502, have a dedicated register for use as the call stack stack pointer with dedicated call, return, push, and pop instructions that implicitly update the dedicated register, thus increasing code density.

The x87 floating point architecture is an example of a set of registers organized as a stack where direct access to individual registers (relative the current top) is also possible. As with stack-based machines in general, having the top-of-stack as an implicit argument allows for a small machine code footprint with a good usage of bus bandwidth and code caches, but it also prevents some types of optimizations possible on processors permitting random access to the register file for all (two or three) operands. A stack structure also makes superscalar implementations with register renaming (for speculative execution) somewhat more complex to implement, although it is still feasible, as exemplified by modern x87 implementations.

## Security
Some computing environments use stacks in ways that may make them vulnerable to security breaches and attacks. Programmers working in such environments must take special care to avoid the pitfalls of these implementations.

For example, some programming languages use a common stack to store both data local to a called procedure and the linking information that allows the procedure to return to its caller. This means that the program moves data into and out of the same stack that contains critical return addresses for the procedure calls. If data is moved to the wrong location on the stack, or an oversized data item is moved to a stack location that is not large enough to contain it, return information for procedure calls may be corrupted, causing the program to fail.

Malicious parties may attempt a stack smashing attack that takes advantage of this type of implementation by providing oversized data input to a program that does not check the length of input. Such a program may copy the data in its entirety to a location on the stack, and in so doing it may change the return addresses for procedures that have called it. An attacker can experiment to find a specific type of data that can be provided to such a program such that the return address of the current procedure is reset to point to an area within the stack itself (and within the data provided by the attacker), which in turn contains instructions that carry out unauthorized operations.

This type of attack is a variation on the buffer overflow attack and is an extremely frequent source of security breaches in software, mainly because some of the most popular compilers use a shared stack for both data and procedure calls, and do not verify the length of data items. Frequently programmers do not write code to verify the size of data items, either, and when an oversized or undersized data item is copied to the stack, a security breach may occur.


---

https://msdn.microsoft.com/en-us/library/ew5tede7(v=vs.140).aspx
https://docs.microsoft.com/en-us/cpp/build/stack-allocation
