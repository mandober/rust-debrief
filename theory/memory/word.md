# Word

https://www.wikiwand.com/en/Word_(computer_architecture)

Word is the fixed-sized unit of data used by a particular processor design.

The word size (width, length) is the number of bits that make up the word: in modern computers this is 32 or 64 bits.

The size of a word influences many aspects of computing: the majority of the registers in a processor are usually word-sized and the largest piece of data that can be transferred to and from the working memory in a single operation is a word. The largest possible address size, used to designate a location in memory, is typically a word.
The word size is also the unit of address resolution.
The memory model of an architecture is strongly influenced by the word size. In particular, the resolution of a memory address, that is, the smallest unit that can be designated by an address, has often been chosen to be the word.

Different amounts of memory are used to store data values with different degrees of precision. The commonly used sizes are usually a power of two multiple of the unit of address resolution (byte or word). Converting the index of an item in an array into the address of the item then requires only a shift operation rather than a multiplication. In some cases this relationship can also avoid the use of division operations. As a result, most modern computer designs have word sizes (and other operand sizes) that are a power of two times the size of a byte.

Another example is the x86 family, of which processors of three different word lengths (16-bit, later 32- and 64-bit) have been released. As software is routinely ported from one word-length to the next, some APIs and documentation define or refer to an older (and thus shorter) word-length than the full word length on the CPU that software may be compiled for. Also, similar to how bytes are used for small numbers in many programs, a shorter word (16 or 32 bits) may be used in contexts where the range of a wider word is not needed (especially where this can save considerable stack space or cache memory space).

For example, Microsoft's Windows API maintains the programming language definition of WORD as 16 bits, despite the fact that the API may be used on a 32- or 64-bit x86 processor, where the standard word size would be 32 or 64 bits, respectively. Data structures containing such different sized words refer to them as WORD (16 bits/2 bytes), DWORD (32 bits/4 bytes) and QWORD (64 bits/8 bytes) respectively.

A similar phenomenon has developed in Intel's x86 assembly language – because of the support for various sizes (and backward compatibility) in the instruction set, some instruction mnemonics carry "d" or "q" identifiers denoting "double-", "quad-" or "double-quad-", which are in terms of the architecture's original 16-bit word size.

In general, new processors must use the same data word lengths and virtual address widths as an older processor to have binary compatibility with that older processor.

```
arch: x86-64
Word size: 64b
Integer sizes: 8b, 16b, 32b, 64b
Floating­-point sizes: 32b, 64b, 80b
Instruction sizes: 8-15b
Unit of address resolution: 8b
Char size: 8b
```


arch    | ws | int        | float    | is
--------|----| -----------|----------|---------
ARMv1   | 32 | 8,32       |          | 32
IA-32   | 32 | 8,16,32    | 16,32,80 | 8-15b
MIPS    | 32 | 8,16,32    | 32,64    | 32
PowerPC | 32 | 8,16,32    | 32,64    | 32
ARMv4   | 32 | 8,16,32    |          | 32,(16)
ARMv6   | 32 | 8,16,32    | 32,64    | 16,32
--------|----| -----------|----------|---------
Alpha   | 64 | 8,16,32,64 | 64,128   | 32
IA-64   | 64 | 8,16,32,64 | 32,64    | 41
ARMv8-A | 64 | 8,16,32,64 | 32,64    | 32
x86-64  | 64 | 8,16,32,64 | 32,64,80 | 8-15


