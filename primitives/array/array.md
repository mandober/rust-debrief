# Array


<!-- TOC -->

- [Storage](#storage)
- [Syntax](#syntax)
- [Type](#type)
- [Access](#access)
- [Coercion](#coercion)
- [Implemented traits](#implemented-traits)

<!-- /TOC -->

## Arrays



## Storage
For example, if a variable referring to a large array, needs to be supplied by value to a function; if the array holds primitives then everything is on the stack and there will be a lot of data moving around; even if the array's elements are non-primitives and thus stored on the heap, for a very large array that involves moving the array itself around (an array with 1000 elements which are all pointers to the data on the heap)
 
 
## Syntax
There are 2 forms for creating an array:
- list literal expression: `[x, y, z]`
- repeat expression: `[x; N]`, which produces an array with `N` copies of `x`, but the type of `x` must be `Copy` (otherwise an error is emitted).

```rust
// literal expression:
let arr = [0, 1, 2];

// repeat expression:
let arr = [0; 3];

// invalid repeat expression:
let arr = [String::from("array"); 3]; // error:
// `Copy` trait is required because the repeated element will be copied
```


## Type

Array's size is a part of its type; the size must be specified as an `usize` constant (must be known at compile-time).

```rust
// type and size are inferred
let arr = [253; 6]; // arr: [i32; 6]

// the type can be elided, the size cannot:
let arr: [_; 6] = [253; 6];

// explicit type annotation enforces the specified type:
let arr: [u8; 6] = [253; 6];

// array's size must be an usize constant
const SIZE: usize = 6;
let arr= [253; SIZE];
```

Arrays of any size are `Copy` if the element type is `Copy` and `Clone` if the element type is `Clone`. This works because `Copy` and `Clone` traits are specially known to the compiler.

References to an array:
- shared reference to an array: `&[T; N]`
- mutable reference: `&mut [T; N]`

A slice is a view into a contiguous sequence (which array is):
- shared slice : `&[T]`
- mutable slices: `&mut [T]`




## Access
Arrays can be indexed with the brackets operator, `[]`. Indexing starts at 0 and it is checked for out-of-bounds errors at run-time; out-of-bounds indexing causes panic.

An array can be mutable, but its size must remain the same.

```rust
let mut array: [i32; 3] = [0; 3];
array[1] = 1;
// array is now [0, 1, 0]
```


It is impossible to move an element out of an array; the (in)direct approach is to use `mem::replace`, which replaces (and returns) an element with another (arbitrary) value of the same type.

```rust
let mut arr = [5, 6];
let x = ::std::mem::replace(&mut arr[0], 7);
println!("{:?}", arr); // [7, 6]
println!("{}", x);     // 5
```



## Coercion
Arrays don't have any inherent methods, but they coerce to slices and the slice provides all the methods that may be called on an array.

Using iterators avoids out-of-bounds array checks: although array itself is not iterable, the slice is.

```rust
let array = [5; 10];
// array is not iterable:
for x in array { } // error:
// the trait bound `[i32; 10]: std::iter::Iterator` is not satisfied

// the solution is to coerce the array
// to a slice by calling slice's iter method:
for x in array.iter() { }
```

If the array has 32 or fewer elements, a reference to an array implements `IntoIterator` trait, providing `into_iter()` method that can be used for iteration.

```rust
let xs: [i32; 4] = [7, 8, 9, 10];
let rs: &[i32; 4] = &xs;
for r in rs {
  println!("{}", r);
}

// a ref to array implements IntoIterator trait
impl<'a, T> IntoIterator for &'a [T; SIZE] { // only if SIZE <= 32
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;
}
```


## Implemented traits

Arrays, of sizes up to 32, implement these traits if the element's type allows:
- `Clone` (only if `T: Copy`)
- `Debug`
- `IntoIterator` (implemented for `&[T; N]` and `&mut [T; N]`)
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`
- `Hash`
- `AsRef`, `AsMut`
- `Borrow`, `BorrowMut`
- `Default`

This limitation exists because Rust doesn't yet support code that is generic over the size of an array type. For example, `[Foo; 3]` and `[Bar; 3]` are instances of same generic type `[T; 3]`, but `[Foo; 3]` and `[Foo; 5]` are __entirely different types__.



```rust
impl<'a, 'b, A, B> PartialEq<[B; SIZE]> for [A; SIZE]
  where A: PartialEq<B>
{
  fn eq(&self, other: &[B; SIZE]) -> bool;
  fn ne(&self, other: &[B; SIZE]) -> bool;
}

// ...
```
