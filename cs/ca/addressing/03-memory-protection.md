

**Memory protection** is about controlling memory access, preventing processes from accessing unauthorized memory, isolating processes from each other and OS.

An attempt to access unowned memory results in a hardware fault (segmentation fault or storage violation exception) generally causing abnormal termination of the offending process.

With regards to security, memory protection encompases techniques like
- NX bit
- executable space protection
- address space layout randomization (ASLR)

Segmentation refers to dividing a computer's memory into segments. A reference to a memory location includes a value that identifies a segment and an offset within that segment.


## Segmentation fault
(segfault) or access violation is a fault, or failure condition, raised by hardware with memory protection, notifying the OS that a program attempted to access a restricted area of memory, thereby commiting a memory access violation.

On standard x86 computers, this is a form of general protection fault. The OS kernel will, in response, usually perform some corrective action, generally passing the fault on to the offending process by sending the process a signal. Processes can in some cases install a custom signal handler, allowing them to recover on their own, but otherwise the OS default signal handler is used, generally causing abnormal termination of the process (a program crash), and sometimes a core dump.

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
