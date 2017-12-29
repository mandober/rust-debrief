# Alignment
https://doc.rust-lang.org/stable/nomicon/repr-rust.html

First and foremost, *all types have an alignment* specified in bytes. The alignment of a type specifies what addresses are valid to store the value at.

A value of alignment n must only be stored at an address that is a multiple of n. So alignment 2 means it must be stored at an even address, and 1 means that it can be stored anywhere. Alignment is at least 1, and always a power of 2.

Most primitives are generally aligned to their size, although this is platform-specific behavior. In particular, on x86 u64 and f64 may be only aligned to 32 bits.

*A type's size must always be a multiple of its alignment*. This ensures that an array of that type may always be indexed by offsetting by a multiple of its size. Note that the size and alignment of a type may not be known statically in the case of dynamically sized types.

Rust gives you the following ways to lay out composite data:
- structs (named product types)
- tuples (anonymous product types)
- arrays (homogeneous product types)
- enums (named sum types, tagged unions)
An enum is said to be C-like if none of its variants have associated data.

Composite structures will have an alignment equal to the maximum of their fields' alignment. Rust will consequently insert padding where necessary to ensure that all fields are properly aligned and that the overall type's size is a multiple of its alignment.

...

However there are several cases where such a representation is inefficient. The classic case of this is Rust's *null pointer optimization*: an enum consisting of a single outer unit variant (e.g. None) and a (potentially nested) non- nullable pointer variant (e.g. &T) makes the tag unnecessary, because a null pointer value can safely be interpreted to mean that the unit variant is chosen instead. The net result is that, for example, `size_of::<Option<&T>>() == size_of::<&T>()`.

There are many types in Rust that are, or contain, non-nullable pointers such as Box<T>, Vec<T>, String, &T, and &mut T. Similarly, one can imagine nested enums pooling their tags into a single discriminant, as they are by definition known to have a limited range of valid values. In principle enums could use fairly elaborate algorithms to cache bits throughout nested types with special constrained representations. As such it is especially desirable that we leave enum layout unspecified today.
