# Assembly

Many variants of assembly. This below is an example.

Notation:
- `%` denotes a register, e.g. `%ebp`
- `l` is for long i.e. 32 bits on x86, 64 bits on x86_64

Opcodes:
- `push` insert value to top of the stack; `ebp` points to top
- `pop`  remove top value off the stack; `ebp` points to top

θδγλξπσϕψωτρ

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

mem to mem needs more than 1 instruction (mem to reg, reg to mem)

