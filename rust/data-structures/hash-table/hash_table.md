# Hash table 

A hash table (hash map) is a data structure which implements an associative array ADT, a structure that can map keys to values. A hash table uses a hash function to produce a key into the associative array (called buckets or slots). Since lookup for the key is done in constant time, hash table are very efficient for insert, search, delete operations, which are all done in O(1) time complexity, with O(n) space complexity.

Hash tables are on average more efficient than search trees or other table lookup structures; they are widely used in many kinds of applications, particularly for associative arrays, database indexing, caches and sets.

For example, a phone book that uses a hash table as a storage: an input (a person's name) is fed into a hash function that produces a key; an (ideally unique) key into an associative array; the key identifies an entry in the array where person's details are stored (name, phone number, etc.).

`name -> fn_hash(name) -> hash; storage: { key: ["name", "phone"] }`

If hash function produces a number that is independent of the array size, the number is then reduced to a number that fits within array, `(0, array_len]`, using the modulo operator.

`fn_hash(input) -> hash` and `key = hash % array_size`


A perfect hash function produces a unique key for each input, but if not perfect, it will produce the same key for different inputs, resulting in a collision. It is not impossible to create a perfect hashing function provided time and space are unbounded, which is hardly ever the case. That's why hashing functions need to be as efficient with space and time as allowed (tolerated), which means collisions are inevitable and so anticipated.


## Terms

An associative array map keys to values, so it is indexed by a _key_, sometimes called an _index_. Index is, unambiguously, an index into array: with indexed arrays, the index is a number and it is indeed called an index; with associative array, an index can be a number (in which case it is called an index) or a string, a stringyfied number or just about anything else, in which case it is usually called a key. In fact, some implementations of associative arrays can have both, a numeric indices along with (non-numeric) keys.

- key:
  - input value (fed into a hashing function)
  - index of (associative) array
  - output value (produced by hashing function)
- hash:
  - output value (produced by hashing function)
- bucket:
  - index (key) of (associative) array
  - a final value of hashing and modulo operation.
- bucket chain:
  - in separate chaining scheme it is the number of units of associated storage. If that storage is a linked list e.g. if each array's entry has its own linked list - the number of nodes in that list is the bucket chain. 
- load factor: `n/k` where `n` is the number of entries occupied in the hash table and `k` is the number of buckets.
- hashing: squashing a value into its small representation



## Collision resolution

Hash collisions are practically unavoidable when hashing a random subset of a large set of possible keys. For example, if 2450 keys are hashed into a million buckets, even with a perfectly uniform random distribution, according to the birthday problem there is approximately a 95% chance of at least two of the keys being hashed to the same slot.

There are several strategies for resolving collisions, two of the most common ones are separate chaining and open addressing.