# Slices

Slice and string slice are two primitive data types, usually seen in their borrowed form. They are a view into a block of memory represented as a two-part fat pointer stored on the stack: a pointer to the data and the length.

There are 2 kinds of slices:
- **sequence slice**, `&[T]` is a dynamically-sized view into a contiguous sequence.
- **string slice**, `&str`, is a dynamically-sized view into a string.

Both slices are solution to the problem of referring to a part of sequential types (not to type as a whole - that’s what references are for).


## Sequential types

Commonly used sequential types are array, `[T; size]`, and vector, `Vec<T>`. They are both compound types, they both hold elements of some type, `T`.

Vector is a growable type of uncertain size, so it allocates its elements on the heap. Array is a fixed type, it must always have a known size (which is a part of its type), so it keeps its elements (assuming its elements are scalar primitives) on the stack.

**Array**: fixed sequence, `[T; n]`
reference to an array: `&[T; n]`
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





Both kinds of slices rarely appear in owned form, they are almost exclusively in their borrowed form in which they offer a quick code generalization without resorting to generics.














Slice and string slice are two primitive data types, usually seen in their 
borrowed form. They are a view into a block of memory represented as a two-part 
fat pointer stored on the stack: a pointer to the data and the length.

[slice](https://doc.rust-lang.org/std/primitive.slice.html)
[str](https://doc.rust-lang.org/std/primitive.str.html)

To disambiguate between these 2 slices, we should call the former a sequence slice;
and say that there are 2 slices: sequence slice and string slice.

A sequence slice, &[T], is a dynamically-sized view into a contiguous sequence.
A string slice, &str is a reference to the part of a string.

Both slices are solution to the problem of referring just to a part of 
sequential types, not to the type as a whole; that's what references are for.


## Sequential types

Commonly used sequential types are array, [T; size], and vector, Vec<T>.
They are both compound types, they can both hold elements of some type, T.
Vector is a growable type of uncertain size, so it allocates its elements on the
heap. Array is a fixed type, it must always have a known size (which is a part of
its type), so it keeps its elements (assuming its elements are scalar primitives)
on the stack.

Array: fixed sequence, [T; n]
reference to an array: &[T; n]
reference to a part of an array: &[T]

Vector: growable sequence, Vec<T>
reference to a vector: &Vec<T>
reference to a part of a vector: &[T]

```rust
// array: fixed owned sequence
let arr: [u8; 3] = [1, 2, 3];
// ref to array
let r1: &[u8; 3] = &arr;
// another ref to array
let ref r2: [u8; 3] = arr;
```

Since array and vector are contiguous sequences and
a reference to contiguous sequence is &[T], it means
a contiguous sequence is [T].

Contiguous sequences rarly appear in type annotations, though.

```rust
// fixed seq: array
let seq: [i32; 3] = [1, 2, 3];
// growable seq: vector
let seq: Vec<i32> = vec![1, 2, 3];
// slice: view into an cont. sequence
let slice: &[i32] = &seq[..];
// or
// if referring to the whole seq., loose the brackets:
let slice: &[i32] = &seq;
// or
// and now for some concrete contiguous sequence annotation:
let ref slice: [i32] = seq;
```

## Sequential string (text) types

The same relationship, which the trio of array (fixed sequence), vector (growable
sequence) and slice (part of sequence) form, is repeated with string literal 
(fixed), String (growable) and string slice (part).

String is an owned, growable, type consisting of a stack-stored meta data 
(length, capacity) and a pointer to string data allocated on the heap.

String literal is peculiar type: it behaves like an owned type, although it has 
a reference type, &str, 

unlike fixed contiguous sequence that an array is, is stored in
the final binary.

string literal: fixed sequence, &str
reference to a string literal i.e. string slice: &str
reference to a part of string literal i.e. string slice: &str[..], &str

String: growable string, String
reference to a String: &String
reference to (a part of) a String: &str[..], &str



















---



fixed         | growable        | borrowed            | view into
--------------|-----------------|---------------------|-----------
array `[T; n]`| vector `Vec<T>` | slice  `&[T]`       | `[T]` cont.seq.
     `&[T; n]`|       `&Vec<T>` |
     `&[T]`   |          `&[T]` |




Sequence slice refers to a part of contiguous sequence, 
which can be an array, [T; size], or vector, Vec<T>.


The contiguous sequence is usually an array or vector.

An array has a type [T; size]
Array is compound type 


Here, string, can be a owned String or a string literal.



## Slices and String slices

Slice and string slice are very similar types, they are both view into data.

Slice is a view into contiguous sequence `[T]`, 
which can be fixed i.e. an array, `[T; size]` or growable i.e. vector `Vec<T>`.

String slice is a view into string, which can be growable i.e. string `String`
or fixed i.e. string literal.
String literal doesn't have a type representation because it is considered as a 
form of string slice, even though string literals are stored in the final binary
unlike string slices which are, in fact, fat pointers and stored on the stack.

fixed         | growable        | borrowed            | view into
--------------|-----------------|---------------------|-----------
array `[T; n]`| vector `Vec<T>` | slice  `&[T]`       | `[T]` cont.seq.
     `&[T; n]`|       `&Vec<T>` |
     `&[T]`   |          `&[T]` |
 `&mut [T; n]`|   `&mut Vec<T>` |

string literal| string `String` | string slice, `&str`| `str`




```rust
// array: fixed, owned, sequence
let arr: [u8; 3] = [1, 2, 3];
// ref to array
let ar1: &[u8; 3] = &arr;
// another ref to array
let ref ar2: [u8; 3] = arr;




// array
let seq: [i32; 3] = [1, 2, 3];
// vector
let seq: Vec<i32> = vec![1, 2, 3];
// slice: view into an cont. sequence
let slice: &[i32] = &seq[..];
// or, if refering to the whole seq., loose the brackets:
let slice: &[i32] = &seq;
// and now for some contiguous sequence annotation:
let ref slice: [i32] = seq;

// must be exactly: seq[..]
let ref slice: [i32] = seq[..]; // type is [i32]


// string literal is stored in the binary: a slice into binary
let string: &str = "text";
// A heap-allocated owned String
let string: String = String::from("text");
// string slice: a view into a String/string literal
let view: &str = &string[..];
// or:
// rare str annotation:
let ref view: str = string[..]; // type is: str
```
