# Memory Address Register (MAR)

Memory Address Register (MAR) is the CPU register that either stores the memory address from which data will be loaded, or the address to which data will be stored.

When reading from memory, data addressed by MAR is fed into the _MDR (memory data register)_ and then used by the CPU. When writing to memory, the CPU writes data from MDR to the memory location whose address is stored in MAR. MAR, which is placed inside the CPU, goes either to the RAM or cache.

The memory address register will apply its contents to the address bus in accordance with the timing requirements of the memory concerned and is controlled by the CPU.

The Memory Address Register is half of a minimal interface between a microprogram and computer storage. The other half is a MDR.
