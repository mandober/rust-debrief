# Memory

* types of memory
* memory hierarchy


In general, memory is divided into persistant, used for storage, and volatile memory, used as main memory and cache. Lacking the universal memory, we use different types of memory for different applications.

Characteristics that are sought from memory are speed (in terms of the CPU speed), durability, persistance, large capacity, small cost.

Each memory type operates at a level in the memory hierarchy where another would be unsuitable. So today, a computer has megabytes of extremely fast but expensive SRAM, comprising CPU caches; gigabytes of slower but cheap DRAM, for main memory; and terabytes of the slow but persistant and cheap flash memory, for storage.

Different types of memory form the memory hierarchy, based on the response times.



## Memory menagement levels
- Hardware level
- OS level
- Program level


## Program level

* a process has its own, separate virtual address space
* processes share the same physical memory
* processes share the swap or page file on the disk
* on 32-bit, a process has 2GB of user-mode virtual address space
* programers work only with virtual addresses, no access to physical addresses
* memory that is managed (allocated and deallocated) is virtual, on the heap
* if managed automatically by GC, virtual memory is in 3 states:
  1. Free: the block of memory has no references to, it is available
  2. Reserved: the block of memory is available for your use and cannot be used for any other allocation request. You cannot store data to this memory block until it is committed.
  3. Committed: the block of memory is assigned to physical storage.
* Virtual address space suffers from fragmentation: free blocks are interspread with occupied blocks so finding a single suitable free block can fail.
* OOM occurs if out of virtual address space to reserve or physical space to commit.
* The page file is used even if demand for physical memory is low:
  The first time there is increased demand for physical memory, the OS makes room by paging out some data. Because that data is not read back in until needed, paging can occur even when the demand for physical memory is low.
