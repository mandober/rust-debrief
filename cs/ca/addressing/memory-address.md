# Memory Address


**Memory address** is a reference to a specific memory location.

**Main memory** (sometimes just RAM) is the memory backed-up by the DRAM memory modules that may be inserted into the motherboard's DIMM slots.

supporting various 
- technologies: channels (DDR2, DDR3, etc.)
- speed (1033 MHz, 1666 Mhz, etc.)
- access time
of various capacity (may be 4GB, 8GB, etc.), , e.g. DDR2, DDR3, etc.


Main memory may be presented as a linear 1-dimensional array, where elements are cells, each capable of storing 8 bits i.e. each cell has capacity to store 1 byte.

The smallest addressable unit (of memory) is a byte.

Each memory address specifies a byte. Memory addrresses start at 0 and increase by one, with the last memory address being influenced by different factors of which the amount of physical memory installed in a computer is not the most important.

Conventionally, for human consumption, a memory address is presented as a hexadecimal number (denoted by `0x` prefix), e.g. 0x0 represents the memory address zero.

A piece of data of exactly 1 byte is referenced directly by a memory address. For example, the smallest data type, a Boolean, needs only 1 bit to represent its values; 0 represents `false` and 1 represent `true`. However, since the smallest addressable unit is a byte, that means the size of a boolean value has to be 1 byte; a single bit is used and the other 7 bits are wasted.

A byte (8 bits) has 2^8 = 256 different states (values); other single-byte-sized data types, that use all of the 256 values are 8-bit unsigned integers (range: 0-255), 8-bit signed integers (-128 to 127) and in some languages a character type, `char`, that usually represents ASCII encoded characters.

Data types bigger then 1 byte use as many bytes (in a contiguous sequence) as they need, but their address refers only to the initial byte in that sequence; and their type reveals how many bytes total they consist of. 

The memory address of the initial byte of a datum is considered the memory address (or base memory address) of the entire datum.

Each memory location has a physical address which is an index in the (memory) array.

CPU uses the index to access the corresponding memory location.

Generally, only BIOS, OS and some specialized utilities have need to address physical memory directly.

The memory controllers' bus consists of a number of parallel lines, each represented by a binary digit. The width of the bus, and thus the number of addressable storage units, and the number of bits in each unit, varies among computers.

## Physical Address
In computing, a physical address (also real address, or binary address), is a memory address that is represented in the form of a binary number on the address bus circuitry in order to enable the data bus to access a particular storage cell of main memory, or a register of memory mapped I/O device.

### Use by CPU
In a computer supporting virtual memory, the term physical address is used mostly to differentiate from a virtual address. In particular, in computers utilizing a memory management unit (MMU) to translate memory addresses, the virtual and physical addresses refer to an address before and after translation performed by the MMU, respectively.

### Unaligned addressing
Depending upon its underlying computer architecture, the performance of a computer may be hindered by unaligned access to memory. For example, a 16-bit computer with a 16-bit memory data bus, such as Intel 8086, generally has less overhead if the access is aligned to an even address. In that case fetching one 16-bit value requires a single memory read operation, a single transfer over a data bus.

