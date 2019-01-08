# Program's layout in Memory 

https://manybutfinite.com/post/anatomy-of-a-program-in-memory/

- Each process in a multi-tasking OS runs in its own memory sandbox.
- This is the virtual address space, which in 32-bit mode is always a 4GB block of memory addresses.
- These virtual addresses are mapped to physical memory by page tables, which are maintained by the operating system kernel and consulted by the processor.
- Each process has its own set of page tables
- but once the virtual addresses are enabled, they apply to all software running in the machine, including the OS (and its kernel).
- Thus, a portion of the virtual address space must be reserved for kernel

*Kernel space*: kernel reserves 1GB (linux) or 2GB (Windows) of the high address range for itself, leaving 2-3 GB in the lower range for *User Mode Space*

This does not mean the kernel uses that much physical memory, only that it has that portion of address space available to map whatever physical memory it wishes.

Kernel space is flagged in the page tables as exclusive to privileged code (ring 2 or lower); The user code cannot access (read or write) the addresses of the kernel space; doing so would trigger a *segmentation fault* (SEGFAULT).


In Linux, kernel space is constantly present and maps the same physical memory in all processes.

Kernel code and data are always addressable, ready to handle interrupts or system calls at any time. In contrast, the mapping for the user-mode portion of the address space changes whenever a process switch happens.


The layout of a process under Linux (x32):

```
┌─────────────────────────┐ 0xFFFF FFFF max (kernel) address
│ KERNEL SPACE            │   ↑
│                         │   1 GB
│                         │   ↓
├─────────────────────────┤ 0xC000 0000 first kernel address
├─────────────────────────┤ 0xBFFF FFFF max user address
│ random stack offset     │ 
├─────────────────────────┤ 
│ STACK: grows down     ↓ │ 
│ toward lower addresses  │
├─────────────────────────┤
│ random mmap offset      │
├─────────────────────────┤
│ mmap                  ↓ │
├─────────────────────────┤
│ free                    │
│                         │
├─────────────────────────┤
│ heap                  ↑ │ ↑ grows up, toward higer addresses
│                         │
│                         │
├─────────────────────────┤
│ random heap offset      │
├─────────────────────────┤
│ bss                     │
├─────────────────────────┤
│ data                    │
├─────────────────────────┤
│ text                    │
├─────────────────────────┤0x0804 8000
│ (reserved)              │
└─────────────────────────┘0x0 min (user) address

```

Linux x32 system:
- available memory: 4GB
- address space: 32-bit
- total addresses: 0-4,294,967,295
- Kernel space: 1GB
- Kernel address range: 0xC000 0000 - 0xFFFF FFFF
- User Mode: 3GB
- User address range: 0x0 - 0xBFFF FFFF


0xFFFF FFFF  4GB         4,294,967,295 th (highest) address
0xBFFF FFFF  3GB Kernel  3,221,225,471 th
0x7FFF FFFF  2GB
0x3FFF FFFF  1GB         1,073,741,823 rd address
0x0000 0000  0GB User    1st address


http://duartes.org/gustavo/blog/post/journey-to-the-stack/
http://duartes.org/gustavo/blog/post/anatomy-of-a-program-in-memory/
http://css.csail.mit.edu/6.858/2017/readings/i386/s02_03.htm

