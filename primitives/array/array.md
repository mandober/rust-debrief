# Array

- online [std doc](https://doc.rust-lang.org/std/primitive.array.html)
- no accompanying module in std
- array is a primitive, compound, generic type; it is a contiguous, fixed-size, sequence of homogenous elements. As a structural type, it only has a literal expression (there's no name `array`).
- type annotation: `[T; N]`, for a constant size N, where N>=0
  - N is the length of the array and a part of its type.
  - N is non-negative compile-time constant.
  - N (like all numbers for index, size and length) is `usize`
- shared reference to array: `&[T; N]`; mut ref: `&mut [T; N]`; but since array coerces to slice, slice is also a (mut) ref to array: `&[T]` or `&mut [T]`


- subtypes: the number of all array types is a product of all available Rust types times `usize`: `T × usize`


- arrays of any size are Copy if the element type is Copy
- trait implementations are statically generated up to size 32
- array cannot be resized
- no way to move elements out of an array (alt: `mem::replace`)
- indexed with `[]`
- access is bounds-checked at run-time, out of bounds indexing causes panic

- array doesn't have any methods, slice does
- array itself is not iterable, the slice is
- can coerced to slice `[T]`


## Types of array

- type annotation: `[T; N]`, for a constant size N, where N>=0
  - N is the length of the array and a part of its type.
  - N is non-negative compile-time constant.
  - N (like all numbers for index, size and length) is `usize`


Array is one big generic supertype, `[T; N]`,
If N is a fixed size constant, for example 3,
then, for example, `[i32; 3]` and `[&str; 3]`
are *instances of same generic type* `[T; 3]`,
but, for example, `[f32; 3]` and `[f32; 5]`
are *entirely different types*.


```rust
// type annotated array
let a: [i32; 6] = [9; 6];
// type partially annotated, partially inferred
let mut a: [_; 6] = [9; 6];
// can mutate the array (but size must remain the same)
a = [3; 6];

println!("{:?}", a);
println!("{}b", std::mem::size_of_val(&a)); // 24b
```

There are two syntactic forms for creating an array:
- A list with each element, i.e. `[x, y, z]`.
- A repeat expression `[x; N]`, which produces an array with `N` copies of x.
  The type of x must be `Copy`.


## Implemented traits

Arrays of sizes 0-32 implement these traits if the element type allows it:

- `Clone` (only if `T: Copy`)
- `Debug`
- `IntoIterator` (implemented for `&[T; N]` and `&mut [T; N]`)
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`
- `Hash`
- `AsRef`, `AsMut`
- `Borrow`, `BorrowMut`
- `Default`


This limitation on the size N exists because Rust doesn't yet support code that is generic over the size of an array type.

`[Foo; 3]` and `[Bar; 3]` are instances of same generic type `[T; 3]`,
but `[Foo; 3]` and `[Foo; 5]` are *entirely different types*.

As a stopgap, trait implementations are statically generated up to size 32.

Arrays of any size are `Copy` if the element type is `Copy`.
This works because the `Copy` trait is specially known to the compiler.

Arrays coerce to slices `[T]`, so a slice method may be called on an array.
Indeed, this provides most of the API for working with arrays.
Slices have a dynamic size and do not coerce to arrays.

There is *no way to move elements out of an array*.
See `mem::replace` for an alternative.


## Examples:

```rust
let mut array: [i32; 3] = [0; 3];
array[1] = 1;
array[2] = 2;

assert_eq!([1, 2], &array[1..]);

// This loop prints: 0 1 2
for x in &array {
    print!("{} ", x);
}
```

An array itself is not iterable:

```rust
let array: [i32; 3] = [0; 3];
for x in array { }
// error: the trait bound `[i32; 3]: std::iter::Iterator` is not satisfied
```

The solution is to coerce the array to a slice by calling a slice method:

```rust
for x in array.iter() { }
```

If the array has 32 or fewer elements, you can also use 
the array reference's `IntoIterator` implementation:

```rust
for x in &array { }
```


## Trait Implementations

```rust
impl<'a, 'b, A, B> PartialEq<[B; 4]> for [A; 4]
  where A: PartialEq<B>
{
  fn eq(&self, other: &[B; 4]) -> bool;
  fn ne(&self, other: &[B; 4]) -> bool;
}

impl<'a, 'b, A, B> PartialEq<[B]> for [A; 4]
  where A: PartialEq<B>
{
  fn eq(&self, other: &[B]) -> bool;
  fn ne(&self, other: &[B]) -> bool;
}


impl<'a, 'b, A, B> PartialEq<&'b [B]> for [A; 4]
  where A: PartialEq<B>
{
  fn eq(&self, other: &&'b [B]) -> bool {};
  fn ne(&self, other: &&'b [B]) -> bool {};
}

impl<'a, 'b, A, B> PartialEq<&'b mut [B]> for [A; 4]
  where A: PartialEq<B>
{
  fn eq(&self, other: &&'b mut [B]) -> bool;
  fn ne(&self, other: &&'b mut [B]) -> bool;
}
```
