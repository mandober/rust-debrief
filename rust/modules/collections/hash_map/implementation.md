# HashMap implementation


By default, HashMap uses a hashing algorithm selected to provide resistance against HashDoS attacks. The algorithm is randomly seeded, and a reasonable best-effort is made to generate this seed from a high quality, secure source of randomness provided by the host without blocking the program. Because of this, the randomness of the seed depends on the output quality of the system's random number generator when the seed is created. In particular, seeds generated when the system's entropy pool is abnormally low such as during system boot may be of a lower quality.

The default hashing algorithm is currently [`SipHash 1-3`][siphash], though this is subject to change at any point in the future. While its performance is very competitive for medium sized keys, other hashing algorithms will outperform it for small keys such as integers as well as large keys such as long strings, though those algorithms will typically not protect against attacks such as HashDoS.

The hashing algorithm can be replaced on a per-HashMap basis using the default, `with_hasher`, and `with_capacity_and_hasher` methods. Many alternative algorithms are available on crates.io, such as the `fnv` crate.

It is required that the keys implement the `Eq` and `Hash` traits, although this can frequently be achieved by using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself, it is important that the following property holds:
`k1 == k2 -> hash(k1) == hash(k2)`
In other words, if two keys are equal, their hashes must be equal.

It is a logic error for a key to be modified in such a way that the key's hash, as determined by the `Hash` trait, or its equality, as determined by the `Eq` trait, changes while it is in the map. This is normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.



## Robin Hood hashing papers:

- Pedro Celis "[Robin Hood Hashing](https:cs.uwaterloo.ca/research/tr/1986/CS-86-14.pdf)" (pdf)
- Emmanuel Goossaert "[Robin Hood hashing](http:codecapsule.com/2013/11/11/robin-hood-hashing/)"
- Emmanuel Goossaert "[Robin Hood hashing: backward shift deletion](http:codecapsule.com/2013/11/17/robin-hood-hashing-backward-shift-deletion/)"


## Hashing algorithams

- [siphash][siphash]
- [CityHash][cityhash]
- [MurmurHash2][smhasher]
- [MurmurHash3][smhasher]
- [SMHasher][smhasher]

[siphash]: https:131002.net/siphash/
[cityhash]: https:github.com/google/cityhash
[smhasher]: https:github.com/aappleby/smhasher



## Robin Hood Hashing

The main performance trick in this hashmap is called Robin Hood Hashing. It gains its excellent performance from one essential operation:

    If an insertion collides with an existing element, and that element's
    "probe distance" (how far away the element is from its ideal location)
    is higher than how far we've already probed, swap the elements.

This massively lowers variance in probe distance, and allows us to get very high load factors with good performance. The 90% load factor used is rather conservative.

> Why a load factor of approximately 90%?

In general, all the distances to initial buckets will converge on the mean. At a load factor of α, the odds of finding the target bucket after k probes is approximately 1-α^k. If we set this equal to 50% (since we converge on the mean) and set k=8 (64-byte cache line / 8-byte hash), α=0.92. I round this down to make the math easier on the CPU and avoid its FPU. Since on average we start the probing in the middle of a cache line, this strategy pulls in two cache lines of hashes on every lookup. I think that's pretty good, but if you want to trade off some space, it could go down to one cache line on average with an α of 0.84.

> Wait, what? Where did you get 1-α^k from?

On the first probe, your odds of a collision with an existing element is α.
The odds of doing this twice in a row is approximately α^2. For three times,
α^3, etc. Therefore, the odds of colliding k times is α^k. The odds of NOT
colliding after k tries is 1-α^k.

The paper from 1986 cited mentions an implementation which keeps track
of the distance-to-initial-bucket histogram. This approach is not suitable
for modern architectures because it requires maintaining an internal data
structure. This allows very good first guesses, but we are most concerned
with guessing entire cache lines, not individual indexes. Furthermore, array
accesses are no longer linear and in one direction, as we have now. There
is also memory and cache pressure that this would entail that would be very
difficult to properly see in a microbenchmark.

