# Memory diagram

Typical memory map for the first 4 gigs of physical memory addresses in an Intel PC.

```
┌─────────────────────────┐4GB      0xff ff ff ff
│ High BIOS (2MB)         │
├─────────────────────────┤
│ PCI, APIC, DMI (~1GB)   │
├─────────────────────────┤3GB
│ Accessible RAM (~3GB)   │
│                         │
│                         │
│                         │
├─────────────────────────┤1MB      0xff ff
│ System BIOS             │
├─────────────────────────┤960KB
│ Extended System BIOS    │
├─────────────────────────┤896KB
│ Extension area          │
├─────────────────────────┤768KB
│ VGA access              │
├─────────────────────────┤640KB
│ Accessible RAM          │
└─────────────────────────┘0        0x00 00 00 00

```

Actual addresses and ranges depend on the specific motherboard and devices present in the computer, but most Core 2 systems are pretty close to the above.

Apart from accessible RAM regions, others regions are mapped; they are physical addresses that are used on the motherboard buses. Inside the CPU, the memory addresses are logical and they must be translated by the CPU into a physical address before memory is accessed on the bus.

The rules for translation of logical addresses into physical addresses are complex and they depend on the mode in which the CPU is running:
- real mode
- 32-bit protected mode
- 64-bit protected mode

CPU mode determines how much physical memory can be accessed.
- In real mode, the CPU can address 1MB of physical RAM.
- In 32-bit mode 4GB are addressable, ignoring physical address extension (PAE), but since 1GB is reserved for DMA, the CPU can effectively use 3GB at most.
- A CPU running in 64-bit mode can address 64GB of physical RAM and can even access some of the memory reserved for DMA devices.


---
http://duartes.org/gustavo/blog/post/motherboard-chipsets-memory-map/