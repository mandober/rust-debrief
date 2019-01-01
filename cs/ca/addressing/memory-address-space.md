## Memory address space

- memory is a byte array of cells that can store some quantity of bits
- Each cell has a number identify itself, called its address
- Adjacent cells have consecutive addresses.
- memory with n cells has 0 to n-1  addresses
- n-bit CPU can access 0 - 2^n-1 address range
- Maximum addressable memory size is n * 2^n
- 32-bit CPU can address 4 GB of memory.


Each cell has a number identify itself, called its address.
Programs refers addresses to reach memory.
Adjacent cells have consecutive addresses. If memory has m cells, the cells will
have addresses 0 to m-1. 
If CPU supports n bit, it can refer addresses from 0 to 2^n – 1.
Intel Pentium II is a 32-bit CPU and can addresses 4 GBytes of memory.
Each cell stores an integer.
An integer is n-bit number.
It is 32 bit(4 bytes) if you have a Intel P2.
Maximum addressable memory size = n * 2^n
In recent years, nearly all manufactures have standardized on an 8-bit cell which is called byte.
Bytes are grouped into words. 
An 32-bit CPU has 4-bytes/word. 
This CPU has has 32-bit registers
can holds 32-bits at a time. 
For that reason, registers that access to memory are also 32 bits. 
So it can point maximum to 11111111111111111111111111111111 in binary format(0xFFFFFFFF in hexal form).
That is same with the 2^n – 1.


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
