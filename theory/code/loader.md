# Loader

A loader is the part of an OS that is responsible for loading programs and libraries. It is one of the essential stages in the process of starting a program, as it places programs into memory and prepares them for execution. Loading a program involves reading the contents of the executable file containing the program instructions into memory, and then carrying out other required preparatory tasks to prepare the executable for running. Once loading is complete, the OS starts the program by passing control to the loaded program code. Embedded systems typically do not have loaders, and instead the code executes directly from ROM.


## Unix
In Unix, the loader is the handler for the system call `execve()`.
The Unix loader's tasks include:
- validation (permissions, memory requirements etc.)
- copying the program image from the disk into main memory
- copying the command-line arguments on the stack
- initializing registers (e.g. the stack pointer)
- jumping to the program entry point (`_start`)


## Windows
In Windows, the loader is the `LdrInitializeThunk` function contained in `ntdll.dll`, whose tasks include:
- initialization of structures in the DLL itself (i.e. critical sections, module lists)
- validation of executable to load
- creation of a heap (via the function `RtlCreateHeap`)
- allocation of environment variable block and `PATH` block
- addition of executable and NTDLL to the module list (a doubly-linked list)
- loading of `KERNEL32.DLL` to obtain several important functions, for instance `BaseThreadInitThunk`
- loading of executable's imports (i.e. dynamic-link libraries) recursively
- in debug mode, it raises the system's breakpoints
- initialization of DLLs
- garbage collection
- calling `NtContinue` on the context parameter given to the loader function (i.e. jumping to `RtlUserThreadStart`, that will start the executable)


## Dynamic linking loaders
Dynamic linker is the part of OS that loads and links external shared libraries (`.so`, `.dll`), needed by an executable during run-time, by copying their content into RAM and then binds those shared libraries dynamically to the running process (filling jump tables, relocating pointers). This approach is also called dynamic linking or late linking. The specific OS and executable format determine the dynamic linker's functions and implementation.

## Windows
Dynamic-link library (DLL) is Microsoft's implementation of the shared library concept in the Microsoft Windows. The file format for DLL (`.dll`, `.ocx`) are the same as for Windows `.exe` files i.e. Portable Executable (PE). As with EXEs, DLLs can contain code, data, and resources, in any combination.

## Unix
In most Unix-like systems, most of the machine code that makes up the dynamic linker is actually an external executable that the kernel loads and executes; when an executable file is loaded, the kernel reads the path of the dynamic linker from it and then attempts to load and execute this binary. The pathname of the dynamic linker is part of the OS's application binary interface (ABI). 
At link time, the path of the dynamic linker that should be used is embedded into the executable image.

## Linux
In Unix-like systems that use ELF format for executables and dynamic libraries (`.so`), the path of the dynamic linker that should be used is embedded at link-time into the executable. The dynamic linker can be influenced (during the program's linking or execution) by altering `LD_LIBRARY_PATH` and `LD_PRELOAD` environment variables, which provide the paths to the shared libraries. For example, by changing the path of `LD_PRELOAD`, the `zlibc` (`uncompress.so`) shared library can be loaded, providing transparent decompression; this would allow reading of pre-compressed (gzipped) file data on BSD and Linux systems as if the files were not compressed, essentially allowing a user to add transparent compression to the underlying filesystem, although with some caveats.
