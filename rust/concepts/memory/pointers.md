# Pointers in Rust


A variable is a named memory chunk storing a value of some type.

It only refers to the first byte of that chunk, but the compiler knows its size (i.e. it knows how much bytes it actually refers to) by its type.



---

To a 64-bit process running on x86_64 architecture on a computer equipped with the maximum allowed amount of memory i.e. 2 EiB (2,048 PiB or 2,097,152 TiB), memory is presented as an array, with byte-sized cells, which is indexed from zero to 2^64-1, which in hex is: `0 - FFFF FFFF FFFF FFFF`.

The minimum addressable size (unit of memory) is a byte.
A value consisting of a single byte (e.g. `char`) gets stored at some memory address. To read that value only that single memory address is needed.
sequence of bytes located at some memory addresses, is realized through the use of variables.



A pointer, being a variable, has a name and type; its type (base type) is the same type as that of a value (variable) it points to. Its direct value is a memory address which it gets via "address of" operator from a variable; its indirect value is the value beginning at that memory address. A pointer points to a single memory address, but its base type determines how many bytes to access from there on.

For example, a variable of pointer to `int` type points to a single memory address, but if `int` is 4 bytes long (depending on the implementation and platform) the pointed to byte at that address plus the next 3 bytes are accessed to form a value of `int` type. In the case of `char`, whose size is always 1 byte, it is just the pointed to byte that is accessed.

The size of a pointer variable is the same regardless of its base type, but the size of memory (in byte-sized chunks) that will be accessed when retrieving the pointed to data depends on the base type of the pointer variable.

In a way, types in C could be ordered horizontally and vertically: all non-pointer types, like `int`, `char`, `float`, `double`, etc., would be ordered horizontally. Then the available types double by each level of (pointer) indirection, so each "horizontal" type "carries" indirection ("vertical") types: `(int *)` (pointer-to-int type), `(int **)` (pointer-to-pointer-to-int type), `(int ***)`, and so on. Thus, there are theoretically infinitely many types in C.

A pointer has its own type and the base type, which is the type of value it points to, so the base type of `(char *)` is `char` (1 byte is accessed), but the base type of `(char **)` is `(char *)` (8 bytes on x64 are accessed), and for `(char ***)` the base type is `(char **)`(again, 8 bytes @ x64); the base types for  

All (valid) pointers store a memory address, so they all have the same size. That size is determined by architecture of the host platform; in x32 systems it is 4 bytes and in x64 it is 8 bytes. The size of pointer needs to be big enough to accommodate the biggest available memory address (on a given platform).

Pointers contain a number that represents a memory location, and in that regard they are all the same - it is the type system that constrains them to a certain type, classifying them as pointers to integers, to characters, to a user-defined type, etc. The compiler will complain if a pointer to, for example, an integer is assigned the address of a float. Nevertheless, they can easily change their (base) type by casting.


<!-- TOC -->

