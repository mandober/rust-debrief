# Address space

<!-- TOC -->

- [IA-32](#ia-32)
- [IA-64](#ia-64)
- [x86-64](#x86-64)
- [AMD64](#amd64)
- [Intel 64](#intel-64)

<!-- /TOC -->


## IA-32
IA-32 is the availability of 32-bit general-purpose processor registers (for example, EAX and EBX), 32-bit integer arithmetic and logical operations, 32-bit offsets within a segment in protected mode, and the translation of segmented addresses to 32-bit linear addresses. The designers took the opportunity to make other improvements as well. Some of the most significant changes are described below.

32-bit integer capability
All general-purpose registers (GPRs) are expanded from 16 bits to 32 bits, and all arithmetic and logical operations, memory-to-register and register-to-memory operations, etc., can operate directly on 32-bit integers. Pushes and pops on the stack default to 4-byte strides, and non-segmented pointers are 4 bytes wide.

The IA-32 architecture defines a 48-bit segmented address format, with a 16-bit segment number and a 32-bit offset within the segment. Segmented addresses are mapped to 32-bit linear addresses.

the Physical Address Extension allowed 36-bit physical addresses, although the linear address size was still 32 bits.

## IA-64
It is a 64-bit register-rich explicitly parallel architecture. The base data word is 64 bits, byte-addressable. The logical address space is 2^64 bytes. The architecture implements 128 integer registers, 128 floating point registers, 64 one-bit predicates, and eight branch registers. The floating point registers are 82 bits long to preserve precision for intermediate results.

## x86-64
x86-64 (x86_64, AMD64, Intel64) is the 64-bit version of the x86 instruction set. It supports vastly larger amounts (theoretically, 2^64 bytes or 16 exabytes) of virtual memory and physical memory. It also provides 64-bit general-purpose registers.


## AMD64
In the current implementation of AMD64, pointers are 64 bit long and addresses only extend to 48 bits.

The primary defining characteristic of AMD64 is the availability of 64-bit general-purpose processor registers (for example, rax and rbx), 64-bit integer arithmetic and logical operations, and 64-bit virtual addresses. The designers took the opportunity to make other improvements as well. Some of the most significant changes are described below.

64-bit integer capability
All general-purpose registers (GPRs) are expanded from 32 bits to 64 bits, and all arithmetic and logical operations, memory-to-register and register-to-memory operations, etc., can now operate directly on 64-bit integers. Pushes and pops on the stack default to 8-byte strides, and pointers are 8 bytes wide.

Larger virtual address space
The AMD64 architecture defines a 64-bit virtual address format, of which the low-order 48 bits are used in current implementations.[11](p120) This allows up to 256 TB (248 bytes) of virtual address space. The architecture definition allows this limit to be raised in future implementations to the full 64 bits,[11](p2)(p3)(p13)(p117)(p120) extending the virtual address space to 16 EB (264 bytes). This is compared to just 4 GB (232 bytes) for the x86.[16]
This means that very large files can be operated on by mapping the entire file into the process' address space (which is often much faster than working with file read/write calls), rather than having to map regions of the file into and out of the address space.

Larger physical address space
The original implementation of the AMD64 architecture implemented 40-bit physical addresses and so could address up to 1 TB (240 bytes) of RAM.[11](p24) Current implementations of the AMD64 architecture (starting from AMD 10h microarchitecture) extend this to 48-bit physical addresses[17] and therefore can address up to 256 TB of RAM. The architecture permits extending this to 52 bits in the future[11](p24)[18] (limited by the page table entry format);[11](p131) this would allow addressing of up to 4 PB of RAM. For comparison, 32-bit x86 processors are limited to 64 GB of RAM in Physical Address Extension (PAE) mode,[19] or 4 GB of RAM without PAE mode.[11](p4)

AMD therefore decided that, in the first implementations of the architecture, only the least significant 48 bits of a virtual address would actually be used in address translation

In addition, the AMD specification requires that the most significant 16 bits of any virtual address, bits 48 through 63, must be copies of bit 47 (in a manner akin to sign extension). If this requirement is not met, the processor will raise an exception.[11](p131) Addresses complying with this rule are referred to as "canonical form."[11](p130) Canonical form addresses run from 0 through 00007FFF'FFFFFFFF, and from FFFF8000'00000000 through FFFFFFFF'FFFFFFFF, for a total of 256 TB of usable virtual address space. This is still 65,536 times larger than the virtual 4 GB address space of 32-bit machines.

This feature eases later scalability to true 64-bit addressing. Many operating systems (including, but not limited to, the Windows NT family) take the higher-addressed half of the address space (named kernel space) for themselves and leave the lower-addressed half (user space) for application code, user mode stacks, heaps, and other data regions.[21] The "canonical address" design ensures that every AMD64 compliant implementation has, in effect, two memory halves: the lower half starts at 00000000'00000000 and "grows upwards" as more virtual address bits become available, while the higher half is "docked" to the top of the address space and grows downwards. Also, enforcing the "canonical form" of addresses by checking the unused address bits prevents their use by the operating system in tagged pointers as flags, privilege markers, etc., as such use could become problematic when the architecture is extended to implement more virtual address bits.

The first versions of Windows for x64 did not even use the full 256 TB; they were restricted to just 8 TB of user space and 8 TB of kernel space.[21] Windows did not support the entire 48-bit address space until Windows 8.1, which was released in October 2013.[21]

## Intel 64
With AMD's introduction of a 64-bit architecture backwards-compatible with x86, x86-64 (also called AMD64), in September 2003, followed by Intel's near fully compatible 64-bit extensions (first called IA-32e or EM64T, later renamed Intel 64)




https://www.wikiwand.com/en/X86_calling_conventions

Stack Allocation
https://msdn.microsoft.com/en-us/library/ew5tede7(v=vs.140).aspx
https://docs.microsoft.com/en-us/cpp/build/stack-allocation