> Future Improvements

- Allow the load factor to be changed dynamically and/or at initialization.
- Possiblity of reusing storage when growing the underlying table? This is exactly the use case for 'realloc', and may be worth exploring.


> Future Optimizations

Another design choice made without any real reason is parameterizing the raw table over keys and values. Technically, all that is needed is the size and alignment of keys and values, and the code should be just as efficient (well, we might need one for power-of-two size and one for not...). This has the potential to reduce code bloat in rust executables, without really losing anything except 4 words (key size, key alignment, val size, val alignment) which can be passed in to every call of a `RawTable` function. This would definitely be an avenue worth exploring.

Annotate exceedingly likely branches in `table::make_hash`
and `search_hashed` to reduce instruction cache pressure
and mispredictions once it becomes possible (blocked on issue #11092).

Shrinking the table could simply reallocate in place after moving buckets
to the first half.


> The growth algorithm

The growth algorithm is basically a fast path of the naive reinsertion-
during-resize algorithm. Other paths should never be taken.

Consider growing a robin hood hashtable of capacity n. Normally, we do this
by allocating a new table of capacity `2n`, and then individually reinsert
each element in the old table into the new one. This guarantees that the
new table is a valid robin hood hashtable with all the desired statistical
properties. Remark that the order we reinsert the elements in should not
matter. For simplicity and efficiency, we will consider only linear
reinsertions, which consist of reinserting all elements in the old table
into the new one by increasing order of index. However we will not be
starting our reinsertions from index 0 in general. If we start from index
i, for the purpose of reinsertion we will consider all elements with real
index j < i to have virtual index n + j.

Our hash generation scheme consists of generating a 64-bit hash and
truncating the most significant bits. When moving to the new table, we
simply introduce a new bit to the front of the hash. Therefore, if an
elements has ideal index i in the old table, it can have one of two ideal
locations in the new table. If the new bit is 0, then the new ideal index
is i. If the new bit is 1, then the new ideal index is n + i. Intuitively,
we are producing two independent tables of size n, and for each element we
independently choose which table to insert it into with equal probability.
However the rather than wrapping around themselves on overflowing their
indexes, the first table overflows into the first, and the first into the
second. Visually, our new table will look something like:

`[yy_xxx_xxxx_xxx|xx_yyy_yyyy_yyy]`

Where x's are elements inserted into the first table, y's are elements
inserted into the second, and _'s are empty sections. We now define a few
key concepts that we will use later. Note that this is a very abstract
perspective of the table. A real resized table would be at least half
empty.

Theorem: A linear robin hood reinsertion from the first ideal element
produces identical results to a linear naive reinsertion from the same
element.


> Adaptive early resizing

To protect against degenerate performance scenarios (including DOS attacks),
the implementation includes an adaptive behavior that can resize the map
early (before its capacity is exceeded) when suspiciously long probe sequences
are encountered.

With this algorithm in place it would be possible to turn a CPU attack into
a memory attack due to the aggressive resizing. To prevent that the
adaptive behavior only triggers when the map is at least half full.
This reduces the effectiveness of the algorithm but also makes it completely safe.

The previous safety measure also prevents degenerate interactions with
really bad quality hash algorithms that can make normal inputs look like a
DOS attack.

The threshold of 128 is chosen to minimize the chance of exceeding it. In particular, we want that chance to be less than 10^-8 with a load of 90%. For displacement, the smallest constant that fits our needs is 90, so we round that up to 128.

At a load factor of α, the odds of finding the target bucket after exactly n
unsuccessful probes are

```
Pr_α{displacement = n} = 
(1 - α) / α * ∑_{k≥1} e^(-kα) * (kα)^(k+n) / (k + n)! * (1 - kα / (k + n + 1))
```

We use this formula to find the probability of triggering the adaptive behavior:

`Pr_0.909{displacement > 128} = 1.601 * 10^-11`


> Paper

Alfredo Viola (2005) "Distributional analysis of Robin Hood linear probing hashing with buckets".
