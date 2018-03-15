# Slice

## Definition

*from std::slice module:*  
A dynamically-sized view into a contiguous sequence, `[T]`.
Slices are a view into a block of memory represented as a pointer and a length.




Slice, `&[T]`or `&mut [T]`, is a dynamically-sized view into a contiguous sequence, `[T]` (array or vector).

Slices are a view into a block of memory represented as a pointer and a length; it is a 2-part fat pointer: a pointer to the data (elements of contiguous sequence, stored either on the stack or on the heap), and the length (information that "fattens" the pointer and represents the number of elements in the slice).

Slice has no ownership over the data it points to, it just borrows it, so it almost exclusively appears in its borrowed form. Despite this definition taken from official documents, there is some ambiguity as to what the slice refers to. In many articles "slice" is used to indicate the contiguous sequence itself as well as a mutable or immutable reference to the contigous sequence (as a whole or some part). Strictly, slice is a kind of type that always appears in the borrowed form. It is mutably or immutably borrowing a (part of) contiguous sequence; slice is a reference`&[T]` or `&mut [T]`, but contiguous sequence is not, `[T]`. In fact, contiguous sequence is a peculiar type: it is not a reference, but it isn't an owned type either.


```rust
// contiguous sequence: array
let seq: [i32; 6] = [1, 2, 3, 4, 5, 6];
// contiguous sequence: vector
let seq: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
// a slice into a part of contiguous sequence
let slice: &[i32] = &seq[2..5];
// a slice into a whole contiguous sequence
let slice: &[i32] = &seq[..];
// a slice into a whole contiguous sequence
let slice: &[i32] = &seq;
// a slice into a whole contiguous sequence. Rare cont.seq. annotation:
let ref slice: [i32] = seq[..];

```


Slices are either mutable or shared:
- shared slice `&[T]`
- mutable slice `&mut [T]`

Slices let you reference a contiguous sequence of
elements in a collection or the whole collection.

All in-bounds elements of arrays and slices are always initialized,
and access to an array or slice is always bounds-checked.

```rust
// A stack-allocated array
let seq: [i32; 3] = [1, 2, 3];
// A heap-allocated array i.e. vector
let seq: Vec<i32> = vec![1, 2, 3];
// A slice into an sequence (array/vector)
let slice: &[i32] = &seq[..];
```



## Unstable feature: `slice_patterns`
https://doc.rust-lang.org/unstable-book/language-features/slice-patterns.html
The tracking issue for this feature is: #23121

If you want to match against a slice or array, you can use `&` with the `slice_patterns` feature:

```rust
#![feature(slice_patterns)]

fn main() {
    let v = vec!["match_this", "1"];

    match &v[..] {
        &["match_this", second] => println!("The second element is {}", second),
        _ => {},
    }
}
```



## Unstable feature: `advanced_slice_patterns`
https://doc.rust-lang.org/unstable-book/language-features/advanced-slice-patterns.html

The tracking issue for this feature is: #23121

The `advanced_slice_patterns` gate lets you use `..` to indicate any number of elements inside a pattern matching a slice. This wildcard can only be used once for a given array. If there's an identifier before the `..`, the result of the slice will be bound to that name. For example:

```rust
#![feature(advanced_slice_patterns, slice_patterns)]

fn is_symmetric(list: &[u32]) -> bool {
    match list {
        &[] | &[_] => true,
        &[x, ref inside.., y] if x == y => is_symmetric(inside),
        _ => false
    }
}

fn main() {
    let sym = &[0, 1, 4, 2, 4, 1, 0];
    assert!(is_symmetric(sym));

    let not_sym = &[0, 1, 7, 2, 4, 1, 0];
    assert!(!is_symmetric(not_sym));
}
```
