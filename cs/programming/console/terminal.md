# Terminal

## Terminal
Initially, terminal was a hardware serial device for text-based interaction with a computer.

Instruction given to perform a task are called commands. Modern computers use **terminal emulators** such as Unix shell, BASH shell, command prompt.

In some operating systems, including Unix, a **pseudoterminal**, **pseudotty**, or **PTY** is a pair of pseudo-devices, one of which, the slave, emulates a hardware text terminal device, the other of which, the master, provides the means by which a terminal emulator process controls the slave.

A **command-line interface** (CLI) or command language interpreter, also known as command-line user interface, console user interface and character user interface (CUI), is a means of interacting with a computer program where the client (user) issues commands to the program in the form of successive lines of text (command lines). A program which handles the interface is called a command language interpreter or shell.

A **line discipline** (LDISC) is a layer in the terminal subsystem in some Unix-like systems. The terminal subsystem consists of 3 layers:
1. the upper layer to provide the character device interface
2. the lower hardware driver to communicate with the hardware or pseudo terminal
3. the middle line discipline to implement behavior common to terminal devices.


**Teletypes** were ASCII-based teleprinters connected in a network called Telex, which was used for transferring telegrams. When the command line eventually replaced the old batch processing model as the interface to a computer, teletypes, being widely available, were used as IO devices.

Numerous teletype models were mutualy incompatible, so the Unix solution was to let the kernel handle all low-level details (word length, baud rate, flow control, parity, control codes). Coloured output and other advanced features made possible in the late 1970s by solid state video terminals (e.g. VT-100) were left to the applications.

Today, software terminal emulators have replaced physical teletypes.


**Console application** is a program that interacts with users via text-based I/O. It lacks the GUI (graphical user interface) and instead uses text user interface (TUI) that works with text I/O streams.
