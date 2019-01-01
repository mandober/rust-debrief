# Process

**Process** is an instance of the running program, containing the program code and maintaining its state.

**Thread** (of execution) is the smallest sequence of programmed instructions that can be managed independently by a scheduler, which is typically a part of the OS. A process always has the main thread, but it can spawn additional threads.


A Linux process can be in one of the following states:
- R: Running or runnable (on run queue)
- D: Uninterruptible sleep (waiting for some event)
- S: Interruptible sleep (waiting for some event or signal)
- T: Stopped, by signal or debugger
- Z: Zombie or orphan process; terminated but not yet reaped by its parent.

There are two sleeping states: Interruptible and uninterruptible sleep. The former is the common case and it means that while the process is part of a wait queue and it may be moved to the running state upon receiving a signal.

The `ps` utility list processes:
`s` process is the *session leader*
`+` process is part of the *foreground process group*

**Job control** is what happens when `^Z` is press to suspend a program, or when the program is run in the background with `&`. A **job** refers to a *process group*.

Builtin shell commands such as `jobs`, `fg`, `bg` are used to manage jobs within a session. Each session is managed by a *session leader*, commonly the shell itself, that colaborates with the kernel through signals and system calls.

Only the foreground job is allowed to write to and to read from the TTY device. If a background process attempts to write to the TTY, the kernel would suspend it by issuing a signal.

In Unix, the kernel communicates with processes asynchronously by issuing signals. Processes may intercept, then even ignore, a subset of signals, but there is also a subset of enforcing signals whose effect is unavoidable.
