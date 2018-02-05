# Reference primitive

- Primitive Type reference, since 1.0.0
- docs: https://doc.rust-lang.org/std/primitive.reference.html
- References: shared, `&` and mutable, `&mut`
- reference represents a borrow of some owned value.
- ref can be obtained by using `&` or `&mut` operators on a value, or by using a `ref` or `ref mut` pattern.
- reference is just a pointer that is assumed to not be null.
- `Option<&T>` has the same memory representation as a nullable pointer, and can be passed across FFI boundaries as such.
- In most cases, refs can be used much like the original value. Field access,method calling, and indexing work the same (save for mutability rules). In addition, the comparison operators transparently defer to the referent's implementation, allowing refs to be compared the same as owned values.
- references have a lifetime



## Lifetime

References have a lifetime attached to them, which represents the scope for which the borrow is valid. A lifetime is said to "outlive" another one if its representative scope is as long or longer than the other. The `'static` lifetime is the longest lifetime, which represents the total life of the program. For example, string literals have a `'static` lifetime because the text data is embedded into the binary of the program, rather than in an allocation that needs to be dynamically managed.


## Coercion

Mutable reference, `&mut T`, can be freely coerced into reference, `&T`, with the same referent type.

Reference with _longer lifetime_ can be freely coerced into reference with _shorter lifetimes_.



## Traits

The following traits are implemented for all references, `&T`, _regardless of the type of its referent_:
- `Copy`
- `Clone` (this will not defer to `T`'s `Clone` implementation if it exists)
- `Deref`
- `Borrow`
- `Pointer`

Mutable references, `&mut T`, get all of the above except `Copy` and `Clone`(to prevent creating multiple simultaneous mutable borrows), plus `DerefMut` and `BorrowMut` regardless of the type of its referent. So mutable references have: 
- `Deref`, `DerefMut`,
- `Borrow`, `BorrowMut`,
- `Pointer`

The following traits are implemented on `&T` references if the underlying `T` also implements that trait:
- all the traits in `std::fmt` except `Pointer` and `fmt::Write`
- `PartialOrd`, `Ord`, `PartialEq`, `Eq`
- `AsRef`
- `Fn` (in addition, `&T` references get `FnMut` and `FnOnce` if `T: Fn`)
- `Hash`
- `ToSocketAddrs`

Mutable references, `&mut T`, get all of the above except `ToSocketAddrs`, plus the following, if `T` implements that trait:
- `AsMut`
- `FnMut` (in addition, `&mut T` references get `FnOnce` if `T: FnMut`)
- `fmt::Write`
- `Iterator`
- `DoubleEndedIterator`
- `ExactSizeIterator`
- `FusedIterator`
- `TrustedLen`
- `Send` (note that `&T` references only get `Send` if `T: Sync`)
- `io::Write`
- `Read`
- `Seek`
- `BufRead`


Note that due to _deref coercion_ simply calling a trait method will act as if they worked on references as well as they do on owned values. The implementations described here are meant for generic contexts, where the final type `T` is a type parameter or otherwise not locally known.

