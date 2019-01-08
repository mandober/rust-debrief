# Concurrent Programming


## Current situation and concurrent programming

**Feature size plateau**       
Unlike the previous decades that saw dramatic increases of CPU frequency nearly every year, the end of 2010s had witnessed the limits of the production process. Semiconductor device fabrication is still managing to take the actual photolithographic process further, conquering production at 14 nanometers in 2017, scale down to 10nm and 7nm process in 2018, expected to reach 5nm in 2020. Miniaturization of production process allowed the increase in CPU frequency, but since 2007, due to, primarily, the problem of efficient heat dissipation, the CPU frequency has plateaued at, more-less, 4GHz mark.

**CPU frequency plateau**      
When further performance gains cannot rely on the increase in CPU frequency, the focus has shifted to horizontal expansion, characterized by the increase in the number of processor units per computer. Although this option was around for a long time, and the motherboards capable of housing two separate CPUs have been available on the market, it wasn't until the Intel developed the multi-core architecture that this technology became common.

**Free performance no more**      
From developers' perspective, better performance, gained by the higher operating speed of CPU, was entirely "free" - no change in their workflow was necessary to enjoy the benefits. On the other hand, if developers want to see improvements over the programs written for single-core computers, a considerable effort needs to be invested to properly understand and implement programming paradigms that fully utilize all available CPU cores through concurrency, multithreading and parallelism.

## Shared mutable state
Inadequate management of shared mutable state was long ago identified as the point that frequently breaks the programs. Variables, especially pointers, acting on the same mutable memory location, are particularly infamous considering the amount of the generated terms to describe their undesirable effects, that emerge when a pointer, unbeknown to others, modifies a critical value.

It was clear that there cannot be mutability in the presence of sharing, or, vice versa, a shared and mutable resource. Sharing alone, e.g. multiple readers of a memory location are fine. A single writer is also ok. Anything else, such as multiple writers, or a reader and writer together, are not.

Traditionally, programming languages have not done much to control safety issues such as this, allowed developers to do anything, including writing potentially unsafe code. Also, it has hard to implement safety features without significantly decreasing performance at the same time. Fortunately, modern languages, such as Rust, have been designed with the safety build-in to address these problems without sacrificing performance. In order to do that, Rust has employed some fairly new language concepts, introducing ownership and lifetimes, that might have seemed exotic to some developers, but, on the other hand, these language constructs were indeed lacking from programming languages, as they do help make programming safer. This predicament exists even before introducing concurrency; than the complexity increases further, demanding the addition of new language concepts to be properly managed.



Topics
  - FP approach to manage concurrency
  - FP: pure functions, immutability, laziness, composition
  - combining asynchronous ops with Task Parallel Library
  - problems and troubleshooting of multithreaded and asynchronous apps
  - concurrent programming models adopting the FP paradigm:
    - functional
    - asynchronous
    - event-driven
    - message passing with agents and actors
  - building high-performance, concurrent systems with FP
  - expressing and composing asynchronous computations in a declarative style
  - accelerating sequential programs with data-parallel programming
  - implementing reactive and event-based programs declaratively with Rx-style
  event streams
  - using functional concurrent collections for building lock-free multithreaded programs
  - writing scalable, performant and robust server-side applications
  - solving problems using concurrent programming patterns: Fork/Join, parallel aggregation, divide and conquer technique
  - processing massive data sets with parallel streams and parallel Map/Reduce
  implementations

Terminology
  - Sequential programming
  - Concurrent programming
  - Parallel programming
  - Multitasking
  - Multithreading



## Programming paradigms

**Sequential programming** refers to writing code that executes linearly, in a strict sequence of progressively ordered consecutive steps, one step at a time. The most defining characteristic of this programming model is that an operation always completes, i.e. *runs to completion*, before another starts.


## Concurrent computing

**Concurrent computing** is a manner of performing computation such that several operations are executed during overlapping time periods - they are executed concurrently, as opposed to being executed sequentially.

**Concurrency** is a property of a system, whatever the system may be: a program, computer, network; there is always a separate, main, thread that is controlling the execution of other computations, acting as a main point of execution. The most defining characteristic of this programming model is that a computation can advance without waiting for some other to finish.

As a paradigm, concurrent programming belongs to modular programming patterns, where a complex task is divided into smaller subtasks eligible for concurrent execution.

Pioneers in the field of concurrent computing include Edsger Dijkstra, Per Brinch Hansen, C. A. R. Hoare.



## Multitasking

