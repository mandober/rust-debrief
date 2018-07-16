# Memory address space

A process usually has in its address space chunks of memory of two types:
- Machine code that includes:
  - program's own code (.text segment)
  - shared libraries
- Data code that includes:
  - initialized data (.data segment)
  - uninitialized, but allocated variables (.bss)
  - run-time stack
  - heap
  - shared memory and memory mapped files.

Some parts of address space may be not mapped at all.

