# Memory hierarchy

One of the main ways to increase performance is minimizing how far down the memory hierarchy one has to go to manipulate data.

Intel Haswell Mobile, 2013:

storage    | ~size  | ~ GB/s | notes
-----------|-------:|-------:|-----------------
registers  | few KB |1 cycle | 0.3ns @ 3GHz
L0 cache   | 6 KB   |        | micro operations
L1 cache   | 128 KB |3 cycles| instructions
L1 cache   | 128 KB | 700    | data
L2 cache   | 1 MB   | 200    | shared data & instructions
L3 cache   | 6 MB   | 100    | shared
L4 cache   | 128 MB | 40     | shared
DRAM       |        | 10     | stack, heap
Disk       |        | 2      | swap space

The third column (except the first row) shows ballpark values; these are not to be taken very precisely by themselves, only to give some kind of idea about relations between them.


# Relative speeds
(Intel Core 2 Duo @ 3GHz)

http://duartes.org/gustavo/blog/post/what-your-computer-does-while-you-wait/


- CPU Core: 1 cycle 0.3ns
Simple instructions take one clock cycle to execute. which is 1/3 of a nanosecond @ 3.0GHz. For reference, light travels approximately 10cm during that time.
- L1 Cache: 3 cycles, ~1ns  
- L2 Cache: 14 cycles, ~4.7ns
- RAM: 250 cycles, ~83ns
- HDD: 40 million cycles


Caches use SRAM, Static RAM, which is faster and more expensive than DRAM.


To put this into perspective, reading from:
- L1 cache is like grabbing a piece of paper from the desk (3 seconds)
- L2 cache is like picking up a book from a nearby shelf (14 seconds)
- RAM is like taking a 4-minute bathroom break
- HDD is like leaving the building to roam the earth for 1 year and 3 months.

13.7ms avg read seek time
15ms avg write seek time
80ms latency ( 1MB/s download rate)

- 10ms launched tongue of the salamander
- 300ms blink of an eye
- reflexes travel at 250 mph
- thinking signals occur at 70 mph
- pain signals travel at a low-priority speed of just 3 mph
- 3-4 mph heart rate average, walking speed
