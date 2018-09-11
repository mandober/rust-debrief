# Module `std::vec`
1.0.0
https://doc.rust-lang.org/stable/std/vec/index.html

A contiguous growable array type with heap-allocated contents, written `Vec<T>`.

Vectors have O(1) indexing, 
amortized O(1) push (to the end) 
and O(1) pop (from the end).

```rust
// You can explicitly create a Vec<T> with new:
let v: Vec<i32> = Vec::new();

// or by using the vec! macro:
let v: Vec<i32> = vec![];
let v = vec![1, 2, 3, 4, 5];
let v = vec![0; 10]; // ten zeroes

// You can push values onto the end (which will grow the vector as needed):
let mut v = vec![1, 2];
v.push(3);

// Popping values works in much the same way:
let mut v = vec![1, 2];
let two = v.pop();

// Vectors also support indexing (through the Index and IndexMut traits):
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;
```

## Structs

`Drain` A draining iterator for Vec<T>.
`IntoIter` An iterator that moves out of a vector.
`Splice` A splicing iterator for Vec.
`Vec` A contiguous growable array type, written Vec<T> but pronounced 'vector'.

`DrainFilter` [LAB] An iterator produced by calling drain_filter on Vec.
`PlaceBack` [LAB] A place for insertion at the back of a Vec.
