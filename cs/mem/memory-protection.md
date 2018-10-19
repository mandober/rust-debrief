# Memory protection

Memory protection is about controlling memory access, which is an important task of the modern OS and ISA. It prevents a process from accessing memory outside its assigned address space. It isolates processes from each other and the OS itself.

An attempt to access unowned memory results in a hardware fault (segmentation fault or storage violation exception) generally causing abnormal termination of the offending process.

With regards to security, memory protection encompases techniques like NX bit, executable space protection, address space layout randomization (ASLR),



Segmentation refers to dividing a computer's memory into segments. A reference to a memory location includes a value that identifies a segment and an offset within that segment.