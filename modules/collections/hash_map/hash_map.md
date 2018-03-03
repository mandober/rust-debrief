# Hash Map

- module: `std::collections::hash_map`
- path: `std::collections::hash_map::HashMap` or `std::collections::HashMap`
- type: `HashMap<K, V, S = RandomState>`
- HashMap is implemented with linear probing and Robin Hood bucket stealing.



# HashMap

```rust
#[derive(Clone)]
pub struct HashMap<K, V, S = RandomState> { /* ... */ }
```



```rust
// HashMap is not in prelude
use std::collections::HashMap;

// Create. Type is inferred as `HashMap<&str, &str>`
let mut book_reviews = HashMap::new();

// Insert
book_reviews.insert("TRLB", "version 1");
book_reviews.insert("Cookbook", "nightly");

// Read
```


## Conversions

```rust
// From array of tuples: (&str, u8)
use std::collections::HashMap;
let array_of_tuples = [("foo", 0), ("bar", 1)];
let hashmap: HashMap<_, _> = array_of_tuples.iter().cloned().collect();
println!("hashap: {:?}", hashmap);
```
