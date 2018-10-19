# Variables

A variable is a memory location paired with a symbolic name (an identifier) that contains some quantity of information referred to as a value.

The variable name is used to refer to the stored value; this separation of name and content allows the name to be used independently of the exact information it represents.

The "value of variable" is given by the corresponding mapping in the symbol table in the environment.

Variables and scope:- (A) Automatic variables: - Each local variable in a function comes into existence only when the function is called, and disappears when the function is exited. Such variables are known as automatic variables. (B) External variables: - These are variables that are external to a function on and can be accessed by name by any function. These variables remain in existence permanently; rather that appearing and disappearing as functions are called and exited, retain their values even after the functions that set them have returned.




- Memory that is allocated on the stack and is fixed in size and scope is called static.
- Memory that uses malloc to be allocated at run time is called dynamic.


## Memory allocation

The specifics of variable allocation and the representation of their values vary widely, both among programming languages and among implementations of a given language.
Many language implementations allocate space for local variables, whose extent lasts for a single function call on the call stack, and whose memory is automatically reclaimed when the function returns.
More generally, in name binding, the name of a variable is bound to the address of some particular block (contiguous sequence) of bytes in memory, and operations on the variable manipulate that block.
Referencing is more common for variables whose values have large or unknown sizes when the code is compiled. Such variables reference the location of the value instead of storing the value itself, which is allocated from a pool of memory called the heap.

Bound variables have values. A value, however, is an abstraction, an idea; in implementation, a value is represented by some data object, which is stored somewhere in computer memory.

The program, or the runtime environment, must set aside memory for each data object and, since memory is finite, ensure that this memory is yielded for reuse when the object is no longer needed to represent some variable's value.


Objects allocated from the heap must be reclaimed, especially when the objects are no longer needed.

In a GC language the runtime environment automatically reclaims objects when extant variables can no longer refer to them. In non-GC languages the memory is allocated explicitly, and then later deallocated, to reclaim it to the memory pool so it can be reused.

Failure to do so leads to memory leaks, in which the heap is depleted as the program runs, risks eventual failure from exhausting available memory.


When a variable refers to a data structure created dynamically, some of its components may be only indirectly accessed through the variable.

In such circumstances, GC must deal with a case where only a portion of the memory reachable from the variable needs to be reclaimed.



---
https://en.wikiversity.org/wiki/Introduction_to_Programming/Scope
https://www.wikiwand.com/en/Variable_(computer_science)

