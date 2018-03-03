# Hash Set

- Struct `std::collections::HashSet`, 1.0.0
- A hash set implemented as a HashMap where the value is ().



```rust
pub struct HashSet<T, S = RandomState> { /* fields omitted */ }
```

As with the HashMap type, a HashSet requires that the elements implement the `Eq` and `Hash` traits. This can frequently be achieved by using 
`#[derive(PartialEq, Eq, Hash)]`. 

If you implement these yourself, it is important that the following property holds: `k1 == k2 -> hash(k1) == hash(k2)`
In other words, if two keys are equal, their hashes must be equal.

It is a logic error for an item to be modified in such a way that the item's hash, as determined by the `Hash` trait, or its equality, as determined by the `Eq` trait, changes while it is in the set. This is normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.




## Conversions


```rust
use std::collections::HashSet;

// From array into iter
let array = [ "ab", "bc", "cd" ];
let hashset: HashSet<_> = array.into_iter().collect();

// From vec
use std::iter::FromIterator;
fn vec_to_set(vector: Vec<u8>) -> HashSet<u8> {
  HashSet::from_iter(vector)
}

// From slice
use std::iter::FromIterator;
fn hashset(slice: &[u8]) -> HashSet<u8> {
  HashSet::from_iter(slice.iter().cloned())
}
```
