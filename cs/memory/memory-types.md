# Memory Types



## Static random access memory (SRAM)

A common SRAM cell consists of 6 transistors. The cell gives a nearly perfect rectangular signal, it quickly changes between the two states, representing 0 and 1. The state is immediately available for reading.

SRAM is orders of magnitude faster than DRAM, but also more expensive due to production complexity. The SRAM cell needs individual constant power for the transistors that maintain the state, making the cell state is stable, so no refresh cycles are needed, as opposed to DRAM.


## Dynamic random access memory (DRAM)

A common DRAM cell consists of one capacitor and one transistor. The former keeps the state while the latter guards the access.

The problem is the capacitor drains ("leaks") too quickly, so the cell must be constantly refreshed - every 64ms (most DRAM chips in 2008). During the refresh cycle no access to the memory is possible.

The second problem, resulting from the tiny charge of the capacitor, is that the signal (charge) from the cell is not directly usable. Unlike the SRAM's, the signal is not even close to rectangular shape so it first must be picked up by a sense amplifier that tries to distinguish a concrete value (0 or 1) over a whole range of charges. The third problem is that charging and draining a capacitor is not instantaneous.



---

## Addressing

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