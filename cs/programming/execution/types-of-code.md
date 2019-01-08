# Types of code

* Program code
* Source code
* Object code
* Bytecode
* Machine code
* Microcode


**Program code** (computer code) is a set of instructions comprising a program which is executed by a computer.

**Machine code** are instructions that a computer can execute directly - a rather lengthy sequence of 1's and 0's, where a group of bits is directly translated into the specific action, selected out of the predefined set of available hardware actions, called the **instruction set**. For example, a certain bit pattern may result in a circuit being closed letting the electric current flow, thus transfering a binary digit from one place to another (e.g. from a register to the ALU).

Every CPU has an instruction set - a built in, predifined set of actions a CPU can perform, which include basic arithmetic operations (add, sub, mul, div), logical operations (and, or, xor, not), loading and storing (mov, ret, )


all CPUs have the instruction that brings them to a halt, which programmers might refer to as `HALT` (or `HLT` or similar) when coding in an assembly language, which is translated into an binary number. The binary number an operation code is translated to is arbitrary - it is set by the manufacturer or, more likely, by the implemented standard.



, e.g. `0x0`.

These instructions take the form of an arbitrary encoded binary numbers.

Machine code is encoding
binary arbitrary


Because these instructions are difficult for humans to read, and writing good programs in machine code or other low-level programming languages is a time-consuming task, most programmers write in the source code of a high-level programming language.

This source code is translated into machine code by either a *compiler* or an *interpreter* so that the computer can execute it to perform its tasks.

A compiler produces *object code* which is usually in machine language but may also be in an *intermediate language* which is at a lower level than the source.

*Bytecode* is a lower level of source which is designed for more efficient interpretation by interpreters.


## Source code
Source code is any collection of code, possibly with comments, authored using a human-readable programming language, usually as plain text.

The source code of a program is specially designed to facilitate the work of computer programmers, who specify the actions to be performed by a computer mostly by writing source code.

The source code is often transformed by an *assembler* or compiler into binary machine code understood by the computer.

In the open source projects, the source code is readily available.

...



# Machine code
Machine code is a computer program written in machine language instructions that can be executed directly by a computer's central processing unit (CPU).

Each instruction causes the CPU to perform a very specific task (load, jump, add, compare) on a unit of data.

Machine code is a strictly numerical language which is intended to run as fast as possible, and may be regarded as the lowest-level representation of a compiled or assembled computer program or as a primitive and hardware-dependent programming language.

While it is possible to write programs directly in machine code, it is tedious and error prone to manage individual bits and calculate numerical addresses and constants manually. For this reason, programs are very rarely written directly in machine code in modern contexts, but may be done for low level debugging, program patching, and assembly language disassembly.

The overwhelming majority of practical programs today are written in higher-level languages or assembly language. The source code is then translated to executable machine code by utilities such as compilers, assemblers and linkers.

Interpreted programs, on the other hand, are not translated into machine code, but the interpreter itself (which may be seen as an executor or processor) performs the instructions of the source code. Interpreter typically consists of directly executable machine code (generated from assembly or high-level language source code).


Internally a CPU may use *microcode* or optimise and transform machine code instructions into sequences of micro operations (but this is not generally considered to be a machine code per se).



---

- [Computer code](https://www.wikiwand.com/en/Computer_code)
- [Source code](https://www.wikiwand.com/en/Source_code)
- [Object code](https://www.wikiwand.com/en/Object_code)
- [Bytecode](https://www.wikiwand.com/en/Bytecode)
- [Machine code](https://www.wikiwand.com/en/undefined)
- [Microcode](https://www.wikiwand.com/en/Microcode)
