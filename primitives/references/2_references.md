# References


## Debrief
- References: `&['x ][mut ]T`
- std [docs](https://doc.rust-lang.org/std/primitive.reference.html) online
- Reference is a primitive, structural type (it has no nominal name).
- It is denoted with `&`: ref to value of type `T` is `&T` or `&mut T`
- Reference is basically just a pointer that is assumed to not be `null`.
- Rust has 2 types of refs, shared and mutable. They must obey 2 rules:
  1. reference cannot outlive its referent
  2. mutable reference cannot be aliased
- Therefore, shared ref has copy, while mutable ref has move semantics.
- Taking a ref to a value is called borrowing:
  - Shared ref is a borrow of value of an owning type.
  - While there are shared refs, referent is frozen: cannot move
  - Mut ref is a borrow of mutable value of an owning type.
  - While there's mut ref, referent is locked: cannot move, no more borrows
- Refs are obtained by using `&` or `&mut` operators, which are akin to type constructor: they take a value of some type and a lifetime, producing a ref.
- Lifetimes describe relationship between refs, and refs and their referents helping in enforcing validity of refs through compile-time checks.
- Every ref has a lifetime, which is the scope for which that ref is valid. 
- References can also be obtained by using a `ref` or `ref mut` pattern.
- References are dereferenced with the `*` operator.
- Optional reference, `Option<&T>`, has the same memory representation as a nullable pointer, so it can be passed across FFI boundaries.
- ref of a temporary val will keep it alive during the scope of the ref itself.



## Reference primitive

References point to memory owned by some other value. They are not the same as (raw) pointers. References are guaranteed to be valid.

A reference type is written `&T`, or `&'a type` when you need to specify an explicit lifetime. Copying a reference is a "shallow" operation: it involves only copying the pointer itself. Releasing a reference has no effect on the value it points to, but a reference of a temporary value will keep it alive during the scope of the reference itself.

References are obtained by using `&` or `&mut` operators, which are sort of type constructor: they take a value and a lifetime, producing a (mutable) reference. They can also be obtained by using a `ref` or `ref mut` pattern.

In most cases, refs can be used much like the original value. Field access,method calling, and indexing work the same (save for mutability rules). In addition, the comparison operators transparently defer to the referent's implementation, allowing refs to be compared the same as owned values.


```rust
// &['a ][mut ]T

// shared refs have copy semantics:
let mut int = 42;
let rf = &num;
let rf2 = rf; // copy: now there are 2 shared refs

// shared refs have move semantics:
let mut num = 323;
let mref = &mut num;
let mref2 = mref; // move: `mref` is no more

// ref mut patterns
let ref mut ant = "ness";
let ref mut iny = &ant;
assert_eq!(&iny, &&mut &&mut "ness");
```




Reference primitive type is a structural (non-nominal) type

Reference is a first-class value: it can be moved or copied, stored into data structs, and returned from functions.

A reference is a value that allows indirect access to particular data. The reference is said to refer to the data, and accessing the data is called dereferencing the reference.

```rust
&['a ][mut ]T

&T
&mut T

&'a T
&'a mut T
```



## Shared references (&)
These point to memory owned by some other value. When a shared reference to a value is created it prevents direct mutation of the value. Interior mutability provides an exception for this in certain circumstances. As the name suggests, any number of shared references to a value may exit. A shared reference type is written &type, or &'a type when you need to specify an explicit lifetime. Copying a reference is a "shallow" operation: it involves only copying the pointer itself, that is, pointers are Copy. Releasing a reference has no effect on the value it points to, but referencing of a temporary value will keep it alive during the scope of the reference itself.

## Mutable references (&mut)
These also point to memory owned by some other value. A mutable reference type is written &mut type or &'a mut type. A mutable reference (that hasn't been borrowed) is the only way to access the value it points to, so is not Copy.


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

