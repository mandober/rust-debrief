# System Bus


An **address bus** is a computer bus (a series of lines connecting two or more devices) that is used to specify a physical address. When a processor or DMA-enabled device needs to read or write to a memory location, it specifies that memory location on the address bus (the value to be read or written is sent on the data bus). The width of the address bus determines the amount of memory a system can address. For example, a system with a 32-bit address bus can address 232 (4,294,967,296) memory locations. If each memory location holds one byte, the addressable memory space is 4 GB.

Accessing an individual byte frequently requires reading or writing the full bus width (a word) at once. In these instances the least significant bits of the address bus may not even be implemented - it is instead the responsibility of the controlling device to isolate the individual byte required from the complete word transmitted. This is the case, for instance, with the VESA Local Bus which lacks the two least significant bits, limiting this bus to aligned 32-bit transfers.

Historically, there were also some examples of computers which were only able to address words.

The __memory bus__ is the computer bus which connects the main memory to the memory controller in computer systems. modern memory buses are designed to connect directly to DRAM chips.

a __control bus__ is part of the system bus, used by CPUs for communicating with other devices within the computer. While the address bus carries the information about the device with which the CPU is communicating and the data bus carries the actual data being processed, the control bus carries commands from the CPU and returns status signals from the devices. For example, if the data is being read or written to the device the appropriate line (read or write) will be active (logic one).

**Lines**
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
