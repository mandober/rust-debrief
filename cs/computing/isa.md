# Instruction Set Architecture

- Instruction set architecture (ISA) is an abstract model of a computer.
- ISA permits multiple impl (that vary in perf, physical size, etc.)
- ISA serves as the interface between software and hardware.
- Software written for an ISA can run on different impl of the same ISA.
- ISA enables binary compatibility between different gens of computers.
- ISA defines every action a computer can perform.

Generally, ISA defines:
- Supported *data types* (8-bit, 16-bit, 32-bit, word, qword, etc.)
- Available state, e.g. main memory and registers
- State semantics, e.g. memory consistency and addressing modes
- Instruction set (IS) machine instructions comprising comp's machine lang (ML)
- IO model


## Overview

An ISA is distinguished from a microarchitecture; CPUs with different microarchitectures can still share a common instruction set.

The concept of an architecture is distinct from the design of a specific machine.

**Microarchitecture** is the set of processor design techniques used in a particular processor to implement the instruction set.


Some virtual machines (VM) that support _bytecode_ as their ISA (Smalltalk, JVM, CLR) implement this by translating the bytecode for commonly used code paths into *native machine code*. In addition, these VMs execute less frequently used code paths by interpretation.


## Classification

An ISA may be classified in a number of different ways. A common classification is by architectural complexity. A complex instruction set computer (CISC) has many specialized instructions, some of which may only be rarely used in practical programs. A reduced instruction set computer (RISC) simplifies the processor by efficiently implementing only the instructions that are frequently used in programs, while the less common operations are implemented as subroutines, having their resulting additional processor execution time offset by infrequent use.[2]

Other types include very long instruction word (VLIW) architectures, and the closely related long instruction word (LIW) and explicitly parallel instruction computing (EPIC) architectures. These architectures seek to exploit instruction-level parallelism with less hardware than RISC and CISC by making the compiler responsible for instruction issue and scheduling.

Architectures with even less complexity have been studied, such as the minimal instruction set computer (MISC) and one instruction set computer (OISC). These are theoretically important types, but have not been commercialized.