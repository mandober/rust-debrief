# Registers

The registers usually form the groups:
- General purpose registers (GPR)
  - Data registers
  - Pointer registers
  - Index registers
- Control registers
- Segments (.bss, .data, .text)

There are ten 32-bit and six 16-bit processor registers in IA-32 architecture.


## Data Registers
The four 32-bit data registers are used for arithmetic, logical, and other operations.

These 32-bit registers can be used in 3 ways:
- 4*32b: E{A,B,C,D}X as complete 32-bit regs
- 4*16b : {A,B,C,D}X lower halves can be used as 16-bit regs
- 8* 8b : {A,B,C,D}{H,L} lower and higher halves of the 16-bit as 8 8-bit regs

`A`ccumulator: IO ops, arithmetic  
`B`ase: indexed addressing  
`C`ount: loop counter  
`D`ata: IO ops  

```
RAX───────┬EAX──────┬───AX────┐
│         │         │ AH │ AL │ x86_64
└─────────└─────────┴────┴────┘
FF       1F         F    7    0
```

## Pointer Registers

EIP, ESP, EBP: 32-bit and corresponding 16-bit lower portions IP, SP, BP.

- __IP: Instruction Pointer__  
The 16-bit IP register stores the offset address of the next instruction to be executed. IP in association with the CS register (as CS:IP) gives the complete address of the current instruction in the code segment.
- __SP: Stack Pointer__   
The 16-bit SP register provides the offset value within the program stack. SP in association with the SS register (SS:SP) refers to be current position of data or address within the program stack.
- __BP: Base Pointer__   
The 16-bit BP register mainly helps in referencing the parameter variables passed to a subroutine. The address in SS register is combined with the offset in BP to get the location of the parameter. BP can also be combined with DI and SI as base register for special addressing.


## Index Registers

ESI and EDI: 32-bit index registers, and their 16-bit lower portions, SI and DI.
Used for indexed addressing and sometimes in addition and subtraction.  
There are two sets of index pointers:
- Source Index (SI) used as source index for string operations.
- Destination Index (DI) used as destination index for string operations.


## Control Registers
The 32-bit instruction pointer register and the 32-bit flags register combined are considered as the control registers.

Many instructions involve comparisons and mathematical calculations and change the status of the flags and some other conditional instructions test the value of these status flags to take the control flow to other location.

The common flag bits are: `ZODIACPTS`
- `O`verflow Flag (OF) indicates the overflow after a signed arithmetic op.
- `D`irection Flag (DF) determines direction for moving or comparing string data. When the DF value is 0, the string operation takes left-to-right direction and when the value is set to 1, the string operation takes right-to-left direction.
- `I`nterrupt Flag (IF) determines whether the external interrupts (keyboard entry) are to be ignored or processed. It disables the external interrupt when the value is 0 and enables interrupts when set to 1.
- `T`rap Flag (TF) allows setting the operation of the CPU in single-step mode. 
- `S`ign Flag (SF) shows the sign of the result of an arithmetic operation. This flag is set according to the sign of a data item following the arithmetic operation. The sign is indicated by the high-order of leftmost bit. A positive result clears the value of SF to 0 and negative result sets it to 1.
- `Z`ero Flag (ZF) indicates the result of an arithmetic or comparison operation. A nonzero result clears the zero flag to 0, and a zero result sets it to 1.
- `A`uxiliary Carry Flag (AF) contains the carry from bit 3 to bit 4 following an arithmetic operation; used for specialized arithmetic. The AF is set when a 1-byte arithmetic operation causes a carry from bit 3 into bit 4.
- `P`arity Flag (PF) indicates the total number of set bits (1) in the result of an arithmetic operation. An even number of 1-bits clears the parity flag and an odd number sets it to 1.
- `C`arry Flag (CF) contains the carry of 0 or 1 from a MSB bit after an arithmetic operation. It also stores the contents of last bit of a shift or rotate operation.


## Segment Registers
Segments are specific areas defined in a program for containing data, code and stack.

There are 3 main segments:
- Code Segment contains all the instructions to be executed. A 16-bit Code Segment register (CS) stores the starting address of the code segment.
- Data Segment contains data, constants and work areas. A 16-bit Data Segment register (DS) stores the starting address of the data segment.
- Stack Segment contains data and return addresses of (sub)procedures. The Stack Segment register (SS) stores the starting address of the stack.

There are other extra segment registers: ES (extra segment), FS and GS, which provide additional segments for storing data.

All memory locations within a segment are relative to the starting address of the segment. A segment begins in an address evenly divisible by 0x10. So, the rightmost hex digit in all such memory addresses is 0, which is not generally stored in the segment registers.

The segment registers stores the starting addresses of a segment. To get the exact location of data or instruction within a segment, an offset value (displacement) is required. To reference any memory location in a segment, the processor combines the segment address in the segment register with the offset value of the location.
