# Address Space


For software programs to save and retrieve stored data, each unit of data must have an address where it can be individually located or else the program will be unable to find and manipulate the data.

Address spaces are created by combining enough uniquely identified qualifiers to make an address unambiguous within the address space.

An address space usually provides a partitioning to several regions according to the mathematical structure it has. In the case of total order, as for memory addresses, these are simply chunks.

Another common feature of address spaces are mappings and translations, often forming numerous layers. This usually means that some higher-level (virtual) address must be translated to lower-level (physical) ones in some way.



## x86
- 32-bit (4B) general-purpose processor registers (GPR) (eg. EAX, EBX)
- 32-bit integer arithmetic and logical operations
- 32-bit offsets within a segment in protected mode
- 32-bit strides default for push and pop on the stack
- 32-bit wide non-segmented pointers
- 36-bit physical addresses with PAE, with 32-bit linear address size
- translation of segmented addresses to 32-bit linear addresses
- 48-bit segmented address format: 16-bit segment number, 32-bit segment offset
- byte-addressable


## x86-64
- 64-bit version of the x86 instruction set
- 2^64 bits virtual and physical memory space
- 2^64 bits (16 EB) max memory size
- 64-bit virtual addresses
- 48-bit physical addresses, gives 256 TB
- 64-bit GPRs (eg. RAX, RBX)
- 64-bit pointers
- 64-bit integer arithmetic and logical operations
- 64-bit strides are default for push and pop on the stack

Even though 64 bits are available, not all of them are used.
- 40-bit physical addresses could address up to 1 TB of RAM. (Original plan)
- 48-bit can address 256 TB of RAM. (Currently in use)
- 52-bit can address 4 PB of RAM. (Possible future extension)
- 64-bit maximum, could address 16 EB of RAM. (Eventually)


### Canonical address
Currently, only 48 LSB bits of a virtual address are actually used in address translation. Moreover, it is required that the most significant 16 bits of virtual address, bits 48-63, be copies of bit 47, else CPU raises an exception.

Addresses complying with this rule are referred to as "canonical form". This makes a total of 256 TB of usable virtual address space.

Span: `0 - 7FFF FFFF FFFF` and `FFFF 8000 0000 0000 - FFFF FFFF FFFF FFFF`.

Addresses start at `0` and increase by 1 all the way to `7FFF FFFF FFFF`, which is the highest address where bit 47 is still 0; it concludes the first (lower) half of the available address space.

The second (higher) half of the available address space has the bit 47 set to 1, which also sets bits 48-63 to 1, and begins with `FFFF 8000 0000 0000`. The last address is the one with all 64 bits set to 1, `FFFF FFFF FFFF FFFF`. Disregarding the automatically mirrored 16 most significant bits, this span is actually `8000 0000 0000` to `FFFF FFFF FFFF`.



hex                 | exp     | dec
--------------------|---------|-------------------------:
lower half:         |         |
0000 0000 0000 0000 | 2^0 -1  |                         0
0000 0000 0000 000F | 2^4 -1  |                        15
0000 0000 0000 00FF | 2^8 -1  |                       255
0000 0000 0000 FFFF | 2^16-1  |                    ‭65.535‬
0000 0000 FFFF FFFF | 2^32-1  |             4.294.967.295‬
0000 000F FFFF FFFF | 2^36-1  |            68.719.476.735
0000 00FF FFFF FFFF | 2^40-1  |         ‭1.099.511.627.775‬
0000 7FFF FFFF FFFF | 2^47-1  |       ‭140.737.488.355.327‬
0000 FFFF FFFF FFFF | 2^48-1  |       ‭281.474.976.710.655‬
higher half:        |         |
FFFF 8000 0000 0000 |         |
FFFF FFFF FFFF FFFF | 2^64-1  |


Many operating systems (Windows) take the higher-addressed half of the address space (kernel space) for themselves and leave the lower-addressed half (user space) for application code, user mode stacks, heaps, and other data regions.

The "canonical address" design ensures that every AMD64 compliant implementation has, in effect, two memory halves: the lower half starts at 0 and "grows upwards" as more virtual address bits become available, while the higher half is "docked" to the top of the address space and grows downwards.

Enforcing the "canonical form" of addresses by checking the unused address bits and preventing their use as, e.g. tagged pointers, ensures this implementation can be extended.
