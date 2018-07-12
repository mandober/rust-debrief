# Memory management


- virtual memory
- garbage collection
- manual memory management



The essential requirement of memory management is to provide ways to dynamically allocate portions of memory to programs at their request, and free it for reuse when no longer needed.


## Application-level memory management

At the program level, memory is managed automatically, using garbage collection, or it is managed manually.

Majority of popular programming languages, and all dynamic, require garbage collection. Even when a language is designed with manual memory management in mind, it can have garbage collection implemented. Some languages allow both methods at the same time, or allow users to opt out of garbage collection and only use manual memory management.

- [Garbage Collection](gc.md)
- [Manual Memory Management](manual-mem-manage.md)



---

A memory address is a reference to a specific memory location. Conventionally, a memory addresses is presented as a hex number.

Each memory location has a physical address which is an index in the (memory) array. The CPU uses the index to access the corresponding memory location.

Generally, only BIOS, OS and some specialized utilities have need to address physical memory directly
The memory controllers' bus consists of a number of parallel lines, each represented by a binary digit. The width of the bus, and thus the number of addressable storage units, and the number of bits in each unit, varies among computers.

---


[Memory management](https://www.wikiwand.com/en/Memory_management)

[Managing Memory-Mapped Files](https://msdn.microsoft.com/en-us/library/ms810613.aspx)