Concurrency describes the ability to run several programs or multiple parts of a program at the same time.

Performing multiple operations at the same time is called *multitasking*.

In a "true" multitasking system, operations are indeed executed in parallel, and parallel execution requires appropriate hardware support, mainly a multi-core CPU.

However, the appearance of multitasking ("faux multitasking") can be achieved even on the older processors by employing a time-slicing technics, such as when a CPU allocates a small amount of time to each of the running processes, then quickly switches between them, giving opportunity to the next one to proceed with its execution. By quickly switching and putting queued processes in control, the CPU creates an illusion of multitasking, even though only one process is executing at any given moment.

~~
Single-core computers can perform multiple tasks in an interleaved manner, giving impression of true multitasking, but at any time only one thread is executing by the processor.

In programming, using concurrency allows for multitasking, achieved by dividing the program into multiple independent processes that run concurrently (at the same time) in different threads.

Using concurrency provides multitasking, achieved by dividing the program into multiple independent sections that can be processed at the same time (concurrently), in different threads, using separate cores.

Concurrency is, by definition, multithreading, but multithreading is not necessarily concurrent.

By splitting the task in multiple subtasks or by constantly juggling with different task, giving each task a certain amount of time, can appear as multitasking especially due to very fast context switching.
~~


This can happen in a single-core CPU or in parallel on a multi-core CPU.
The *throughput* (the rate at which the CPU processes a computation) and responsiveness of the program can be improved through *asynchronous* and/or *parallel* execution of a task.

Concurrency gives the impression that these threads are running in parallel and that different parts of the program can run simultaneously. But in a single-core environment, the execution of one thread is temporarily paused and switched to another thread.
Simultaneously performing several tasks requires additional resources. In computer programming, this process is called parallelism.

*Parallelism* is the concept of executing multiple tasks at once concurrently, literally at the same time on different cores, to improve the speed. Although all parallel programs are concurrent, not all concurrency is parallel.

That's because *parallelism* depends on the actual runtime environment, and it requires multiple cores. Parallelism is achievable only in multicore devices and is the means to increasing performance and throughput.

The goal of parallelism is to maximize the use of all available computational resources.

Parallelism can be achieved when a single task is split into multiple independent subtasks, which are then run using all the available cores.

A multicore machine allows parallelism for simultaneously executing different tasks without interruption.

The concept of timing is fundamental for simultaneously executing operations in parallel. In such a program, operations are concurrent if they can be executed in parallel, and these operations are parallel if the executions overlap in time.

Parallel computing is a type of computation in which many calculations are carried out simultaneously, operating on the principle that large problems can often be divided into smaller ones, which are then solved at the same time.

Parallelism and concurrency are related programming models. A parallel program is also concurrent, but a concurrent program isn't always parallel, with parallel programming being a subset of concurrent programming.

While concurrency refers to the design of the system, parallelism relates to the execution. Concurrent and parallel programming models are directly linked to the local hardware environment where they are performed.


*Multitasking* performs multiple tasks concurrently over time.

Multitasking is the concept of performing multiple tasks over a period of time by executing them concurrently.

Computer multitasking was designed in the days when computers had a single CPU to concurrently perform many tasks while sharing the same computing resources. Initially, only one task could be executed at a time through time slicing of the CPU.

*Time slicing* refers to a sophisticated scheduling logic that coordinates execution between multiple threads.

The amount of time the schedule allows a thread to run before scheduling a different thread is called *thread quantum*.

The CPU is time sliced so that each thread gets to perform one operation before the execution context is switched to another thread.

*Context switching* is a procedure handled by the OS to multitask for optimized performance, but in a single-core computer, multitasking can possibly degrade performance by introducing overhead as a consequence of context switching between threads.

Context switch in a single-core machine gives the illusion that multiple tasks run in parallel, but only one task is processed at a time.

Two kinds of multitasking OSs:
* **Cooperative** multitasking systems, where the scheduler lets each task run to completion or a thread explicitly yields execution control back to the scheduler
* **Preemptive** multitasking systems, where the scheduler prioritizes tasks, switching the execution context once the allocated time elapses and the control gets transferred to other tasks.

Most OSs, designed in the last decade, provide preemptive multitasking. Multitasking is beneficial for responsiveness of a system by avoiding *blocking* of the currently executing thread, e.g. freezing of UI during long operations.


**Multithreading** is an extension of the concept of multitasking, aiming to improve the performance of a program by maximizing and optimizing computer resources.

Multithreading is a form of concurrency that uses multiple threads of execution.

