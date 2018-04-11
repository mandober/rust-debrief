# Alignment

- all types have alignment value specified in bytes.
- alignment specifies what addresses are valid to store the value at.
- these addresses must be multiples of alignment value.
- alignment value is at least 1, and always a power of 2.
- most primitives are aligned to their size (platform-specific)
- alignment must be a 
type's size must be a multiple of its alignment.



https://doc.rust-lang.org/stable/nomicon/repr-rust.html


## Alignment
All types in Rust have an alignment specified in bytes. The alignment of a type specifies what main memory addresses are valid to store the value at. A value of alignment `n` must only be stored at an address that is a multiple of `n`. So alignment 2 means it must be stored at an even address, and 1 means that it can be stored anywhere. Alignment is at least 1, and always a power of 2. Most primitives are generally aligned to their size, although this is platform specific behavior. In x86 architecture, `u64` and `f64` may be only aligned to 32 bits.

A type's size must always be a multiple of its alignment. This ensures that an array of that type may always be indexed by offsetting by a multiple of its size. The size and alignment of _dynamically sized types_ is not known (statically, at compile time).


## Laying out composite data
- arrays: homogeneous, product types
- structs: heterogeneous, named, product types
- tuples: heterogeneous, anonymous, product types
- enums: heterogeneous, named, sum types (tagged unions)

An enum is said to be field-less (C-like) if none of its variants have associated data.

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

This struct will be 4 byte aligned on an architecture that aligns these primitives to their respective sizes. The whole struct will therefore have a size that is a multiple of 4 bytes. There is no indirection: all data is stored within the struct, as in C. In Rust, save for arrays which are densely packed and in-order, the layout of data is not specified by default.






...

However there are several cases where such a representation is inefficient. The classic case of this is Rust's *null pointer optimization*: an enum consisting of a single outer unit variant (e.g. None) and a (potentially nested) non- nullable pointer variant (e.g. &T) makes the tag unnecessary, because a null pointer value can safely be interpreted to mean that the unit variant is chosen instead. The net result is that, for example, `size_of::<Option<&T>>() == size_of::<&T>()`.

There are many types in Rust that are, or contain, non-nullable pointers such as Box<T>, Vec<T>, String, &T, and &mut T. Similarly, one can imagine nested enums pooling their tags into a single discriminant, as they are by definition known to have a limited range of valid values. In principle enums could use fairly elaborate algorithms to cache bits throughout nested types with special constrained representations. As such it is especially desirable that we leave enum layout unspecified today.
