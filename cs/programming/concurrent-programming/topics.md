Concurrency
https://www.wikiwand.com/en/Concurrency_(computer_science)

Concurrent computing
https://www.wikiwand.com/en/Concurrent_computing

Parallel computing
https://www.wikiwand.com/en/Parallel_computing

Multitasking
https://www.wikiwand.com/en/Computer_multitasking

Multiprocessing
https://www.wikiwand.com/en/Multiprocessing

Mutual exclusion
https://www.wikiwand.com/en/Mutual_exclusion


---

## Concurrency

Concurrency is a property of a system e.g. of an algorithm, a program, a computer, a network, etc.).

> Concurrency is the ability of a system where its different parts can be executed out-of-order or partially, without affecting the final result.

This allows for parallel execution of the concurrent parts, which can significantly improve overall execution speed, provided a multi-core computer.

In more technical terms, concurrency refers to the decomposability property of a program, algorithm, or problem into order-independent or partially-ordered components or parts.

A number of mathematical models have been developed for general concurrent computation including:
- Petri nets
- process calculi
- the parallel random-access machine model
- the actor model
- the Reo Coordination language

https://www.wikiwand.com/en/Petri_net
https://www.wikiwand.com/en/Process_calculi
https://www.wikiwand.com/en/Parallel_random-access_machine
https://www.wikiwand.com/en/Actor_model
https://www.wikiwand.com/en/Reo_Coordination_Language


## History
The fundamental figure and founder of concurrent computing was Edsger Dijkstra, a Dutch scientist, with his seminal 1965 paper that introduced the problem of *mutual exclusion*.

## Issues
Because computations in a concurrent system can interact with each other while being executed, the number of possible execution paths in the system can be extremely large, and the resulting outcome can be indeterminate.

Concurrent use of shared resources can cause indeterminacy, leading to issues such as deadlocks and resource starvation.

https://www.wikiwand.com/en/Deadlock
https://www.wikiwand.com/en/Resource_starvation

Design of concurrent systems often entails finding reliable techniques for coordinating their execution, data exchange, memory allocation and execution scheduling in order to minimize response time and maximize throughput.
