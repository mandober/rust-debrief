memory model

__Flat memory model or linear memory model__ refers to a memory addressing paradigm in which "memory appears to the program as a single contiguous address space."[1] The CPU can directly (and linearly) address all of the available memory locations without having to resort to any sort of memory segmentation or paging schemes.

Memory management and address translation can still be implemented on top of a flat memory model in order to facilitate the operating system's functionality, resource protection, multitasking or to increase the memory capacity beyond the limits imposed by the processor's physical address space, but the key feature of a flat memory model is that the entire memory space is linear, sequential and contiguous from address zero to MaxBytes − 1.

