# Main Memory

- Main (primary) memory is a temporary storage that holds a program, and the data that program manipulates, while the processor is executing the program.
- collection of Dynamic Random Access Memory (DRAM) chips
- organized as an array of bytes, each with its own address.


# RAM
Physically, the primary memory (colloquially just RAM) is a collection of Dynamic Random Access Memory (DRAM) chips. The DRAM chips are several orders of magnitute cheaper, physically smaller, slower then SRAM chips.

A piece of software is a called program when it's stored in secondary (or other) memory tiers. When a program is started it is loaded into the DRAM, becoming a process. Any piece of data comprising a program, initially stored in the secondary memory, must first be loaded into DRAM, before it can continue on its way to a CPU register (SRAM), where it must be located in order to be used.

## RAM organization
Logically, RAM can be viewed as a contiguous array of bytes, that is, an array whose elements' size is exactly 1 byte (8 bits). Each element in this array has its own index i.e. memory address. Akin to array's indices, memory addresses are also zero-based. Usually, they are presented as hexadecimal numbers, having several notations, e.g. $$\mathtt{0x0, 0H, 0_{16}, 0_{hex}}$$, all represent zero in hex no

The sizes of data items that correspond to C program variables vary according to type.

For example, on an x86-64 machine running Linux, data of type `short` require 2 bytes, types `int` and `float` 4 bytes, and types `long` and `double` 8 bytes.

