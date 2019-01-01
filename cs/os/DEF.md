# Definitions


**Loader** is the part of OS that reads the contents of an executable file from the secondary storage into the main memory, preparing it for execution. OS then runs the program by passing control to the loaded program's code segment. Loader *validates* the executable, *allocates* required memory, *initializes* heap and relevant structures, *reads program image* into memory, *obtains* functions from system libraries, initializes *registers*, places program's *arguments on stack*, performs *garbage collection*, and finally *jumps* to the program's *entry point*, thereby *transfering control* to program's *code segment*.

**Dynamic linker** is the part of OS that loads and links external shared libraries (`.so`, `.dll`), needed by an executable during run-time, by reading their content into memory and binding them dynamically to the running process. This is also called **dynamic linking** or **late linking**.


**Race condition** is the undesireble behavior of a system where the output is dependent on a non-deterministic event. Race conditions can manifest themselves as unintended computing effects.

**Synchronization** is a strategy employed to control processes or threads, in order to ensure their collaboration and fair distribution of system resources. In order to avoid race conditions, a synchronization strategy is employed to deal with fair distribution of resources. A poorly designed strategy may allow for emergance of deadlocks.

**Deadlock** is a state in which threads are waiting on each other for some action to happend, such as to release a lock over a shared resource. Deadlock is a common problem in parallel computing and distributed systems, where software and hardware locks are used to arbitrate shared resources and implement process synchronization. In an OS, a deadlock occurs when a process or thread enters a waiting state because a requested system resource is held by another waiting process, which in turn is waiting for realse of some other resource that is held by another waiting process.

**Scheduling** is the method by which work is assigned to agents to perform it. The work may be virtual computation elements such as threads, processes or data flows, which are in turn scheduled onto hardware resources such as processors, network links or expansion cards.
