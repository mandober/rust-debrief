# Memory Types

## Static random access memory (SRAM)
- A common SRAM cell consists of 6 transistors.
- The cell gives a nearly perfect rectangular signal.
- The signal quickly changes between the states, representing 0 and 1.
- The state is immediately available for reading.
- SRAM is the fastest type of memory with access time as low as 2 CPU cycles
- it is the most expensive memory due to production complexity.
- cell needs individual constant power for transistors maintaining the state
- cell state is stable and no refresh cycles are needed, as opposed to DRAM.


## Dynamic random access memory (DRAM)
- A common DRAM cell consists of one capacitor and one transistor.
- The capacitor keeps the state while the transistor guards the access.
- The problem is the capacitor drains ("leaks") too quickly, so
- the cell must be constantly refreshed, every 64ms (2008).
- During the refresh cycle no access to the memory is possible.
- capacitor holds a tiny charge so the signal is not directly usable.
- sense amplifier distinguish a concrete state over a range of charges.
- charging and draining a capacitor is not instantaneous.
