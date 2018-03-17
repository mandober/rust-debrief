# Destructuring

Types that can be destructured:
- tuples
- enums
- pointer types
- structs
- arrays (feature gated, add attr `#![feature(slice_patterns)]`)


Destructuring
- Irrefutable patterns: binding sites (`let`, function's parameters, etc.)
  - use `_` non-binding pattern to ignore an element
  - use `..` ("the rest" pattern) to ignore the rest of elements
  - constants cannot be used
- Refutable patterns with `match`/`if let`/`while let`
  - use `_` non-binding pattern to ignore an element
  - use `_` non-binding pattern as "catch-all" (default, last match arm)
  - use `..` ("the rest" pattern) to ignore the rest of elements
  - constants can be used and they must match exactly



## Array destructuring

```rust
#![feature(slice_patterns)]
let array = [1, 2, 3];
let [_, b, ..] = array; // b=2
```

## Tuple destructuring

```rust
let tuple = (1, 2, 3);
let (_, b, ..) = tuple; // b=2
```


Tuple destructuring with refutable patterns (`match/if let/while let`)

```rust
// tuple
let tuple = (5, 9, 14, 25);
// constants (5 and 14) must match exactly:
if let (5, b, 14, ..) = tuple {
  b; // b=9
}


// tuple
let pair = (122, 255);
// match expression (when a pattern is constant, exact match is needed)
match pair {
  (122, y) => println!("122 and {}", y),
  // or:
  (x, y) if x == 122 => println!("122 and {}", y),

  // the match-all pattern in this case is:
  (x, y) => println!("{} and {}", x, y),
  
  // by now, all possibilities are exhausted, so this can't be used:
  // _ => ...
}
```


## Struct destructuring

```rust
// declaring
struct Point { x: u8, y: u8 }

// instantiating
let point = Point { x: 1, y: 2 };

// destructuring
let Point { x, y } = p; // x=1, y=2

// destructuring with renaming
let Point { x: a, y: b } = p; // a=1, b=2

// destructuring with renaming and ignoring
let Point { x: a, .. } = p; // a=1 (ignore the rest of fields)

// destructuring with ignoring
let Point { x: _, y } = p; // y=2 (ignore the first field)
```



## Pointers types destructuring
There's a distinction between destructuring and dereferencing:
- dereferencing uses `*`
- destructuring uses `&`, `ref`/`mut`/`box` (`box` is feature gated)

```rust
let rf = &42;
match rf {
  // copy!
  &v => println!("&T => T by copying: {}", v),
}

// or:
// avoid `&` by dereferencing before matching:
match *rf {
  v => println!("{}", v),
}
```
