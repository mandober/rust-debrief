# Memory layers

Generaly, an address space defines a range of discrete addresses, each of which corresponds to a unique location (memory, disk sector, peripheral device, network host, etc.); a **memory address space** defines a range of discrete memory addresses, each corresponding to a unique location in memory.

*The problem*:    
In the early days of computing, programs had unrestricted access,they were able to manipulate the entire memory just by specifying the memory address. As the comlexity of computers grew, it was necessary to adapt a strict policy (that would curb programs' memory access, isolate them from each other, completely deny access to critical memory areas, increasing system security and memory safety).

*The solution*:     
Naturally, the solution was indirection: the memory address space was to be abstracted, forming memory layers. **Memory layers** allow strict control and management by controlling its representation. The strategy, still used today, is to define two memory representations: virtual and physical; then, the higher-level *virtual memory* is distributed to consumers (programs), while the lower-level *physical memory* is used as the underlaying storage. For this to work, the memory management unit (MMU), usually located in the CPU, is responsible for mapping and translation of addresses between these two layers.

*The benefits*:     





# Memory Model

## Flat Memory Model
Flat (linear) memory model refers to a memory addressing paradigm in which memory appears to the program as a single contiguous address space.

The key feature of a flat memory model is that the entire memory space is linear, sequential and contiguous from address zero to MaxBytes − 1.

The CPU can directly (and linearly) address all of the available memory locations without having to resort to any sort of _memory segmentation_ or _paging schemes_.

Memory management and address translation can still be implemented on top of a flat memory model in order to facilitate OS's functionality, resource protection, multitasking or to increase the memory capacity beyond the limits imposed by the processor's physical address space.
