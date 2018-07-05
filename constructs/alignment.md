# Alignment

- all types have alignment value specified in bytes
- alignment specifies what addresses are valid to store the value at
- these addresses must be a multiple of the alignment
- alignment is always a power of two: at least 1, then 2, 4, 8, 16, etc.
- most primitives are aligned to their size (although platform-specific)
- type's size must always be a multiple of its alignment


All types in Rust have an alignment specified in bytes. Alignment specifies what memory addresses are valid to store the value at. A value of alignment `n` must only be stored at an address that is a multiple of `n`. So alignment 2 means it must be stored at an even address, and 1 means that it can be stored anywhere.

Alignment is at least 1, and always a power of 2. Most primitives are generally aligned to their size, although this is platform specific behavior. In x86 architecture, `u64` and `f64` may be aligned only to 8 bytes (32 bits).

A type's size must always be a multiple of its alignment. This ensures that an array of that type may always be indexed by offsetting by a multiple of its size. The size and alignment of dynamically sized types (DST) may not be known statically i.e. at compile time.




## Array Alignment
An array, `[T; n]`, has the same alignment as `T`.

Arrays are densely packed and layed out in-order. An array of type `T` can always be indexed by offsetting the start of the array by (a multiple of) the size of `T`.

```rust
// array of Ts
let ar: [u16; 4] = [10, 11, 12, 13];

// size of T * array length == size of array of Ts
assert_eq!(size_of_val(&10_u16) * ar.len(), size_of_val(&ar));

// size of T == size of array of Ts divided by array length
assert_eq!(size_of_val(&10_u16), size_of_val(&ar) / ar.len());
```

## Unit Type Alignment
The unit type, `()`, is guaranteed as a zero-sized type to have a size of 0 and an alignment of 1.



## Structs

The memory layout of a struct is undefined by default to allow for compiler optimizations, such as field reordering.

The layout can be fixed with the `#[repr(...)]` attribute.

In either case, fields may be given in any order in a corresponding struct expression; the resulting struct value will always have the same memory layout.



Composite structures have an alignment equal to the maximum of their fields' alignment. The compiler will insert padding to ensure that all fields are properly aligned and that the overall type's size is a multiple of its alignment.

```rust
struct A {
  a: u8,          // 1B
  _pad1: [u8; 3], //+3B so the next field is aligned

  b: u32,         // 4B

  c: u16,         // 2B
  _pad2: [u8; 2], //+2B to make size of struct multiple of 4
}
```

This struct will be 4 byte aligned on an architecture that aligns these primitives to their respective sizes. The whole struct will therefore have a size that is a multiple of 4 bytes.

There is no indirection: all data is stored within the struct, as in C.


However there are several cases where such a representation is inefficient.

The classic case of this is _null pointer optimization (NPO)_: an enum consisting of a single outer unit variant (e.g. None) and a (potentially nested) non- nullable pointer variant (e.g. &T) makes the tag unnecessary, because a null pointer value can safely be interpreted to mean that the unit variant is chosen instead.

The result is that, e.g. `size_of::<Option<&T>>() == size_of::<&T>()`


There are many types in Rust that are, or contain, non-nullable pointers such as Box<T>, Vec<T>, String, &T, and &mut T. Similarly, one can imagine nested enums pooling their tags into a single discriminant, as they are by definition known to have a limited range of valid values. In principle enums could use fairly elaborate algorithms to cache bits throughout nested types with special constrained representations. As such it is especially desirable that we leave enum layout unspecified today.




## Enums
Each enum instance has a discriminant which is an integer associated to it that is used to determine which variant it holds. An opaque reference to this discriminant can be obtained with the mem::discriminant function.

Under the default representation, the specified discriminant is interpreted as an isize value although the compiler is allowed to use a smaller type in the actual memory layout. The size and thus acceptable values can be changed by using the C representation, `#[repr(C)]`, or a primitive representation, 
e.g. `#[repr(u8)]`,  `#[repr(u16)]`, etc.




Any enum value consumes as much memory as the largest variant for its corresponding enum type, as well as the size needed to store a discriminant.

Generally, an enum would be laid out (modulo the size and position of tag) approximately as:

```rust
enum Foo {
  A(u32),
  B(u64),
  C(u8),
}

// would be laid out as:

struct FooRepr {
  data: u64, // this is either a u64, u32, or u8 based on `tag`
   tag: u8,   // 0 = A, 1 = B, 2 = C
}
```

this is approximately how it would be laid out in general (modulo the size and position of tag).


However there are several cases where such a representation is inefficient. The classic case of this is Rust's "null pointer optimization": an enum consisting of a single outer unit variant (e.g. None) and a (potentially nested) non- nullable pointer variant (e.g. &T) makes the tag unnecessary, because a null pointer value can safely be interpreted to mean that the unit variant is chosen instead. The net result is that, for example, size_of::<Option<&T>>() == size_of::<&T>().

There are many types in Rust that are, or contain, non-nullable pointers such as Box<T>, Vec<T>, String, &T, and &mut T. Similarly, one can imagine nested enums pooling their tags into a single discriminant, as they are by definition known to have a limited range of valid values. In principle enums could use fairly elaborate algorithms to cache bits throughout nested types with special constrained representations. As such it is especially desirable that we leave enum layout unspecified today.


---
Links
- [repr]: https://doc.rust-lang.org/nightly/nomicon/repr-rust.html
- [0079]: http://rust-lang.github.io/rfcs/0079-undefined-struct-layout.html
