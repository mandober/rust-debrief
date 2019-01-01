# Addressing

## Canonical address
Currently, only 48 LSB bits of a virtual address are actually used in address translation. Moreover, it is required that the most significant 16 bits of virtual address, bits 48-63, be copies of bit 47, else CPU raises an exception.

Addresses complying with this rule are referred to as "canonical form". This makes a total of 256 TB of usable virtual address space.

Span: `0 - 7FFF FFFF FFFF` and `FFFF 8000 0000 0000 - FFFF FFFF FFFF FFFF`.

Addresses start at `0` and increase by 1 all the way to `7FFF FFFF FFFF`, which is the highest address where bit 47 is still 0; it concludes the first (lower) half of the available address space.

The second (higher) half of the available address space has the bit 47 set to 1, which also sets bits 48-63 to 1, and begins with `FFFF 8000 0000 0000`. The last address is the one with all 64 bits set to 1, `FFFF FFFF FFFF FFFF`. Disregarding the automatically mirrored 16 most significant bits, this span is actually `8000 0000 0000` to `FFFF FFFF FFFF`.

Many OS (Windows) take the higher-addressed half of the address space (kernel space) for themselves and leave the lower-addressed half (user space) for application code, user mode stacks, heaps, etc.


## Physical Address
A physical address (real address) is a memory address that is represented in the form of a binary number on the address bus circuitry in order to enable the data bus to access a particular storage cell of main memory, or a register of memory mapped I/O device.

**Use by central processing unit**    
In a computer supporting virtual memory, the term physical address is used mostly to differentiate from a virtual address. In particular, in computers utilizing a memory management unit (MMU) to translate memory addresses, the virtual and physical addresses refer to an address before and after translation performed by the MMU, respectively.

**Unaligned addressing**     
Depending upon its underlying computer architecture, the performance of a computer may be hindered by unaligned access to memory. For example, a 16-bit computer with a 16-bit memory data bus, such as Intel 8086, generally has less overhead if the access is aligned to an even address. In that case fetching one 16-bit value requires a single memory read operation, a single transfer over a data bus.

If the 16-bit data value starts at an odd address, the processor may need to perform two memory read cycles to load the value into it, i.e. one for the low address (throwing away half of it) and then a second read cycle to load the high address (throwing away again half of the retrieved data). On some processors, such as the Motorola 68000 and Motorola 68010 processors, and SPARC processors, unaligned memory accesses will result in an exception being raised (usually resulting in a software exception, such as POSIX's SIGBUS, being raised).

**Use by other devices**       
This section needs expansion. You can help by adding to it. (January 2012)
The direct memory access (DMA) feature allows other devices in the mother board but CPU to address the main memory. Such devices, therefore, also need to have a knowledge of physical addresses.

### Implementation
Early processors used a wire for each bit of the address width. For example, a 16-bit address bus had 16 physical wires making up the bus. As the buses became wider and lengthier, this approach became expensive in terms of the number of chip pins and board traces. Beginning with the Mostek 4096 DRAM, address multiplexing implemented with multiplexers became common. In a multiplexed address scheme, the address is sent in two equal parts on alternate bus cycles. This halves the number of address bus signals required to connect to the memory. For example, a 32-bit address bus can be implemented by using 16 lines and sending the first half of the memory address, immediately followed by the second half memory address.
