# Loader


## Loaders
A **loader** is the part of OS that reads the contents of an executable file from the secondary storage into the main memory, preparing it for execution. OS then runs the program by passing control to the loaded program's code segment.


### Unix
The loader is the handler for syscall `execve()` that performs:
- *validation* by checking permissions, memory requirements, etc.
- *reading* the program image into main memory
- *placing* potential command-line arguments on the stack
- *initializing registers*, e.g. stack base pointer, stack frame pointer
- *jumping* to the program entry point, usually labeled `_start`


### Windows
The loader is the handler for `LdrInitializeThunk` function (from `ntdll.dll`) that performs:
- *initializes* DLL structures, i.e. critical sections, module lists
- *validates* executable about to load
- *creates heap* via `RtlCreateHeap` function
- *allocates* environment variables block
- *appends* the executable and the `NTDLL` to the module list
- *reads* `KERNEL32.DLL` to get needed functions, e.g. `BaseThreadInitThunk`
- *loads* exe's imports recursively, i.e. required DLLs
- *raises* the system's breakpoints (if in debug mode)
- *initializes* DLLs
- *collects garbage*
- *calls* `NtContinue` to begin running the executable



## Dynamic linking loaders

**Dynamic linker** is the part of OS that loads and links external shared libraries (`.so`, `.dll`), needed by an executable during run-time, by copying their content into RAM and then binds those shared libraries dynamically to the running process (filling jump tables, relocating pointers). This approach is also called **dynamic linking** or **late linking**.

The specific OS and executable format determine the dynamic linker's functions and implementation.

### Windows
Dynamic-link library (DLL) is Microsoft's implementation of the shared library concept in the Microsoft Windows. The file format for DLL (`.dll`, `.ocx`) are the same as for Windows `.exe` files i.e. Portable Executable (PE). As with EXEs, DLLs can contain code, data, and resources, in any combination.

### Unix
In most Unix-like systems, most of the machine code that makes up the dynamic linker is actually an external executable that the kernel loads and executes; when an executable file is loaded, the kernel reads the path of the dynamic linker from it and then attempts to load and execute this binary. The pathname of the dynamic linker is part of the OS's application binary interface (ABI). 
At link time, the path of the dynamic linker that should be used is embedded into the executable image.

### Linux
In Unix-like systems that use ELF format for executables and dynamic libraries (`.so`), the path of the dynamic linker that should be used is embedded at link-time into the executable. The dynamic linker can be influenced (during the program's linking or execution) by altering `LD_LIBRARY_PATH` and `LD_PRELOAD` environment variables, which provide the paths to the shared libraries. For example, by changing the path of `LD_PRELOAD`, the `zlibc` (`uncompress.so`) shared library can be loaded, providing transparent decompression; this would allow reading of pre-compressed (gzipped) file data on BSD and Linux systems as if the files were not compressed, essentially allowing a user to add transparent compression to the underlying filesystem, although with some caveats.
