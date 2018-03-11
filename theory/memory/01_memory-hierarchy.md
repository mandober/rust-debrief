# Memory hierarchy


The memory hierarchy is a grouping of storage, based on the response times, with the fastest components located on the top of the hierarchy, close to the CPU. Going towards lower components, each descending layer is (orders of magnitude) slower with (orders of magnitude) larger capacity. In a practical approach all storage can be divided into 3 major tiers:
- Internal: CPU registers and several levels of cache (SRAM)
- Primary storage: main system memory (DRAM)
- Secondary storage (SDD, HDD)

Each tier can further be divided into its own hierarchy based on response times, and even though significant improvements are made, for example in the secondary storage with SSD pushing aside the traditional rotating disks, there is still a huge gap in terms of responsiveness and speed between adjacent memory tiers. One way to increase system performance is minimizing how far down the memory hierarchy one has to go to fetch the data.

 ns         | cycle | notes
-----------:|------:|-----------------
        0.3 |     1 | CPU @ 3Ghz
          1 |     3 | L1 cache reference
          3 |     9 | Branch mispredict
          4 |    12 | L2 cache reference
         17 |    51 | Mutex lock/unlock
         83 |   250 | RAM reference
    150,000 |       | 4K SSD Random access
    250,000 |       | 1MB seq. RAM read
  1,000,000 |       | 1MB seq. SSD read
 10,000,000 | 40mil | HDD seek
 20,000,000 |       | 1MB seq. HDD read

1s = 10^3ms = 10^6μm = 10^9ns

These values may not precise, but they illustrate the speed ratios between different layers in memory hierarchy. Multiplying these numbers by at least a billion, produced some popular analogies aimed to put things into perspective:
- L1 is like grabbing a piece of paper from the desk
- L2 is like picking up a book from a nearby shelf
- RAM is like taking a 4-minute break
- HDD is like roaming the earth for 15 months.

The CPU works in cycles dictated by a clock, and at 3 GHz, 1 cycle takes 0.3ns. It takes one clock cycle for CPU to execute a simple instruction. A light beam travels approximately 10cm during that time. Electronic signals, depending a many factors, travel through circuitry at 50–99% of the speed of light.




## References
- https://www.wikipedia.com/en/Memory_hierarchy
- http://duartes.org/gustavo/blog/post/what-your-computer-does-while-you-wait/
- http://people.eecs.berkeley.edu/~rcs/research/interactive_latency.html
- https://developers.redhat.com/blog/2016/03/01/reducing-memory-access-times-with-caches/