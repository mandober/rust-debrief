## Alignment

**Data structure alignment** is the way data is arranged in the main memory and it encompasses 3 separate, but related issues:
1. Data alignment
2. Data structure padding
3. Data structure packing

CPU performs reads and writes to memory the most efficiently when the data is naturally aligned i.e. when the data address is a multiple of the data size.

Data alignment refers to aligning elements according to their natural alignment. To ensure natural alignment, it may be necessary to insert some padding between structure elements or after the last element of a structure. Many languages handle data alignment automatically. Rust is among languages that allow some degree of control over alignment and padding.

A memory address `x`, is said to be `n-byte aligned` when `x` is a multiple of `n` bytes (where `n` is a power of 2). In this context a byte is the smallest unit of memory access, i.e. each memory address specifies a different byte. An `n-byte aligned` address would have a minimum of `log2(n)` least significant zeros (LSZ) when expressed as binary number.


        n | n^2 |   << 1 |   LSZ   |
---------:|----:|-------:| -------:|
        1 | 0^2 |      1 |
       10 | 1^2 |      2 |       1
      100 | 2^2 |      4 |       2
     1000 | 3^2 |      8 |       3
    10000 | 4^2 |     16 |       4
   100000 | 5^2 |     32 |       5
  1000000 | 6^2 |     64 |       6
 10000000 | 7^2 |    128 |       7
100000000 | 8^2 |    256 |       8



The alternate wording `b-bit aligned` designates a `b/8` byte aligned address (ex. 64-bit aligned is 8 bytes aligned).

A **memory access** is said to be aligned when the datum being accessed is `n` bytes long and the datum address is `n-byte` aligned. When a memory access is not aligned, it is said to be _misaligned_. Note that by definition byte memory accesses are always aligned.

A **memory pointer** that refers to primitive data that is `n` bytes long is said to be aligned if it is only allowed to contain addresses that are `n-byte` aligned, otherwise it is said to be _unaligned_. A memory pointer that refers to a data aggregate (a data structure or array) is aligned if (and only if) each primitive datum in the aggregate is aligned.

Note that the definitions above assume that each primitive datum is a power-of-2 bytes long. When this is not the case (as with 80-bit floating-point on x86) the context influences the conditions where the datum is considered aligned or not.

Data structures can be stored in memory on the stack with a static size known as _bounded_ or on the heap with a dynamic size known as _unbounded_.


*Problems*:    
To align the data to the word boundaries, it may be necessary to insert some unused bytes between the end of the last data structure and the start of the next, which is data structure padding.


Alignment is a property of a memory address, expressed as the numeric address modulo a power of 2.

For example, the address `0x0001103F` modulo 4 is 3; that address is said to be aligned to `4n+3`, where 4 indicates the chosen power of 2. The alignment of an address depends on the chosen power of two. This same address modulo 8 is 7. An address is said to be aligned to `X` if its alignment is `Xn+0`.


--

https://www.wikiwand.com/en/Data_structure_alignment
https://docs.microsoft.com/en-us/cpp/cpp/alignment-cpp-declarations
https://docs.microsoft.com/en-us/cpp/build/examples-of-structure-alignment
