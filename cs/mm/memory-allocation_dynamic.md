# Dynamic memory allocation

The task of fulfilling an allocation request consists of locating a block of unused memory of sufficient size. Memory requests are satisfied by allocating portions from a large pool of memory called the heap or free store. At any given time, some parts of the heap are in use, while some are free and available for future allocations.

Several issues complicate the implementation, such as external fragmentation, which arises when there are many small gaps between allocated memory blocks, which invalidates their use for an allocation request.

The allocator's metadata can also inflate the size of (individually) small allocations. This is often managed by chunking.

The memory management system must track outstanding allocations to ensure that they do not overlap and that no memory is leaked.


