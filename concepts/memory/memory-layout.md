# Memory Layout

- `#[repr(...)]` attribute is used to fix a memory layout
- `#[repr(Rust)]` default (Rust) repr uses unspecified layout
- `#[repr(C)]` uses the C layout




## Data Layout

Except for arrays, the layout of data is not specified in Rust.

Mechanisms to lay out compound types in memory:
- arrays: homogeneous, product types
- structs: heterogeneous, named, product types
- tuples: heterogeneous, anonymous, product types
- enums: heterogeneous, named, sum types (tagged unions)



## Rust repr

Default memory layout is used if attribute `#[repr(Rust)]` is explicitly specified or if the `#[repr(...)]` attribute is entirely omitted.

This representation means that the memory layout is left unspecified in order to allow for compiler optimizations, such as field reordering.

Under this representation the types are aligned on byte boundaries to the power of two. The types use at least 1 byte, then 2, 4, 8, 16, etc. 

Primitives are aligned to their size.

Structs have alignment according to the largest field.


## repr attribute

Memory layout can be fixed with the `#[repr(...)]` attribute.

In either case, fields may be given in any order in a corresponding struct expression; the resulting struct value will always have the same memory layout.


## Array Layout
Arrays are laid out so that the nth element of the array is offset from the start of the array by n * the size of the type bytes. An array of [T; n] has a size of size_of::<T>() * n and the same alignment of T.

Arrays are in-order, densely packed. An array of type `T` can always be indexed via offsetting by a multiple of the size of `T`.

```rust
// array of Ts
let ar: [u16; 4] = [10, 11, 12, 13];

// size of T * array length == size of array of Ts
assert_eq!(size_of_val(&10_u16) * ar.len(), size_of_val(&ar));

// size of T == size of array of Ts divided by array length
assert_eq!(size_of_val(&10_u16), size_of_val(&ar) / ar.len());
```

## Slice Layout
Slices have the same layout as the section of the array they slice.

This goes for the raw `[T]` type, not pointers (`&[T]`, `Box<[T]>`, etc.) to slices.


## Tuple Layout
Tuples do not have any guarantees about their layout.

The exception to this is the unit tuple, `()`, which is guaranteed as a zero-sized type to have a size of 0 and an alignment of 1.


## Union Layout
The memory layout of a union is undefined by default, but the `#[repr(...)]` attribute can be used to fix a layout.
