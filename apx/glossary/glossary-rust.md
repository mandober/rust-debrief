# Rust-centric glossary

<!-- TOC -->

- [Bikeshed](#bikeshed)
- [Blanket implementations](#blanket-implementations)
- [Bors](#bors)
- [Dynamically sized type](#dynamically-sized-type)
- [Fat pointer](#fat-pointer)
- [ICE](#ice)
- [Interior mutability](#interior-mutability)
- [Marker interfaces](#marker-interfaces)
- [Newtype](#newtype)
- [Phantom data](#phantom-data)
- [Phantom types](#phantom-types)
- [Raw pointer](#raw-pointer)
- [Rust](#rust)
- [Slice](#slice)
- [Trait object](#trait-object)
- [Zero sized type](#zero-sized-type)

<!-- /TOC -->


## Bikeshed
A highly engaged discussion about a non-fundamental, even trivial, aspect of the Rust language.

## Blanket implementations
Conditionally implementing a trait for a type that implements some other specific trait. Implementation of a trait on any type that satisfies the trait bounds are called blanket implementations. For example, the standard library implements `ToString` trait on any type that implements `Display` trait.

## Bors
`bors` is a continuous integration bot on github in service of Rust. It's a script that runs tests on a, previously reviewed, pull request, merging it if all tests pass successfully.

## Dynamically sized type
Dynamically Sized Types (DST) are types without statically known size or alignment. Due to this, these types can only exist behind some kind of pointer. A pointer to a DST is a fat (multi-worded) pointer, consisting of a proper pointer and additional data that _completes_ it. Two frequently used DSTs are **trait objects** and **slices**.

## Fat pointer
A pointer with accompanying extra information. It comprises a pointer and one or more associated fields that "complete" the pointer. For example, a string is a fat pointer made up of pointer to some data on the heap, a length (number of characters it points to) and a capacity (additional space for characters reserved).

## ICE
Internal compiler error: an internal assertion failure in the compiler, which always indicates a bug in the compiler.

## Interior mutability
Interior mutability is a pattern where an immutable type exposes an API for mutating an interior value, and the borrowing rules apply at runtime instead of compile time.

## Marker interfaces
Marker interfaces contain no methods at all and serve to provide run-time information to generic processing using reflection. In Rust, marker interface is realized through marker traits: Copy, Sized, Send, Sync.

## Newtype
A tuple structure with a single unnamed field. Used to create type wrappers. For example: `struct Meter(i32)`. 

## Phantom data
Zero-sized type used to mark things that "act like" they own a `T`. Adding a `PhantomData<T>` field to your type tells the compiler that your type acts as though it stores a value of type `T`, even though it doesn't really. This information is used when computing certain safety properties. More about [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).

## Phantom types
Phantom data and phantom types are related, but not identical. A phantom type parameter is simply a type parameter which is never used. In Rust, this often causes the compiler to complain, and the solution is to add a dummy use by way of [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).

## Raw pointer
A pointer is a variable that contains the memory address of some value. To access the value it points to, the pointer is dereferenced. In Rust, these kind of pointers are called raw pointers; there is immutable raw pointer, `*const T` and mutable raw pointer, `*mut T`.

## Rust
According to one version, the Rust language gets its name after a fungi that is very robust, distributed (as opposed to a non-single-cellular), with a parallel reproduction. Now, hold the adjectives, drop the rest.

## Slice
A slice is a view into some contiguous storage. Slice is a fat pointer; the information that completes a slice is the number of elements it points to.

## Trait object
Dynamically sized type that implements some trait. The original type is _erased_ in favor of runtime reflection, with a _vtable_ containing all the information necessary to use the type. The information that "completes" a trait object (trait object is a fat pointer) is a pointer to its vtable.

## Zero sized type
ZST are types that occupy no space (e.g. empty tuple, empty array).
