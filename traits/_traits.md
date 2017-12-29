# acquired traits

`core::clone::Clone` ability to explicitly duplicate an object.
`std::borrow::Borrow` borrowing data.
`std::borrow::BorrowMut` mutably borrowing data.





# Trait Index


`std::any::Any` A type to emulate dynamic typing.
`std::ascii::AsciiExt` Extension methods for ASCII-subset only operations.
`std::borrow::Borrow` borrowing data.
`std::borrow::BorrowMut` mutably borrowing data.
`std::borrow::ToOwned` generalization of `Clone` to borrowed data.
`std::boxed::FnBox` [LAB] version of the `FnOnce` intended for use with boxed closure objects.
`core::clone::Clone` A common trait for the ability to explicitly duplicate an object.

__std::cmp__
`Eq`         Trait for equality comparisons which are equivalence relations.
`Ord`        Trait for types that form a total order.
`PartialEq`  Trait for equality comparisons which are partial equivalence relations.
`PartialOrd` Trait for values that can be compared for a sort-order.

__std::convert__
`AsMut`   A cheap, mutable reference-to-mutable reference conversion.
`AsRef`   A cheap reference-to-reference conversion. Used to convert a value to a reference value within generic code.
`From`    Simple and safe type conversions in to Self. Reciprocal of `Into`.
`Into`    A conversion that consumes self, which may or may not be expensive. Reciprocal of `From`.
`TryFrom` [LAB] Attempt to construct Self via a conversion.
`TryInto` [LAB] An attempted conversion that consumes self, which may or may not be expensive.

__std::default__
`Default` A trait for giving a type a useful default value.

__std::error__
`Error` Base functionality for all errors in Rust.



__std::fmt__
`std::fmt::Display`   Format trait for the `{}` placeholder.
`std::fmt::Debug`     Format trait for the `{:?}` placeholder.

__core::fmt__
`core::fmt::Write`    Required methods to format a message into a stream.
`core::fmt::Binary`   Format trait for the `b` character.
`core::fmt::Octal`    Format trait for the `o` character.
`core::fmt::LowerHex` Format trait for the `x` character.
`core::fmt::UpperHex` Format trait for the `X` character.
`core::fmt::LowerExp` Format trait for the `e` character.
`core::fmt::UpperExp` Format trait for the `E` character.
`core::fmt::Pointer`  Format trait for the `p` character.


__core::iter__
`Iterator`      An interface for dealing with iterators.
`IntoIterator`  Conversion into an `Iterator`.
`FromIterator`  Conversion from an `Iterator`.
`Sum`           Trait to represent types created by summing up an iterator.
`Product`       Trait to represent types created by multiplying iterator elements.
`Step`          Objects that can be stepped over in both directions.
`Extend`        Extend a collection with the contents of an iterator.
`FusedIterator` An iterator that always continues to yield `None` when exhausted.
`TrustedLen`    An iterator that reports an accurate length using `size_hint`.
`DoubleEndedIterator` An iterator able to yield elements from both ends.
`ExactSizeIterator`   An iterator that knows its exact length.



__std::io__ (`std::io::prelude::*`)
`Read`    Allows for reading bytes from a source.
`Write`   A trait for objects which are byte-oriented sinks.
`Seek`    Provides a cursor which can be moved within a stream of bytes.
`BufRead` Type of Reader which has an internal buffer, allowing it to perform extra ways of reading.



__core::marker__
`Copy`   Types whose values can be duplicated simply by copying bits.
`Send`   Types that can be transferred across thread boundaries.
`Sized`  Types with a constant size known at compile time.
`Sync`   Types for which it is safe to share references between threads.
`Unsize` Types that can be "unsized" to a dynamically-sized type.


__std::hash__
`Hash`        A hashable type.
`Hasher`      A trait for hashing an arbitrary stream of bytes.
`BuildHasher` A trait for creating instances of Hasher.


__std::str__
`FromStr`  A trait to abstract the idea of creating a new instance of a type from a string.

__std::slice__
`SliceConcatExt` [LAB] An extension trait for concatenating slices
`SliceIndex`     [LAB] A helper trait used for indexing operations.


__std::ops__
`Add`           The addition operator `+`.
`AddAssign`     The addition assignment operator `+=`.
`Sub`           The subtraction operator `-`.
`SubAssign`     The subtraction assignment operator `-=`.
`Mul`           The multiplication operator `*`.
`MulAssign`     The multiplication assignment operator `*=`.
`Div`           The division operator `/`.
`DivAssign`     The division assignment operator `/=`.
`Rem`           The remainder operator `%`.
`RemAssign`     The remainder assignment operator `%=`.

`BitAnd`        The bitwise `AND` operator `&`.
`BitAndAssign`  The bitwise `AND` assignment operator `&=`.
`BitOr`         The bitwise `OR` operator `|`.
`BitOrAssign`   The bitwise `OR` assignment operator `|=`.
`BitXor`        The bitwise `XOR` operator `^`.
`BitXorAssign`  The bitwise `XOR` assignment operator `^=`.

`Neg`           The unary negation operator `-`.
`Not`           The unary logical negation operator `!`.

`Shl`           The left shift operator `<<`.
`ShlAssign`     The left shift assignment operator `<<=`.
`Shr`           The right shift operator `>>`.
`ShrAssign`     The right shift assignment operator `>>=`.

`Deref`     Used for immutable dereferencing operations, like `*v`.
`DerefMut`  Used for mutable dereferencing operations, like in `*v = 1`.
`Index`     Used for indexing operations in immutable contexts.
`IndexMut`  Used for indexing operations in mutable contexts.
`Drop`      Used to run some code when a value goes out of scope.

`Fn`     The version of the call operator that takes an immutable receiver.
`FnMut`  The version of the call operator that takes a mutable receiver.
`FnOnce` The version of the call operator that takes a by-value receiver.

`Try`      [LAB] A trait for customizing the behavior of the `?` operator.
`Boxed`    [LAB] Core trait for the box EXPR form.
`BoxPlace` [LAB] Specialization of Place trait supporting box EXPR.
`Placer`   [LAB] Interface to implementations of PLACE <- EXPR.
`InPlace`  [LAB] Specialization of Place trait supporting PLACE <- EXPR.

`CoerceUnsized` [LAB] Trait that indicates that this is a pointer or a wrapper 
                for one, where unsizing can be performed on the pointee.

`Place` [LAB] Both PLACE <- EXPR and box EXPR desugar into expressions that 
        allocate an intermediate "place" that holds uninitialized state. The 
        desugaring evaluates EXPR, and writes the result at the address returned 
        by the pointer method of this trait.

