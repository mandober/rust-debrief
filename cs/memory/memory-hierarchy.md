# Memory hierarchy


The memory hierarchy is a grouping of storage, based on the response times, with the faster components located towards the top of the hierarchy, closer to the CPU.

Going towards lower components, each descending layer is (orders of magnitude) slower with (orders of magnitude) larger capacity.

Practically, all storage can be divided into 3 tiers:
- Internal: CPU registers and several levels of cache (SRAM)
- Primary storage: main system memory (DRAM)
- Secondary storage (SDD, HDD)

Each tier can further be divided into its own hierarchy based on response times, and even though significant improvements are made (for example in the secondary storage with SSD pushing aside the traditional rotating disks) there is still a huge gap in terms of responsiveness and speed between adjacent memory tiers.

One way to increase system performance is minimizing how far down the memory hierarchy one has to go to fetch the data.


The following table illustrates this:

 ns         | cycles | notes
-----------:|-------:|-----------------
 0.3        |      1 | CPU @ 3Ghz
 1          |      3 | L1 cache reference
 3          |      9 | Branch mispredict
 4          |     12 | L2 cache reference
 17         |     51 | Mutex lock/unlock
 83         |    250 | RAM reference
 150,000    |        | 4K SSD Random access
 250,000    |        | 1MB seq. RAM read
 1,000,000  |        | 1MB seq. SSD read
 10,000,000 |  40mil | HDD seek
 20,000,000 |        | 1MB seq. HDD read

1s = 10^3 ms = 10^6 μm = 10^9 ns

These values may not precise, but they illustrate the speed ratios well enough. Multiplying the time (in the table above) by a billion, brings the time scale into our perspective, allowing for popular analogies such as this:
- accessing L1 is like grabbing a piece of paper from the desk (seconds)
- accessing L2 is like picking up a book from a nearby shelf (seconds)
- accessing RAM is like taking a 4-minute break (minutes)
- accessing HDD is like roaming the earth for 15 months (months)

It takes 1 clock cycle for CPU to execute a simple instruction, and at 3 GHz, 1 cycle is 0.3ns (a light beam travels approximately 10cm during that time). Electronic signals, depending a many factors, travel through circuitry at 50–99% of the speed of light.

