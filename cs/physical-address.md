# physical-address

In the early days of computing, programs would have direct, sole, unrestricted access to the entire physical memory and could write data anywhere just by specifying the physical memory address.

Today, programs have illusion they have direct, sole, unrestricted access to memory, when, not only is the OS solely in charge of it, but the medium itself is not even real.

Virtual memory system abstracts the memory management by mapping memory addresses, used by a program, called virtual addresses, into physical addresses in computer memory, thereby increasing security by isolating the processes and providing extended space, beyond the physically available RAM, through paging and swapping.

The OS manages virtual address spaces and the assignment of real memory to virtual memory. The A CPU has memory management unit (MMU) for automatic virtual to physical address translation. The program sees the memory as a vast contiguous address space, his happy place.

An address space defines a range of discrete addresses, each of which corresponds to a memory location.

---


In computing, a physical address (also real address, or binary address), is a memory address that is represented in the form of a binary number on the address bus circuitry in order to enable the data bus to access a particular storage cell of main memory, or a register of memory mapped I/O device.

Use by central processing unit

In a computer supporting virtual memory, the term physical address is used mostly to differentiate from a virtual address. In particular, in computers utilizing a memory management unit (MMU) to translate memory addresses, the virtual and physical addresses refer to an address before and after translation performed by the MMU, respectively.[1]

Unaligned addressing

Depending upon its underlying computer architecture, the performance of a computer may be hindered by unaligned access to memory. For example, a 16-bit computer with a 16-bit memory data bus, such as Intel 8086, generally has less overhead if the access is aligned to an even address. In that case fetching one 16-bit value requires a single memory read operation, a single transfer over a data bus.

If the 16-bit data value starts at an odd address, the processor may need to perform two memory read cycles to load the value into it, i.e. one for the low address (throwing away half of it) and then a second read cycle to load the high address (throwing away again half of the retrieved data). On some processors, such as the Motorola 68000 and Motorola 68010 processors, and SPARC processors, unaligned memory accesses will result in an exception being raised (usually resulting in a software exception, such as POSIX's SIGBUS, being raised).

Use by other devices

This section needs expansion. You can help by adding to it. (January 2012)
The direct memory access (DMA) feature allows other devices in the mother board but CPU to address the main memory. Such devices, therefore, also need to have a knowledge of physical addresses.



An __address bus__ is a computer bus (a series of lines connecting two or more devices) that is used to specify a physical address. When a processor or DMA-enabled device needs to read or write to a memory location, it specifies that memory location on the address bus (the value to be read or written is sent on the data bus). The width of the address bus determines the amount of memory a system can address. For example, a system with a 32-bit address bus can address 232 (4,294,967,296) memory locations. If each memory location holds one byte, the addressable memory space is 4 GB.

Implementation

Early processors used a wire for each bit of the address width. For example, a 16-bit address bus had 16 physical wires making up the bus. As the buses became wider and lengthier, this approach became expensive in terms of the number of chip pins and board traces. Beginning with the Mostek 4096 DRAM, address multiplexing implemented with multiplexers became common. In a multiplexed address scheme, the address is sent in two equal parts on alternate bus cycles. This halves the number of address bus signals required to connect to the memory. For example, a 32-bit address bus can be implemented by using 16 lines and sending the first half of the memory address, immediately followed by the second half memory address

Examples

Accessing an individual byte frequently requires reading or writing the full bus width (a word) at once. In these instances the least significant bits of the address bus may not even be implemented - it is instead the responsibility of the controlling device to isolate the individual byte required from the complete word transmitted. This is the case, for instance, with the VESA Local Bus which lacks the two least significant bits, limiting this bus to aligned 32-bit transfers.

Historically, there were also some examples of computers which were only able to address words.

The __memory bus__ is the computer bus which connects the main memory to the memory controller in computer systems. modern memory buses are designed to connect directly to DRAM chips.


a __control bus__ is part of the system bus, used by CPUs for communicating with other devices within the computer. While the address bus carries the information about the device with which the CPU is communicating and the data bus carries the actual data being processed, the control bus carries commands from the CPU and returns status signals from the devices. For example, if the data is being read or written to the device the appropriate line (read or write) will be active (logic one).

Lines

The number and type of lines in a control bus varies but there are basic lines common to all microprocessors, such as:

Read ({\displaystyle {\overline {RD))}). A single line that when active (logic zero) indicates the device is being read by the CPU.
Write ({\displaystyle {\overline {WR))}). A single line that when active (logic zero) indicates the device is being written by the CPU.
Byte enable (\overline E). A group of lines that indicate the size of the data (8, 16, 32, 64 bytes).
The conter bus manger the information flow between components indicated whether the operation is a read or write.
The RD and WR signals of the control bus control the reading or writing of RAM, avoiding bus contention on the data bus.[1]

Additional lines are microprocessor-dependent, such as:

Transfer ACK ("acknowledgement"). Delivers information that the data was acknowledged (read) by the device.
Bus request (BR, BREQ, or BRQ). Indicates a device is requesting the use of the (data) bus.
Bus grant (BG or BGRT). Indicates the CPU has granted access to the bus.
Interrupt request (IRQ). A device with lower priority is requesting access to the CPU.
Clock signals. The signal on this line is used to synchronize data between the CPU and a device.
Reset. If this line is active, the CPU will perform a hard reboot.
Systems that have more than one bus master have additional control bus signals that control which bus master drives the address bus, avoiding bus contention on the address bus.[1]