Multithreading implies concurrency, but concurrency doesn't necessarily imply multithreading.

Multithreading enables a program to explicitly subdivide specific tasks into individual threads that run in parallel within the same process.

A *process* is an instance of a program running within a computer system. Each process has one or more *threads* of execution, and no thread can exist outside a process

A **thread** is a unit of computation, an independent set of programming instructions designed to achieve a particular result, which the OS's scheduler independently executes and manages.

Multithreading differs from multitasking: unlike multitasking, with multithreading the threads share resources. But the resource sharing of multithreading presents more challenges to programmers than multitasking.

The concepts of parallel and multithreading programming are closely related. But in contrast to parallelism, multithreading is hardware-agnostic, which means that it can be performed regardless of the number of cores. Parallel programming is a superset of multithreading. You could use multithreading to parallelize a program by sharing resources in the same process, for example, but you could also parallelize a program by executing the computation in multiple processes or even in different computers.


SUMMARY
* Sequential programming refers to a set of ordered instructions executed one at a time on one CPU.
* Concurrent programming handles several operations at a time using either a single or a multi-core CPU.
* Parallel programming executes multiple operations at the same time on multiple CPUs. All parallel programs are concurrent, running simultaneously, but not all concurrency is parallel. The reason is that parallelism is achievable only on multicore devices.
* Multitasking concurrently performs multiple threads from different processes. Multitasking doesn't necessarily mean parallel execution, which is achieved only when using multiple CPUs.
* Multithreading extends the idea of multitasking; it's a form of concurrency that uses multiple, independent threads of execution from the same process. Each thread can run concurrently or in parallel, depending on the hardware support.



## The need for concurrency
Let’s consider a processor with N (as any number) running cores. In a single-threaded application, only one core runs. The same application executing multiple threads will be faster, and as the demand for performance grows, so too will the demand for N to grow, making parallel programs the standard programming model choice for the future.

Current hardware trends predict more cores instead of faster clock speeds; therefore, developers have no choice but to embrace this evolution and become parallel programmers.


## The pitfalls of concurrent programming
Using sequential programs, the execution of the code takes the happy path of predictability and determinism. Conversely, multithreaded programming requires commitment and effort to achieve correctness. Furthermore, reasoning about multiple executions running simultaneously is difficult because we’re used to thinking sequentially.

**Determinism** is a fundamental requirement in building software as computer programs are often expected to return identical results from one run to the next, but this property becomes hard to resolve in a parallel execution. External circumstances, such as the operating system scheduler or cache coherence, could influence the execution timing and, therefore, the order of access for two or more threads and modify the same memory location. This time variant could affect the outcome of the program.


A method is called *threadsafe* when the data and state don't get corrupted if two or more threads attempt to access and modify the data or state at the same time.

A *side effect* arises when a method changes some state from outside its scope, or it communicates with the "outside world", e.g manipulating a database or the file system. In the presence of side effects, the determinism of the computation is lost because the order in which concurrent tasks execute becomes variable. A solution is to avoid side effects in favor of *pure functions*.


When many threads continually access shared data, you must consider how to safeguard the data structure to guarantee its integrity.

A thread should write and modify a memory location *atomically*, without interference by other threads. An **atomic** operation accesses a shared memory and completes in a single step relative to other threads.

In practice, programs written in languages with mutable variables and/or imperative style, are always be vulnerable to data races.

A **data race** occurs when two or more threads of a process manipulate the same mutable variable, i.e. memory location, concurrently, giving rise to a *race condition*. If at least one of the accesses updates the variable's value while other threads read that value without using any memory protection mechanism (locks, mutex, etc).

The solution in imperative paradigm is to protect the mutable state by controlling the memory access using locks. A thread must first obtain a lock to memory location in order to be able to modify it. If another thread is already using (locking) that memory location, the thread must wait until the resource is unlocked. This technique is called *mutual exclusion* or *mutex* for short as it assures only one thread at the time has exclusive access to a memory location.

The introduction of locks to synchronize multi-threaded access to a shared resources solves the problem of data corruption, but introduces other problems such as *deadlock*, *livelock*, *thread starvation*, etc.

A **deadlock** is a state in which each member of a thread group is waiting for some other thread to take action, such as releasing a lock. A deadlock occurs when a thread enters a waiting state because a requested system resource is held by another waiting thread, which in turn is waiting for another resource held by yet another waiting thread. If a thread is unable to change its state indefinitely because the resources requested by it are being used by another waiting thread, then the system is *deadlocked*.
