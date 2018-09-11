# x86 CPU modes

## x86 CPUs

Intel 8086
- in 1978, first `x86` CPU
- operating in real mode
- 16-bit linear address space
- 20-bit segmented memory address space
- 1 MB (2^20 Bytes) addressable memory

Intel 80186
- in 1982, `186`
- 16-bit linear address space
- 20-bit segmented memory address space

Intel 80286
- in 1982, `286`
- introducing protected mode
- 16-bit linear address space
- 14/16 bit segment/offset size
- 24-bit segmented memory address space

Intel 80386
- in 1985, `386`
- 32-bit linear address space
- 14/32 bit segment/offset size
- 32-bit segmented memory address space


## CPUs modes

Real mode
- 16-bit linear address space
- 20-bit segmented memory address space
- 1 MB (2^20 Bytes) addressable memory
- no support for memory protection, multitasking, or code privilege levels
- all x86 compatible CPUs start in real mode when reset

Protected mode
- Intel 80286 in 1982, `286`
- 16-bit linear address space
- 14-bit/16-bit segment/offset size
- 24-bit segmented memory address space
- virtual memory, paging and safe multi-tasking designed

Unreal mode
- Intel 80286

Long mode
- x86_64