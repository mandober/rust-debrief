# Instruction Set Architecture

- ISA is abstract computer model; interface between software and hardware.
- ISA defines all actions a CPU can perform.
- by complexity: RISC, CISC, (VLIW, LIW, EPIC)


Generally, an ISA defines:
- Supported data types (8-bit, 16-bit, 32-bit, word, qword)
- Available states (e.g. main memory, registers, caches)
- State semantics (e.g. memory consistency, addressing modes)
- Instructions set (machine instructions comprising comp's machine lang)
- IO model


## Classification

by Architectural complexity:
* RISC
* CISC


**Complex Instruction Set Computer** (CISC) is driven by enhancement. It packs the lot including many specialized (possibly infrequentlly used) instructions; if an operation can be made more optimal by providing a dedicated instruction, it will usually make its way into the ISA.

**Reduced Instruction Set Computer** (RISC) is driven by ease of implementation. It simplifies the CPU by implementing only the basic instructions.

Other architectures include:
- **VLIW** Very long instruction word
- **LIW**  Long instruction word
- **EPIC** Explicitly parallel instruction computing
- **MISC** Minimal instruction set computer
- **OISC** One instruction set computer

The former 3 architectures aim to exploit instruction-level parallelism, thus reducing hardware, by having the compiler issue instructions and perform scheduling. The latter two aim to decrease complexity even further then RISC.



---


https://www.wikiwand.com/en/Reduced_instruction_set_computer

https://www.wikiwand.com/en/Minimal_instruction_set_computer
https://www.wikiwand.com/en/One_instruction_set_computer

