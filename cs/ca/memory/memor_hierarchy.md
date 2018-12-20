# Memory hierarchy

The memory hierarchy separates storage into a hierarchy based on response time.

Memory is organized in hierarchy, from the fastest (CPU registers), across the medium (RAM) to the slowest (SDD, HDD). Going from higher to lower hierarchy components, each descending layer is slower, but has a larger capacity. The storage, taking a practical approach, can be divided into 3 major categories: 
- Internal: CPU registers and cache (SRAM)
- Primary storage: main system memory (DRAM)
- Secondary storage (SDD, HDD)
Each category can further be divided into its own hierarchy based on response times or average access times (AAT), but considering only the above levels, there is a considerable difference (orders of magnitude) in AAT between layers.

One way to increase system performance is minimizing how far down the memory hierarchy CPU has to go to fetch data.



## Access time ratios

This example shows response times of the most important components of the memory hierarchy. The values are not precise, but they still demonstrate the approximate ratios.


ns 1990     | ns 2005    | ns 2018    | notes
-----------:|-----------:|-----------:|-----------------
 181        | 1          |        0.5 | L1 cache reference
 603        |          3 |          3 | Branch mispredict
 784        |          4 |          4 | L2 cache reference
 3017       |         17 |         17 | Mutex lock/unlock
 207        |        100 |        100 | RAM reference
 -          |    150,000 |    150,000 | 4K SSD Random access
 3,038,000  |    250,000 |    250,000 | 1MB seq. RAM read
 -          |  1,000,000 |  1,000,000 | 1MB seq. SSD read
 20,000,000 | 10,000,000 | 10,000,000 | HDD seek
640,000,000 | 20,000,000 | 20,000,000 | 1MB seq. HDD read



memory     |    speed |     notes
-----------|---------:|-----------------
CPU @ 3GHz | 1 cycle  | 0.3ns
registers  |          | 
L1 cache   | 3 cycles | 1ns (10% miss rate)
L2 cache   |          | 5ns (1% miss rate)
L3 cache   |          | 10ns (0.2% miss rate)
DRAM       |  10      | 50ns
Disk       |   2      | swap


fetch | cycles | time | notes
-------------------------------

```
L1 cache reference                           0.5 ns
Branch mispredict                            5   ns
L2 cache reference                           7   ns                      14x L1 cache
Mutex lock/unlock                           25   ns
Main memory reference                      100   ns                      20x L2 cache, 200x L1 cache
Read 4K randomly from SSD*             150,000   ns      150 us          ~1GB/sec SSD
Read 1MB seq. memory                   250,000   ns      250 us
Round trip within same datacenter      500,000   ns      500 us
Read 1 MB sequentially from SSD*     1,000,000   ns    1,000 us    1 ms  ~1GB/sec SSD, 4X memory
Disk seek                           10,000,000   ns   10,000 us   10 ms  20x datacenter roundtrip
Read 1 MB sequentially from disk    20,000,000   ns   20,000 us   20 ms  80x memory, 20X SSD
```

Notes:
* 1 ns = 10^-9 seconds
* 1 us = 10^-6 seconds = 1,000 ns
* 1 ms = 10^-3 seconds = 1,000 us = 1,000,000 ns




The CPU works in cycles dictated by a clock, and at 3 GHz, 1 cycle takes 0.3ns. It takes one clock cycle for CPU to execute a simple instruction. A light beam travels approximately 10cm during that time. Electronic signals, depending a many factors, travel through circuitry at 50–99% of the speed of light.

- L1 Cache: 3 cycles, ~1ns
- L2 Cache: 14 cycles, ~4.7ns
- RAM: 250 cycles, ~83ns
- HDD: 40 million cycles

Caches use SRAM (Static RAM) which is faster and more expensive than DRAM.

13.7ms avg read seek time
15ms avg write seek time
80ms latency ( 1MB/s download rate)


## Speed comparison
- 10ms launched tongue of the salamander
- 300ms blink of an eye
- reflexes travel at 250 mph
- thinking signals occur at 70 mph
- pain signals travel at a low-priority speed of just 3 mph
- 3-4 mph heart rate average, walking speed
- Median human reaction time (to some stimulus showing up on a screen): 270 ms



## Popular analogies

To put this into perspective:
- L1 cache is like grabbing a piece of paper from the desk (3 seconds)
- L2 cache is like picking up a book from a nearby shelf (14 seconds)
- RAM is like taking a 4-minute bathroom break
- HDD is like leaving the building to roam the earth for 1 year and 3 months.

L1 - There is a sandwich in front of you.
L2 - Walk to the kitchen and make a sandwich
RAM - Drive to the store, purchase sandwich fixings, drive home and make sandwich
HD - Drive to the store. Purchase seeds. Grow seeds..... .... ... Harvest lettuce, wheat, etc. Make sandwich.

L1 - The files are on your desk.
L2 - The files are in your filing cabinet.
RAM - The files are in a cabinet 10 floors below you. Take the stairs.
HD - The files are on the other side of the continent. Take the wheelbarrow.





## References
- https://www.wikipedia.com/en/Memory_hierarchy
- https://developers.redhat.com/blog/2016/03/01/reducing-memory-access-times-with-caches/
- http://duartes.org/gustavo/blog/post/what-your-computer-does-while-you-wait/
- http://norvig.com/21-days.html#answers
- Latency: 
  - Latency through the years: http://people.eecs.berkeley.edu/~rcs/research/interactive_latency.html
  - https://prezi.com/pdkvgys-r0y6/latency-numbers-for-programmers-web-development/
  - https://dzone.com/articles/every-programmer-should-know
  - https://gist.github.com/hellerbarde/2843375
  - https://gist.github.com/jboner/2841832
  - https://gist.github.com/dakull/2842457
  - https://gist.github.com/jhclark/2845836
