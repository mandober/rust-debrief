# Module `std::slice` 1.0.0
https://doc.rust-lang.org/std/slice/

A dynamically-sized view into a contiguous sequence, `[T]`.

Slices are a view into a block of memory represented as a pointer and a length.

```rust
// slicing a vector of integers
let vec = vec![1, 2, 3];
// NOTE: were the line above alone, elements would be inferred as i32, but
// because of the next line, they are in fact inferred as u8 from the context!
let int_slice: &[u8] = &vec[..];

// coercing an array to a string-slice slice
let str_slice: &[&str] = &["one", "two", "three"];
```

Slices are either mutable or shared. The shared slice type is `&[T]`, while the mutable slice type is `&mut [T]`, where T represents the element type. 

For example, you can mutate the block of memory that a mutable slice points to:

```rust
let x = &mut [1, 2, 3];
x[1] = 7;
assert_eq!(x, &[1, 7, 3]);
```


## Trait Implementations
There are several implementations of common traits for slices.
Some examples include:
`Clone`
`Eq`, `Ord` - for slices whose element type are Eq or Ord.
`Hash` - for slices whose element type is Hash.
`Iteration`

The slices implement `IntoIterator`. The iterator yields references to the slice elements.

```rust
let numbers = &[0, 1, 2];
for n in numbers {
    println!("{} is a number!", n);
}
```

The mutable slice yields mutable references to the elements:

```rust
let mut scores = [7, 8, 9];
for score in &mut scores[..] {
    *score += 1;
}
```

This iterator yields mutable references to the slice's elements, so while the element type of the slice is `i32`, the element type of the iterator is `&mut i32`.
- `iter` and `iter_mut` are the explicit methods to return the default iterators.
- alos return iterators: `split`, `splitn`, `chunks`, `windows`, etc.


## Structs
There are several structs that are useful for slices, such as Iter, which represents iteration over a slice.

`Chunks`
An iterator over a slice in (non-overlapping) chunks (size elements at a time).

`ChunksMut`
An iterator over a slice in (non-overlapping) mutable chunks (size elements at a time). When the slice len is not evenly divided by the chunk size, the last slice of the iteration will be the remainder.

`Iter`    Immutable slice iterator
`IterMut` Mutable slice iterator.
`Windows` An iterator over overlapping subslices of length size.

`RSplitN`
An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice.

`RSplitNMut`
An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice.

`Split`
An iterator over subslices separated by elements that match a predicate function.

`SplitMut`
An iterator over the subslices of the vector which are separated by elements that match pred.

`SplitN`
An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits.

`SplitNMut`
An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits.

`RSplit`
[LAB] An iterator over subslices separated by elements that match a predicate function, starting from the end of the slice.

`RSplitMut`
[LAB] An iterator over the subslices of the vector which are separated by elements that match pred, starting from the end of the slice.



## Traits

`SliceConcatExt`
[LAB] An extension trait for concatenating slices

`SliceIndex`
[LAB] A helper trait used for indexing operations.



## Functions

`from_raw_parts` ⚠ 
Forms a slice from a pointer and a length.

`from_raw_parts_mut` ⚠
Performs the same functionality as `from_raw_parts`, except that a mutable slice is returned.
