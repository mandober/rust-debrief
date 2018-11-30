# Assembly languages

Assembly languages expose the computer architecture to the programmer, providing a convenient textual way for expressing programs for the machine while doing nothing to hide the actual behavior of the hardware.

Each assembly language statement typically corresponds to one machine language instruction; the difference is that the assembly language statements are written in convenient human-friendly form, while machine languages are expressed in (human-unfriendly) binary code - the only form a computer can consume.

The source code is a sequence of lines, each composed of a sequence of punctuation marks, numbers and strings. The processing that recognizes these structures is formally called **lexical analysis**. The symbols recognized at this level are called **lexemes**.

Lexical elements of the source code are classified into structures according to their function, which, in an assembly language, include labels, opcodes, operands, and possibly comments. This level of analysis is called **syntactic analysis** and is performed with respect to the **grammar** or **syntax** of a given language.

Each line of an assembly language program starts with the symbolic name of a machine instruction, called opcode (operation code).


- Constants: we simply incorporate the appropriate constants from the source code into the machine code, translating each to hexadecimal.
- Relative offsets: give the number of bytes ahead (if positive) or behind (if negative) the location immediately after the location that references the offset. Negative offsets are represented using 2's complement notation.

We cannot complete the translation without determining where the code will be placed in memory. Suppose, for example, that we place this code in memory starting at location 020016. This allows us to determine which byte goes in what memory location, and it allows us to assign values to the two labels IDENDL and ISOR, and thus, fill out the values of all of the 2-byte address fields to complete the translation.

```
symbol	address

START	 0200
LOOP	 0209
NOSUB	 0218
IDENDL	 0221
ISOR	 0222
```
the symbol table for this small example, sorted into numerical order. For a really large program, we might rewrite the table into alphabetical order to before using it to finish the assembly.

The programmer writing the line `STA IDENDL` must know that it means "store the value of the A register in the location labeled IDENDL", and the CPU, when it executes the corresponding binary instruction `8D 21 02` must know that this means "store the value of the A register in the location 0221".

```
                          Source code
                            /    \       compiler or
          programmer's     /      \      assembler's
        view of meaning   /        \  view of meaning
                         /          \
            Abstract Meaning ------ Machine Code

                          hardware's
                       view of meaning
```
Figure 1. Views of the meaning of a program.

BNF definition of the small assembly language:
```
<program> := <line> <EOF> 
          | <line> <program>

<line> := <definition> 
        | <statement> 
        | <comment>

<definition> := <identifier> = <operand> <comment>

<statement> := <label> <instruction> 
            | <instruction>

<instruction> := <opcode> <operand> <comment> 
              | <comment>

<comment> := ;<text> <EOL> 
            | <EOL>

<label> := <identifier> :

<opcode> := Byte | Word

<operand> := <identifier> 
          | <number>
```
- program is a sequence of lines
- line is either a definition, statement or comment
- definition is an identifier, followed by equals and operand
- label part of a statement is optional
- opcode; operand part of an instruction is optional
- comments at ends of lines are optional
- label is a symbol followed by a colon
- legal opcodes are `Byte` and `Word`
- operand is either an identifier or a number



## Examples

Notation:
- `%` denotes a register, e.g. `%ebp`
- `l` is for long i.e. 32 bits on x86, 64 bits on x86_64

Opcodes:
- `push` insert value to top of the stack; `ebp` points to top
- `pop`  remove top value off the stack; `ebp` points to top

Register sizes:
- format `ξρ`,  hold 16 bits (e.g.  ax,  bx,  sp,  bp, etc.)
- format `eξρ`, hold 32 bits (e.g. eax, ebx, esp, ebp, etc.)
- format `rξρ`, hold 65 bits (e.g. rax, rbx, rsp, rbp, etc.)

e: extended
r: 
ξ: abc letter, a-d
ρ: x/l, p


Registers:
- `esp` stack pointer (extended i.e. 32 bits)
- `ebp` base pointer (extended i.e. 32 bits)

Example:
- `pushl` and `popl` operate on the value that is on top of the stack
- `movl` opcode will operate on any value in the stack frame.
- a memory address to operate on is calculated as offset from the value (address) held by base pointer, `%ebp`.

Add two 4 bytes signed ints (long words in GCC)
same instructions whether signed or unsigned: 
  `int t = x + y;`
in assembly: 
  `addl 8(%ebp), %eax`
similar to: 
  `int x += y`: `int eax; int *ebp; eax += ebp[2];`


Operands:
- x in register: `%eax`
- y in memory (on the stack): `M[%ebp+8]`
- t in register: `%eax` (the result will be put in `eax` register)

`addl 8(%ebp), %eax`
First operand is in register `%eax`, but the second is in memory, in the current stack frame. Its memory address is relative to the stack base pointer, `ebp`; it is 8 bytes (2 longs) away from the address stored in the `%ebp`.



## Instructions

* Data transfer:
  - *Load* from mem to register: `%reg = mem[address]`
  - *Store* from register into mem: `mem[address] = %reg`
* Arithmetic
  - add, sub, on reg or mem data
* Control transfer
  - unconditinal jump to/from procedures
  - conditinal branches


## Moving data

IA32 registers:
- general purpose: %eax, %ecx, %edx, %ebx, %esi, %edi
- stack pointers: %esp (stack pointer), %ebp (stack base pointer)

syntax: `movX src, dest`, where X is: `b`, `w`, `l` i.e.
byte (8bits), word (2bytes, 16bits), long (4bytes, 32bits)
Little endian: bytes making the word and long are stored reversed, from LSB (least significant byte) to MSB.  
For example, a long, a hex number   
BF FF F6 D8 is stored as
D8 F6 FF BF

Operand types:
- Immediate: constants i.e. integers. 
  - Prefixed with $
  - Sensible only as src
  - examples: `movl $0xBF, %eax`, `movl $-42, %eax`
- Register: one of 8 registers
  - examples: `movl %eax, %edx`
  - esp and ebp are for stack ops
  - other have special uses for particular instructions
- Memory
  - movl moves 4 consecutive bytes of data at a memory address given by a register
  - many address modes
  - example address mode: `movl $6, (%eax)` move 6 to mem add pointed by eax;
    `movl $2, 4(%esp)` moves number 2 to a memory address given by esp (stack pointer) offsetted by 4 bytes (+4B).

* mem to mem needs more than 1 instruction (mem to reg, reg to mem)

