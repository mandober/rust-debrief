# Programming Paradigms


# Main Memory
- The main memory is a temporary storage device that holds both a program and the data it manipulates while the processor is executing the program.
- Physically, main memory consists of a collection of dynamic random access memory(DRAM) chips.
- Logically, memory is organized as a linear array of bytes, each with its own unique address (array index) starting at zero.
- In general, each of the machine instructions that constitute a program can consist of a variable number of bytes. 
- The sizes of data items that correspond to C program variables vary according to type. For example, on an x86-64 machine running Linux, data of type short require 2 bytes, types int and float 4 bytes, and types long and double 8 bytes.


# CPU
- The CPU is the engine that interprets and executes instructions.
- At its core is a word-size register called the program counter (PC). 
- At any point in time, the PC contains the address of some machine-language instruction in main memory.


> Example: Sandy Bridge architecture, Intel Core i5-2500K, 3.70 GHz
> - Level 1 cache size:
>   - L1i: 4 x 32 KB 8-way set associative INSTRUCTION caches
>   - L1d: 4 x 32 KB 8-way set associative DATA caches
>   - latency: 4, @3.3GHz approx. 1.21 ns
>   - per core (4 cores), 32768 B each
> - Level 2 cache size:
>   - 4 x 256 KB 8-way set associative caches
>   - latency: 11
>   - per core (4 cores)
> - Level 3 cache size:
>   - 6 MB 12-way set associative SHARED CACHE
>   - latency: 25
> Accessing instructions stored in the L1 would take 4 cycles or, which, at 3.3GHz, is approx. 1.21 ns.