If the 16-bit data value starts at an odd address, the processor may need to perform two memory read cycles to load the value into it, i.e. one for the low address (throwing away half of it) and then a second read cycle to load the high address (throwing away again half of the retrieved data). On some processors, such as the Motorola 68000 and Motorola 68010 processors, and SPARC processors, unaligned memory accesses will result in an exception being raised (usually resulting in a software exception, such as POSIX's SIGBUS, being raised).

### Use by other devices
The direct memory access (DMA) feature allows other devices besides CPU, to address the main memory. Such devices need to know hoe to deal with physical memory addresses.

An __address bus__ is a computer bus (a series of lines connecting two or more devices) that is used to specify a physical address. When a processor or DMA-enabled device needs to read or write to a memory location, it specifies that memory location on the address bus (the value to be read or written is sent on the data bus).

> The width of the address bus determines addressable amount of memory.

For example, a system with a 32-bit address bus can address 2^32 (4,294,967,296) memory locations. If each memory location holds one byte, the **addressable memory space** is 4 GB.


## Implementation
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


---

Virtual memory system abstracts the memory management by mapping memory addresses, used by a program, called virtual addresses, into physical addresses in computer memory, thereby increasing security by isolating the processes and providing extended space, beyond the physically available RAM, through paging and swapping.

The OS manages virtual address spaces and the assignment of real memory to virtual memory. The A CPU has memory management unit (MMU) for automatic virtual to physical address translation. The program sees the memory as a vast contiguous address space, his happy place.

An address space defines a range of discrete addresses, each of which corresponds to a memory location.

---

For software programs to save and retrieve stored data, each unit of data must have an address where it can be individually located or else the program will be unable to find and manipulate the data.

Address spaces are created by combining enough uniquely identified qualifiers to make an address unambiguous within the address space.

An address space usually provides (or allows) a partitioning to several regions according to the mathematical structure it has. In the case of total order, as for memory addresses, these are simply chunks. 

Another common feature of address spaces are mappings and translations, often forming numerous layers. This usually means that some higher-level (virtual) address must be translated to lower-level (physical) ones in some way.

An iconic example of virtual-to-physical address translation is virtual memory, where different pages of virtual address space map either to page file or to main memory physical address space. It is possible that several numerically different virtual addresses all refer to one physical address and hence to the same physical byte of RAM. It is also possible that a single virtual address maps to zero, one, or more than one physical address.


---

## Memory Addressing

## DRAM access
A program selects a memory location using a virtual address. The processor translates this into a physical address and finally the memory controller selects the RAM chip corresponding to that address. To select the individual memory cell on the RAM chip, parts of the physical address are passed in the form of a number of address lines.

Addressing memory locations individually would be impractical: 4GB of RAM would require 2^32 address lines; instead the address is passed encoded as a binary number using a smaller set of address lines.

The address passed to the DRAM chip this way must be demultiplexed first. A demultiplexer with N address lines has 2^N output lines. These output lines can be used to select the memory cell. Using this direct approach is not a big problem for chips with small capacities, but if the number of cells grows, this approach is not suitable anymore.

A 1GB chip would need 30 address lines and 2^30 select lines. The size of a demultiplexer increases exponentially with the number of input lines when speed is not to be sacrificed. A demultiplexer for 30 address lines needs a lot of chip real estate in addition to the complexity (size and time) of the demultiplexer. Even more importantly, transmitting 30 impulses on the address lines synchronously is much harder than transmitting "only" 15 impulses. Fewer lines have to be laid out at exactly the same length or timed appropriately (DDR3 can automatically adjust the timing, but its tolerance has a limit).


## SDRAM
Synchronous DRAM works relative to a time source - the memory controller provides the clock, the frequency of which determines the speed of the Front Side Bus (FSB).

RAM cells are in a matrix, so selecting a cell means selecting the appropriate row and column. To save resources (access lines), the address is sent split in two parts: half of the address that selects the row is sent first, opening the Row Access Signal (`RAS`) line. After a predetermined RAS-to-CAS Delay (tRCD), the second part of the address that selects the column can be sent, opening the Column Access Signal (`CAS`) line. The addressing phase is then complete. After a CL (CAS Latency) delay needed for preparation, the data is read. With read complete, the RAS can remain open allowing the memory controller to only send another CAS (column selecting half of the address) to select the column, making it possible to quickly access another cell in the same row. The CAS resending can happened at `Tx` Command Rate, where x is 1 or 2 (cycles).

With all this preparation to get to the data, it would be wasteful to only transfer one data word. This is why DRAM modules allow the memory controller to specify how much data is to be transmitted. Often the choice is between 2, 4, or 8 words. This allows filling entire lines in the caches without a new RAS/CAS sequence.

SDRAM transmits one word per cycle. This is what the first generation does. DDR is able to transmit two words per cycle. This cuts down on the transfer time but does not change the latency. In principle, DDR2 works the same although in practice it looks different.

Another constraint is that an SDRAM module needs time after a RAS signal before it can precharge another row (denoted as tRAS). This number is usually pretty high, in the order of two or three times the tRP value. 

DDR modules are described with `w-x-y-z-T` notation (`CL-tRCD-tRP-tRAS-Tx`).

For instance, `2-3-2-8-T1`:
- w=2 CAS Latency (CL)
- x=3 RAS-to-CAS delay (tRCD)
- y=2 RAS Precharge (tRP) (Row Precharge time)
- z=8 Active to Precharge delay (tRAS)
- T=1 Command Rate


---

> Excerpts from ["What every programmer should know about memory"][am] by Ulrich Drepper, 2007-09-21

[am]: https://lwn.net/Articles/250967/
