# Hash trait
https://doc.rust-lang.org/std/hash/trait.Hash.html

Trait `std::hash::Hash` 1.0.0

A hashable type.
Types implementing `Hash` are able to be hashed with an instance of `Hasher`.

```rust
pub trait Hash {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher;

    fn hash_slice<H>(data: &[Self], state: &mut H)
        where H: Hasher,
    { ... }
}
```


## Implementing Hash

You can derive Hash with `#[derive(Hash)]` if all fields implement `Hash`.

The resulting hash will be the combination of 
the values from calling hash on each field.

```rust
#[derive(Hash)]
struct Rustacean {
    name: String,
    country: String,
}
```

If you need more control over how a value is hashed, 
you can of course implement the Hash trait yourself:

```rust
use std::hash::{Hash, Hasher};

struct Person {
    id: u32,
    name: String,
    phone: u64,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
}
```


## `Hash` and `Eq`

When implementing both `Hash` and `Eq`, it is 
important that the following property holds:
  k1 == k2 -> hash(k1) == hash(k2)

In other words, if two keys are equal, their hashes must also be equal.
`HashMap` and `HashSet` both rely on this behavior.

Thankfully, you won't need to worry about upholding this property when deriving 
both `Eq` and `Hash` with `#[derive(PartialEq, Eq, Hash)]`.


## Required Methods

```rust
fn hash<H>(&self, state: &mut H)
    where H: Hasher
```

Feeds this value into the given Hasher.

Examples:

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

let mut hasher = DefaultHasher::new();
7920.hash(&mut hasher);
println!("Hash is {:x}!", hasher.finish());
```


## Provided Methods


```rust
fn hash_slice<H>(data: &[Self], state: &mut H)
    where H: Hasher
```
1.3.0

Feeds a slice of this type into the given Hasher.

Examples:

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

let mut hasher = DefaultHasher::new();
let numbers = [6, 28, 496, 8128];
Hash::hash_slice(&numbers, &mut hasher);
println!("Hash is {:x}!", hasher.finish());
```
