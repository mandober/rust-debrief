# Memory hierarchy

The memory hierarchy is a grouping of memory, based on the response times.
The faster components are located towards the top of the hierarchy, closer to the CPU. Going towards lower components, each descending layer is (orders of magnitude) slower with (orders of magnitude) larger capacity.



## Tiers
Practically, all storage can be divided into 3 tiers:
- Internal:
  - CPU registers
  - several levels of cache (SRAM):
    - L1 data instruction (L1i)
    - L1 data chache (L1d)
    - L2 cache
    - L3 cache
- Primary storage: main memory (DRAM)
- Secondary storage (SDD, HDD)

Each tier can further be divided into its own hierarchy based on response times, but this sub-hierarchy, in modern computers, is the mostly evident only in regards to cache, that is each cache level is somewhat slower (and has more capacity) then the previous one. It might also apply to secondary storage with regards to SSD and HDD. However, the gap between the tiers is still tremendously huge. The following table is an approximate illustration:

ns         | cycles | notes
----------:|-------:|-----------------
0.3        |      1 | CPU @ 3Ghz
1          |      3 | L1 cache reference
3          |      9 | Branch mispredict
4          |     12 | L2 cache reference
17         |     51 | Mutex lock/unlock
83         |    250 | DRAM reference
150,000    |        | 4K SSD Random access
250,000    |  1 mil | 1MB seq. DRAM read
1,000,000  |  4 mil | 1MB seq. SSD read
10,000,000 | 40 mil | HDD seek
20,000,000 | 80 mil | 1MB seq. HDD read

Note: 1s = 10^3 ms = 10^6 μm = 10^9 ns

These values may not precise, but they illustrate the speed ratios well enough. Multiplying the time (in the table above) by a billion, brings the time scale into human perspective, produced some popular analogies such as:
- accessing L1 is like grabbing a piece of paper from the desk (~3 sec)
- accessing L2 is like picking up a book from a nearby shelf (~12 sec)
- accessing RAM is like taking a coffee break (~4 min)
- accessing HDD takes more then a year in this analogy (~15 months)

It takes 1 clock cycle for CPU to execute a simple instruction, and at 3 GHz, 1 cycle is 0.3ns (a light beam travels approximately 10cm during that time). Electronic signals, depending a many factors, travel through circuitry at 50–99% of the speed of light.

A certain way to increase system performance is minimizing how far down the memory hierarchy one has to go to fetch the data.


## Types of RAM

One of the main resons for the huge discrepancy between access time and speed  of different RAM types are the manufacturing trade-offs. Today, the most dominant types of RAM are static and dynamic.

Static RAM (SRAM) has orders of magnitude faster access time in comparision with dynamic (DRAM), but the cell is bigger and SDRAM is significantly more expensive to produce. DRAM is cheap to produce and the cells can be stacked together in manageble enclosures.


## Cache

All levels of CPU caches are usually implemented as SRAM. The caches temporarily store (cache) data from RAM, so future requests by the CPU for that same data can be satisfied quickly. The caching is especially important because of the two properties a program instructions and data have: temporal and spatial locality.


This means that, over short periods of time, there is a good chance that the same instructions or data gets reused. For instructions this means that there are most likely loops in the code so that the same code gets executed over and over again (the perfect case for spatial locality).

Data accesses are also ideally limited to small regions. Even if the memory used over short time periods is not close together there is a high chance that the same data will be reused before long (temporal locality).

For code this means, for instance, that in a loop a function call is made and that function is located elsewhere in the address space. The function may be distant in memory, but calls to that function will be close in time.

For data it means that the total amount of memory used at one time (the working set size) is ideally limited but the memory used, as a result of the random access nature of RAM, is not close together.

Realizing that locality exists is key to the concept of CPU caches as we use them today.

-

All communtication (load and store) with main memory have to go through the cache.

The connection between the CPU core and the cache is a special, fast connection. In a simplified representation, the main memory and the cache are connected to the system bus which can also be used for communication with other components of the system.

-

In addition we have CPUs which have multiple cores and each core can have multiple threads.

The difference between a core and a thread is that separate cores have separate copies of all the hardware resources.

The cores can be completely independent unless they are using the same resources (e.g. the connections to the outside) at the same time.

On the other hand, threads share almost all of the CPU's resources.