- [Memory protection](#memory-protection)
- [segmentation fault](#segmentation-fault)
- [null pointer](#null-pointer)
- [Bounded pointer](#bounded-pointer)
- [Tagged pointer](#tagged-pointer)
- [Function Pointer](#function-pointer)
- [Smart Pointer](#smart-pointer)
- [Opaque pointer](#opaque-pointer)
- [Functors](#functors)

<!-- /TOC -->

A variable is a memory address paired with an associated symbolic name that contains some quantity of information referred to as a value. Using that symbolic name (the name of a variable) is the usual way to reference the stored value. The "value of variable" is given by the corresponding mapping in the symbol table in the environment.

A pointer is a primitive whose value is a memory address.

If that memory address designates a valid value


refers to ("points to") another value in the memory using its memory address.

A pointer references a location in memory, and obtaining the value stored at that location is known as dereferencing the pointer.

A memory pointer (or just pointer) is a primitive, whose value is intended to be used as a memory address; a pointer points to a memory address; a pointer points to a data in memory when the pointer's value is the data's memory address.

More generally, a pointer is a kind of reference, it references a datum stored somewhere in memory; to obtain that datum is to dereference the pointer. The feature that separates pointers from other kinds of reference is that a pointer's value is meant to be interpreted as a memory address, which is a rather low-level concept.

References serve as a level of indirection: a pointer's value determines which memory address (that is, which datum) is to be used in a calculation. Since indirection is a fundamental aspect of algorithms, pointers are often expressed as a fundamental data type in programming languages; in statically (or strongly) typed programming languages, the type of a pointer determines the type of the datum to which the pointer points.

*in data structures*
When setting up data structures like lists, queues and trees, it is necessary to have pointers to help manage how the structure is implemented and controlled. Typical examples of pointers are start pointers, end pointers, and stack pointers. These pointers can either be absolute (the actual physical address or a virtual address in virtual memory) or relative (an offset from an absolute start address ("base") that typically uses fewer bits than a full address, but will usually require one additional arithmetic operation to resolve).

*Architectural roots*
Pointers are a very thin abstraction on top of the addressing capabilities provided by most modern architectures. In the simplest scheme, an address, or a numeric index, is assigned to each unit of memory in the system, where the unit is typically either a byte or a word – depending on whether the architecture is byte-addressable or word-addressable – effectively transforming all of memory into a very large array. The system would then also provide an operation to retrieve the value stored in the memory unit at a given address (usually utilizing the machine's general purpose registers).

In the usual case, a pointer is large enough to hold more addresses than there are units of memory in the system. This introduces the possibility that a program may attempt to access an address which corresponds to no unit of memory, either because not enough memory is installed (i.e. beyond the range of available memory) or the architecture does not support such addresses. The first case may, in certain platforms such as the Intel x86 architecture, be called a segmentation fault (segfault). The second case is possible in the current implementation of AMD64, where pointers are 64 bit long and addresses only extend to 48 bits. Pointers must conform to certain rules (canonical addresses), so if a non-canonical pointer is dereferenced, the processor raises a general protection fault.


## Memory protection
Memory protection is a way to control memory access rights on a computer, and is a part of most modern instruction set architectures and operating systems. The main purpose of memory protection is to prevent a process from accessing memory that has not been allocated to it. This prevents a bug or malware within a process from affecting other processes, or the operating system itself. An attempt to access unowned memory results in a hardware fault, called a segmentation fault or storage violation exception, generally causing abnormal termination of the offending process. Memory protection for computer security includes additional techniques such as address space layout randomization and executable space protection.

Segmentation refers to dividing a computer's memory into segments. A reference to a memory location includes a value that identifies a segment and an offset within that segment.


## segmentation fault

https://www.wikiwand.com/en/Segmentation_fault

In computing, a segmentation fault (often shortened to segfault) or access violation is a fault, or failure condition, raised by hardware with memory protection, notifying an operating system (OS) the software has attempted to access a restricted area of memory (a memory access violation). On standard x86 computers, this is a form of general protection fault. The OS kernel will, in response, usually perform some corrective action, generally passing the fault on to the offending process by sending the process a signal. Processes can in some cases install a custom signal handler, allowing them to recover on their own,[1] but otherwise the OS default signal handler is used, generally causing abnormal termination of the process (a program crash), and sometimes a core dump.

Segmentation faults are a common class of error in programs written in languages like C that provide low-level memory access. They arise primarily due to errors in use of pointers for virtual memory addressing, particularly illegal access. Another type of memory access error is a bus error, which also has various causes, but is today much rarer; these occur primarily due to incorrect physical memory addressing, or due to misaligned memory access – these are memory references that the hardware cannot address, rather than references that a process is not allowed to address.

Newer programming languages may employ mechanisms designed to avoid segmentation faults and improve memory safety. For example, the Rust programming language employs an "Ownership" based model to ensure memory safety.

The following are some typical causes of a segmentation fault:
- Dereferencing null pointers – this is special-cased by memory management hardware
- Attempting to access a nonexistent memory address (outside process's address space)
- Attempting to access memory the program does not have rights to (such as kernel structures in process context)
- Attempting to write read-only memory (such as code segment)

These in turn are often caused by programming errors that result in invalid memory access:
- Dereferencing or assigning to an uninitialized pointer (wild pointer, which points to a random memory address)
- Dereferencing or assigning to a freed pointer (dangling pointer, which points to memory that has been freed/deallocated/deleted)
- A buffer overflow
- A stack overflow
- Attempting to execute a program that does not compile correctly. (Some compilers will output an executable file despite the presence of compile-time errors.)

In C code, segmentation faults most often occur because of errors in pointer use, particularly in C dynamic memory allocation. Dereferencing a null pointer will always result in a segmentation fault, but wild pointers and dangling pointers point to memory that may or may not exist, and may or may not be readable or writable, and thus can result in transient bugs.



## null pointer
https://www.wikiwand.com/en/Null_pointer

In computing, a null pointer has a value reserved for indicating that the pointer does not refer to a valid object. Programs routinely use null pointers to represent conditions such as the end of a list of unknown length or the failure to perform some action; this use of null pointers can be compared to nullable types and to the Nothing value in an option type.

A null pointer should not be confused with an uninitialized pointer: A null pointer is guaranteed to compare unequal to any pointer that points to a valid object. However, depending on the language and implementation, an uninitialized pointer may not have any such guarantee. It might compare equal to other, valid pointers; or it might compare equal to null pointers. It might do both at different times.

Dereferencing a null pointer typically results in an attempted read or write from memory that is not mapped, triggering a segmentation fault or memory access violation.

In languages with a tagged architecture, a possibly null pointer can be replaced with a tagged union which enforces explicit handling of the exceptional case; in fact, a possibly null pointer can be seen as a tagged pointer with a computed tag.

## Bounded pointer
https://www.wikiwand.com/en/Bounded_pointer
In computer science, a bounded pointer is a pointer that is augmented with additional information that enable the storage bounds within which it may point to be deduced. This additional information sometimes takes the form of two pointers holding the upper and lower addresses of the storage occupied by the object to which the bounded pointer points.

Use of bound information makes it possible for a compiler to generate code that performs bounds checking, i.e. that tests if a pointer's value lies within the bounds prior to dereferencing the pointer or modifying the value of the pointer. If the bounds are violated some kind of exception may be raised. This is especially useful for data constructs such as arrays in C.


## Tagged pointer
https://www.wikiwand.com/en/Tagged_pointer
In computer science, a tagged pointer is a pointer with additional data associated with it, such as an indirection bit or reference count. This additional data is often "folded" into the pointer, meaning stored inline in the data representing the address, taking advantage of certain properties of memory addressing.

The name comes from "tagged union", and the additional data is called a "tag" or "tags", though strictly speaking "tag" refers to data specifying a type, not other data; however, the usage "tagged pointer" is standard.


