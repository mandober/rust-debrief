# Word

- word is a fixed-sized datum used as a unit of data by a given architecture
- word size (number of bits) influences architecture's characteristics:
  - machine integers are word-sized
  - memory address has the size of word
  - registers are word-sized
  - largest memory chunk transferred in a single operation is word-sized
  - machine instructions


A word is the unit of data used by a particular CPU architecture. A word is a fixed-sized piece of data, handled as a unit, by instruction set and CPU. The number of bits in a word (word size, word width, word length) is an important characteristic of any architecture as it affects a lot of aspects, e.g.
- Size of __integers__: ints may be available in different sizes, but one of those sizes will always be the word. The smaller sizes are employed for efficient use of memory; when loaded, they are usually stored in a larger, word-sized, register.
- __Memory address__ must be of a size capable of expressing the needed range of values, so it is often word-sized.
- __CPU registers__ have the size appropriate for the type of data they hold (ints, floats or address). Many architectures use _general purpose registers_ that can hold any of the several data types, so they must be capable of holding the largest, which is word-sized.
- __Memory - CPU transfer__, transfer between the memory and the registers, i.e. the amount of data transferred, is often a word. In the simplest memory subsystems, the word is transferred over the memory data bus, which has a width of a word. In memory subsystems that use caches, the word-sized transfer is the one between CPU and L1 cache.
- __Unit of address resolution__: successive address values designate successive units of memory; this unit is the unit of address resolution. In most computers, the unit is either a byte or a word. _If the unit is a word_, then a larger amount of memory can be accessed using an address of a given size at the cost of added complexity to access individual characters. _If the unit is a byte_, then individual characters can be addressed.
- __Machine instructions__ are normally the size of word. This is a natural choice since instructions and data usually share the same memory subsystem.


x86-64
* word (pointer) size: 64b (8 bytes)
* int sizes: 8b, 16b, 32b, 64b
* float sizes: 32b, 64b, 80b
* Instruction sizes: 8-15b
* Unit of address resolution: 8b (1 byte)


arch    | WS | ints          | floats     | IS
--------|----| --------------|------------|------
ARMv1   | 32 | 8, 32         |            | 32
ARMv4   | 32 | 8, 16, 32     |            | 32,(16)
ARMv6   | 32 | 8, 16, 32     | 32, 64     | 16,32
ARMv8-A | 64 | 8, 16, 32, 64 | 32, 64     | 32
MIPS    | 32 | 8, 16, 32     | 32, 64     | 32
PowerPC | 32 | 8, 16, 32     | 32, 64     | 32
Alpha   | 64 | 8, 16, 32, 64 | 64, 128    | 32
IA-32   | 32 | 8, 16, 32     | 16, 32, 80 | 8-15b
AMD64   | 64 | 8, 16, 32, 64 | 32, 64, 80 | 8-15
IA-64   | 64 | 8, 16, 32, 64 | 32, 64     | 41


## Word and byte addressing

The resolution unit of a memory address, i.e. the smallest unit that can be designated by an address, is commonly either a word or a byte.


### Word addressing

If word is used, address values that differ by one designate adjacent memory words. This is natural in machines that deal in word (or multiples of word) units, and has the advantage of allowing instructions to use minimally sized fields to contain addresses, making the size of instruction set smaller. 

Individual bytes can be accessed. One way is to manipulate bytes by a combination of shift and mask operations in registers. Alternatively, many word-oriented machines implement byte operations with instructions using special byte pointers in registers or memory.


### Byte addressing
On the other hand, when doing byte processing, it is usually better to use the byte. This allows an arbitrary character within a string to be addressed easily. A word can still be addressed, but the address to be used requires a few more bits than the alternative, e.g. the word size needs to be a multiple of the character size.




---

[Word](https://www.wikiwand.com/en/Word_(computer_architecture))