# Trait index by module

- any
  - `Any` type to emulate dynamic typing.
- ascii
  - `AsciiExt` methods for ASCII only operations.
- borrow
  - `Borrow` borrowing data.
  - `BorrowMut` mutably borrowing data.
  - `ToOwned` generalization of `Clone` to borrowed data.
- boxed
  - `FnBox` version of `FnOnce` for use with boxed closure objects. __LAB__
- cmp
  - `Eq` equivalence relations.
  - `Ord` types that form a total order.
  - `PartialEq` partial equivalence relations.
  - `PartialOrd` values that can be compared for a sort-order.
- convert
  - `AsRef` cheap to ref conversion.
  - `AsMut` cheap, to mutable reference conversion.
  - `From` simple and safe type conversions into Self.
  - `Into` consumes self, (maybe expensive).
  - `TryFrom` try to construct Self via a conversion. __LAB__
  - `TryInto` try conversion, consumes self, (maybe expensive). __LAB__
- default
  - `Default` type's default value.
- error
  - `Error` base functionality for all errors.
- fmt
  - `Display` format trait for an empty format, `{}`.
  - `Write` required methods to format a message into a stream.
  - `Debug` `?` formatting.
  - `Binary` `b` formatting.
  - `Octal` `o` formatting.
  - `LowerExp` `e` formatting.
  - `UpperExp` `E` formatting.
  - `LowerHex` `x` formatting.
  - `UpperHex` `X` formatting.
  - `Pointer` `p` formatting.
- core::iter
  - `Iterator`      An interface for dealing with iterators.
  - `IntoIterator`  Conversion into an `Iterator`.
  - `FromIterator`  Conversion from an `Iterator`.
  - `Sum`           types created by summing up an iterator.
  - `Product`       types created by multiplying iterator elements.
  - `Step`          Objects that can be stepped over in both directions.
  - `Extend`        Extend a collection with the contents of an iterator.
  - `FusedIterator` continues to yield `None` when exhausted.
  - `TrustedLen`    An iterator that reports an accurate length using `size_hint`.
  - `DoubleEndedIterator` An iterator able to yield elements from both ends.
  - `ExactSizeIterator`   An iterator that knows its exact length.
- core::clone
  - `Clone` ability to explicitly duplicate an object.
- core::marker
  - `Copy`   Types whose values can be duplicated simply by copying bits.
  - `Send`   Types that can be transferred across thread boundaries.
  - `Sized`  Types with a constant size known at compile time.
  - `Sync`   Types for which it is safe to share references between threads.
  - `Unsize` Types that can be "unsized" to a dynamically-sized type.
- std::io (`std::io::prelude::*`)
  - `Read`    Allows for reading bytes from a source.
  - `Write`   A trait for objects which are byte-oriented sinks.
  - `Seek`    Provides a cursor which can be moved within a stream of bytes.
  - `BufRead` Type of Reader which has an internal buffer, allowing it to perform extra ways of reading.
- hash
  - `Hash`        A hashable type.
  - `Hasher`      A trait for hashing an arbitrary stream of bytes.
  - `BuildHasher` A trait for creating instances of Hasher.
- str
  - `FromStr`  A trait to abstract the idea of creating a new instance of a type from a string.
- slice
  - `SliceConcatExt` [LAB] An extension trait for concatenating slices
  - `SliceIndex`     [LAB] A helper trait used for indexing operations.
- ops
  - `Add`           The addition operator `+`.
  - `AddAssign`     The addition assignment operator `+=`.
  - `Sub`           The subtraction operator `-`.
  - `SubAssign`     The subtraction assignment operator `-=`.
  - `Mul`           The multiplication operator `*`.
  - `MulAssign`     The multiplication assignment operator `*=`.
  - `Div`           The division operator `/`.
  - `DivAssign`     The division assignment operator `/=`.
  - `Rem`           The remainder operator `%`.
  - `RemAssign`     The remainder assignment operator `%=`.
  - `BitAnd`        The bitwise `AND` operator `&`.
  - `BitAndAssign`  The bitwise `AND` assignment operator `&=`.
  - `BitOr`         The bitwise `OR` operator `|`.
  - `BitOrAssign`   The bitwise `OR` assignment operator `|=`.
  - `BitXor`        The bitwise `XOR` operator `^`.
  - `BitXorAssign`  The bitwise `XOR` assignment operator `^=`.
  - `Neg`           The unary negation operator `-`.
  - `Not`           The unary logical negation operator `!`.
  - `Shl`           The left shift operator `<<`.
  - `ShlAssign`     The left shift assignment operator `<<=`.
  - `Shr`           The right shift operator `>>`.
  - `ShrAssign`     The right shift assignment operator `>>=`.
  - `Deref`     Used for immutable dereferencing operations, like `*v`.
  - `DerefMut`  Used for mutable dereferencing operations, like in `*v = 1`.
  - `Index`     Used for indexing operations in immutable contexts.
  - `IndexMut`  Used for indexing operations in mutable contexts.
  - `Drop`      Used to run some code when a value goes out of scope.
  - `Fn`     The version of the call operator that takes an immutable receiver.
  - `FnMut`  The version of the call operator that takes a mutable receiver.
  - `FnOnce` The version of the call operator that takes a by-value receiver.
  - `Try`      [LAB] A trait for customizing the behavior of the `?` operator.
  - `Boxed`    [LAB] Core trait for the box EXPR form.
  - `BoxPlace` [LAB] Specialization of Place trait supporting box EXPR.
  - `Placer`   [LAB] Interface to implementations of PLACE <- EXPR.
  - `InPlace`  [LAB] Specialization of Place trait supporting PLACE <- EXPR.
  - `CoerceUnsized` [LAB] Trait that indicates that this is a pointer or a wrapper for one, where unsizing can be performed on the pointee.
  - `Place` [LAB] Both PLACE <- EXPR and box EXPR desugar into expressions that allocate an intermediate "place" that holds uninitialized state. The desugaring evaluates EXPR, and writes the result at the address returned by the pointer method of this trait.
