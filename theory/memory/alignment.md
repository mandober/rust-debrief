## Alignment

https://www.wikiwand.com/en/Data_structure_alignment
https://docs.microsoft.com/en-us/cpp/cpp/alignment-cpp-declarations
https://docs.microsoft.com/en-us/cpp/build/examples-of-structure-alignment


Data structure alignment is the way data is arranged and accessed in computer memory. It consists of three separate but related issues: *data alignment*, *data structure padding* and *packing*. 

When a modern computer reads from or writes to a memory address, it will do this in word sized chunks, typically 32 bits or 64 bits on modern systems. Data alignment means putting the data at a memory address equal to some multiple of the word size, which increases the system's performance due to the way the CPU handles memory. To align the data to the word boundaries, it may be necessary to insert some unused bytes between the end of the last data structure and the start of the next, which is data structure padding.

Alignment is a property of a memory address, expressed as the numeric address modulo a power of 2.

For example, the address 0x0001103F modulo 4 is 3; that address is said to be aligned to 4n+3, where 4 indicates the chosen power of 2. The alignment of an address depends on the chosen power of two. The same address modulo 8 is 7. An address is said to be aligned to X if its alignment is Xn+0.

