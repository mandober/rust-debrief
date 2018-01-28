# Slices

Slice and string slice are two primitive data types, usually seen in their borrowed form. They are a view into a block of memory represented as a two-part fat pointer stored on the stack: a pointer to the data and the length. 

There are 2 kinds of slices:
- **sequential slice**, `&[T]` is a dynamically-sized view into a contiguous sequence. 
- **string slice**, `&str`, is a dynamically-sized view into a string.

Both slices are solution to the problem of referring to a part of sequential types (not to type as a whole - that’s what references are for).

## Sequential types

Commonly used sequential types are array and vector. They are compound types and they are both able to hold homogenous elements of type `T`. Vector is a growable type whose size might not be known at compile-time; it always allocates on the heap. Array is a fixed type; once declared, its size,  which is a part of its type, cannot be changed. Assuming its elements are scalar primitives, array uses only the stack.

**Array**: fixed sequence, `[T; size]`
reference to an array: `&[T; size]`
reference to a part of an array: `&[T]`

**Vector**: growable sequence, `Vec<T>`
reference to a vector: `&Vec<T>`
reference to a part of a vector: `&[T]`

```rust
// array: fixed, owned sequence
let array: [u8; 3] = [1, 2, 3];
// reference to array
let r1: &[u8; 3] = &array;
// another reference to array
let ref r2: [u8; 3] = array;

// vector: growable, owned sequence
let vector: Vec<char> = vec!['x', 'z'];
// vec reference
let r1: &Vec<char> = &vector;
// another reference to vec
let ref r2: Vec<char> = vector;
```

Since array and vector are contiguous sequences and a reference to a part of a contiguous sequence is `&[T]`, then a contiguous sequence is `[T]`. 

```rust
let seq: [i32; 3] = [1, 2, 3];     // array
let seq: Vec<i32> = vec![1, 2, 3]; // or vec

// slice
let slice: &[i32] = &seq[..];

// OR: if referring to the whole 
// cont.seq, loose the brackets
let slice: &[i32] = &seq;

// OR: and now for some contiguous
// sequence type annotation:
let ref slice: [i32] = seq;
```

The same relationship, which the trio of array (fixed sequence), vector (growable sequence) and slice (part of sequence) form, is repeated with string literal (fixed), String (growable) and string slice (part). 
