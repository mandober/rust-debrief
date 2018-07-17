# Memory Types

## Static random access memory (SRAM)

A common SRAM cell consists of 6 transistors. The cell gives a nearly perfect rectangular signal, it quickly changes between the two states, representing 0 and 1. The state is immediately available for reading.

SRAM is orders of magnitude faster than DRAM, but also more expensive due to production complexity. The SRAM cell needs individual constant power for the transistors that maintain the state, making the cell state is stable, so no refresh cycles are needed, as opposed to DRAM.

## Dynamic random access memory (DRAM)

A common DRAM cell consists of one capacitor and one transistor. The former keeps the state while the latter guards the access.

The problem is the capacitor drains ("leaks") too quickly, so the cell must be constantly refreshed - every 64ms (most DRAM chips in 2008). During the refresh cycle no access to the memory is possible.

The second problem, resulting from the tiny charge of the capacitor, is that the signal (charge) from the cell is not directly usable. Unlike the SRAM's, the signal is not even close to rectangular shape so it first must be picked up by a sense amplifier that tries to distinguish a concrete value (0 or 1) over a whole range of charges. The third problem is that charging and draining a capacitor is not instantaneous.

