# Virtual Memory

Virtual memory system abstracts the memory management by mapping memory addresses, used by a program (virtual addresses) into physical addresses in RAM. This increases security by isolating the processes from each other and provides extended space, beyond the physically available RAM, through paging and swapping techniques.

The OS manages virtual address spaces and the assignment of real memory to virtual memory. The A CPU has memory management unit (MMU) for automatic virtual to physical address translation. The program sees the memory as a vast contiguous address space, his happy place.

