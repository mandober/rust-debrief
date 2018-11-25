# Librarly Files


## LIB
The `.lib` files are library files and they are used for __static linking__. That is, they contain precompiled code that other applications can link to.  When you link your application with a .lib library, then the .lib library's code is intermixed with your own code in the final executable.

## DLL
The `.dll` files are dynamically linked library files and applications link to them at runtime.

DLL - Dynamically Linked Libary. 

Library:
It is a library of callable functions.

Dynamically Linked:
Because the actual code of DLL (that contains functions), is going to remain stored inside of the dll.
Instead of merging the code from dll with an executable, the executable will contain references to the actual executable code pieces inside of dll, and Windows will resolve ("link") your application's calls with the .dll file's subroutines dynamically (at run time).

So at the end of the day, a .dll file just contains a bunch of CODE.
The difference comes in in HOW the Windows operating system manages that code.

