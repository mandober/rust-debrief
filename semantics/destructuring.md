# Destructuring

<!-- TOC -->

- [Irrefutable patterns](#irrefutable-patterns)
- [Arrays](#arrays)
- [Tuples](#tuples)
- [Structs](#structs)
- [References](#references)
- [Box](#box)
- [Slices](#slices)
- [Feature gated](#feature-gated)

<!-- /TOC -->


Types that can be destructured:
- tuples
- arrays
- enums
- structs
- slices (feature gated, add attr `#![feature(slice_patterns)]`)
- pointer types
- boxes


Destructuring places
- Irrefutable patterns: binding sites (`let`, fn params, `for` loop)
  - constants cannot be used
- Refutable patterns with ``if/while let`
  - constants can be used and they must match exactly
  - use `_` non-binding pattern as "catch-all" (default, last match arm)



## Irrefutable patterns
Destructuring must take irrefutable pattern 

```rust
// let binding
let (x, y) = (4, 5);
println!("destr: {}, {}", x, y);


// in function parameters
fn destr_array([a, b]: [u8; 2]) {
  println!("destr. array: {}, {}", a, b);
}

fn destr_tup((a, b): (u8, u8)) {
  println!("destr. tuple: {}, {}", a, b);
}

let pair = [1, 2];
destr_array(pair);

let pair = (4, 5);
destr_tup(pair);
```


## Arrays

```rust
// let binding, irrefutable pattern
let [c, d] = [1, 2];

// use `_` to ignore elements
let [_, b, _] = [1, 2, 3];

// to use `..` to ignore the rest of elements,
// first enable `slice_patterns` feature:
#![feature(slice_patterns)]
let [a, ..] = [1, 2, 3];
```


## Tuples

```rust
// irrefutable patterns:
let (f, g, h) = (7, 8, 9);

// use `_` to ignore elements
let (_, b, _) = (7, 8, 9);

// use `..` to ignore the rest of elements (no feature flag needed)
let (a, ..) = (7, 8, 9);

// use `..` to ignore the elements in the middle
let (a, .., z) = (7, 8, 9, 10, 11);


// refutable patterns:

// constants must match exactly:
if let (5, b, 14, ..) = (5, 9, 14, 25) {
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


## Structs

```rust
// struct 1
struct Point { 
  x: u8,
  y: u8
}
let point = Point { x: 1, y: 2 };

// destructuring:
let Point { x, y } = p; // x=1, y=2

// destructuring with renaming:
let Point { x: a, y: b } = p; // a=1, b=2

// destructuring with renaming and ignoring
let Point { x: a, .. } = p; // a=1 (ignore the rest of fields)

// destructuring with ignoring
// let Point { x: _, y } = p; // y=2 (ignore the first field)
let Point { x: _, y } = p; // y=2 (ignore the first field)


// struct 2
struct TuplePoint {
  x: (u8, u8),
  y: u8
}
let tpoint = TuplePoint { x: (1, 2), y: 3 };

// destructuring
let TuplePoint { x, .. } = tpoint; // x => (1, 2)

// destructuring with renaming
let TuplePoint { x: (a, b), y } = tpoint;
println!("{:?}", x); // x => (1, 2)
println!("{:?}", y); // x => 3
println!("{}", a);   // a => 1
println!("{}", b);   // b => 2
```




## References
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

## Box
It is not possible to destructure a box in a match pattern in stable Rust; however, this feature is available in nightly release under the feature flag 
`#![feature(box_patterns)]`. Box patterns let you match on a box.

```rust
#![feature(box_patterns, box_syntax)]

// `box` is used to create a box
let b = Some(box 5);

match b {
  // and to destructure it
  Some(box n) => println!("Do not try and destructure the box {}", n),
         None => println!("Instead try to realize the truth"),
            _ => println!("There is no box")
}
```


## Slices

Vectors cannot be destructured directly, but they can be converted to slices: slices can be destructured using `#![feature(slice_patterns)]`

```rust
#![feature(slice_patterns)]
#![feature(match_default_bindings)]

// empty vector (slice)
let vec: Vec<u8> = vec!();
if let [] = v.as_slice() {
    println!("empty");
}

// non-empty slice
let v: Vec<u8> = vec!(1, 2, 3, 4, 5);
if let [_, s, ..] = v.as_slice() {
    println!("{:?}", s) 
}

// match slice of numbers
// add `#![feature(match_default_bindings)]` lest put `&` in matching arms:
// &[]  => ...
// &[x] => ...
match v.as_slice() {
  []         => println!("empty"),
  [x]        => println!("{}", x),
  [_, y, ..] => println!("{}", y),
  [head, ..] => println!("{}", head),
  [.., tail] => println!("{}", tail),
  // can match first and last, ignoring the rest:
  [head, .., tail] => println!("{}, {}", head, tail),
  // but binding the rest won't work:
  //[first, ..last] => ...
  _          => println!("no match");
}



// match slice of &str
let v: Vec<&str> = vec!("foo", "bar", "baz", "zik");
match v.as_slice() {
  ["foo", .., x]    => println!("{}", x),
  [_, "bar", z, _]  => println!("{}", z),
  [_, .., y]        => println!("{}", y),
  ["foo", ..]       => println!("foo"),
                  _ => println!("no match"),
};

// or:
// match slice of &str
match ["foo", "bar"] {
  ["foo", ..] => println!("foo"),
            _ => println!("no match"),
};
```


## Feature gated

```rust
#![feature(slice_patterns)]
#![feature(match_default_bindings)]
#![feature(pattern_parentheses)]
#![feature(exclusive_range_pattern)]
```

- `advanced_slice_patterns` lets you use `..` pattern in slice matching.
- `box_patterns`lets you match on `Box`es.
- `match_default_bindings` improves pattern-matching on references in `match`.
- `slice_patterns` matching against a slice or array, with `&` feature.
dotdoteq_in_patterns
exclusive_range_pattern
pattern
- `match_default_bindings` improves pattern-matching on references in `match`.
- `match_beginning_vert` add a '|' to the beginning of a match arm
