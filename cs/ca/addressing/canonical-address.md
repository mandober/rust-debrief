# Canonical address

Currently, only 48 LSB bits of a virtual address are actually used in address translation. Moreover, it is required that the most significant 16 bits of virtual address, bits 48-63, be copies of bit 47, else CPU raises an exception.

Addresses complying with this rule are referred to as "canonical form". This makes a total of 256 TB of usable virtual address space.

Span: `0 - 7FFF FFFF FFFF` and `FFFF 8000 0000 0000 - FFFF FFFF FFFF FFFF`.

Addresses start at `0` and increase by 1 all the way to `7FFF FFFF FFFF`, which is the highest address where bit 47 is still 0; it concludes the first (lower) half of the available address space.

The second (higher) half of the available address space has the bit 47 set to 1, which also sets bits 48-63 to 1, and begins with `FFFF 8000 0000 0000`. The last address is the one with all 64 bits set to 1, `FFFF FFFF FFFF FFFF`. Disregarding the automatically mirrored 16 most significant bits, this span is actually `8000 0000 0000` to `FFFF FFFF FFFF`.

Many OS (Windows) take the higher-addressed half of the address space (kernel space) for themselves and leave the lower-addressed half (user space) for application code, user mode stacks, heaps, etc.
