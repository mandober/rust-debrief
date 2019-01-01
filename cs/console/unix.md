# UNIX Architecture

In a strict sense, an **operating system** can be defined as the software that controls the hardware resources of the computer and provides an environment under which programs can run. Generally, we call this software the **kernel**, since it is relatively small and resides at the core of the environment. 

The interface to the kernel is a layer of software called the **system calls**. **Libraries routines** are common functions built on top of the system call interface, but applications are free to use both. The **shell** is a special application that provides an interface for running other applications.

In a broad sense, an operating system consists of the kernel and all the other software that makes a computer useful and gives the computer its personality. This other software includes system utilities, applications, shells, libraries of common functions, etc. For example, Linux is the kernel used by the GNU/Linux operating system.

A shell is a command-line interpreter that reads user input and executes commands. The user input to a shell is normally from the terminal (an interactive shell) or sometimes from a file (called a shell script).

The UNIX **file system** is a hierarchical arrangement of directories and files. Everything starts in the directory called **root**, whose name is the single character `/`.

A **directory** is a file that contains **directory entries**. Logically, we can think of each directory entry as containing a filename along with a structure of information describing the attributes of the file.

The **attributes** of a file are such things as the type of file (regular file, directory), the size of the file, the owner of the file, permissions and when the file was last modified.

The names in a directory are called **filenames**. The only two characters that cannot appear in a filename are the slash character (/) and the null character. The slash separates the filenames that form a pathname and the null character terminates a pathname. Two filenames are automatically created whenever a new directory is created: . (called dot) and .. (called dot-dot). Dot refers to the current directory, and dot-dot refers to the parent directory. In the root directory, dot-dot is the same as dot.

A sequence of one or more filenames, separated by slashes and optionally starting with a slash, forms a **pathname**. A pathname that begins with a slash is called an absolute pathname; otherwise, it’s called a relative pathname. Relative pathnames refer to files relative to the current directory. The name for the root of the file system (/) is a special-case absolute pathname that has no filename component.

Every process has a **working directory**, sometimes called the current working directory (cwd). This is the directory from which all relative pathnames are interpreted. A process can change its working directory with the chdir function.

When we log in, the working directory is set to our **home directory**. Our home directory is obtained from our entry in the password file

**File descriptors** (FD) are normally small non-negative integers that the kernel uses to identify the files accessed by a process. Whenever it opens an existing file or creates a new file, the kernel returns a file descriptor that we use when we want to read or write the file. By convention, all shells open 3 descriptors whenever a new program is run: standard input (stdin), standard output (stdout) and standard error (stderr).


**Unbuffered I/O** is provided by the functions `open`, `read`, `write`, `lseek` and `close`. These functions all work with file descriptors.

The **standard I/O functions** provide a buffered interface to the unbuffered I/O functions.

A **program** is an executable file residing on disk in a directory. A program is read into memory and is executed by the kernel as a result of one of the 7 `exec` functions.

An executing instance of a program is called a **process**. Some operating systems use the term **task** to refer to a program that is being executed.

The UNIX System guarantees that every process has a unique numeric identifier called the **process ID**. The process ID is always a non-negative integer.

There are 3 primary functions for **process control**: `fork`, `exec` and `waitpid`.


The `exec` family of system calls has 7 variants:
- `execve()`
- `execvp()`


Each variant expects slightly different protocol.
- variable number of string arguments
- list of string arguments
- the environment within the process runs


`execvp()`
This variant expects a program name and a vector (`v` for vector) of string arguments with the first being the name of a program; that program is searched for by consulting the `PATH` environment variable (`p` for PATH).



When a computer boots, the kernel is loaded and initialized, then it starts a process called `init`, which stays resident. Init's job is to manage other processes.

To start a new process, an existing process (parent process) is duplicated via the `fork` system call, and the newly created copy of the process (called child) is assigned a PID (process ID) in the form of a positive integer.

The `fork` returns the child's PID to the parent process, or a negative integer to indicate an error; it also returns 0 to the child process to indicate successful fork completion.

Only then a new process can start; the `exec` system call stops the running process and replaces it by loading a new one in its place. The process invoking `exec` never returns after that call.

So to run a new process, an existing process splits in two, parent and child processes. The child then calls `exec` and a new process replace it. The parent process is then able to keep track of its child processes with the `wait` system call.
