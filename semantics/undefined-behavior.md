# Undefined Behavior (UB)

Rust code, including the code within `unsafe` blocks and `unsafe` functions, is incorrect if it exhibits any of the behaviors in the following list. It is the programmer's responsibility when writing unsafe code to make it impossible for safe code to exhibit these behaviors.

- Data races
- Dereferencing a null or dangling raw pointer
- Unaligned pointer reading and writing outside of [`read_unaligned`][2] and [`write_unaligned`][3].
- Reads of [undef][4] (uninitialized) memory.
- Breaking the [pointer aliasing rules][5] on accesses through raw pointers; a subset of the rules used by C.
- `&mut T` and `&T` follow LLVM's scoped [noalias][6] model, except if the `&T` contains an [`UnsafeCell`][7].
- Mutating non-mutable data i.e. data reached through a shared reference or data owned by a `let` binding), unless that data is contained within an [`UnsafeCell`][7].
- Invoking undefined behavior via compiler intrinsics:
  - Indexing outside of the bounds of an object with [`offset`][8] with the exception of one byte past the end of the object.
  - Using [`std::ptr::copy_nonoverlapping_memory`][9], aka the `memcpy32`and `memcpy64` intrinsics, on overlapping buffers.
- Invalid values in primitive types, even in private fields and locals: 
  - Dangling or null references and boxes.
  - A value other than `false` (0) or `true` (1) in a `bool`.
  - A discriminant in an `enum` not included in the type definition.
  - A value in a `char` which is a surrogate or above `char::MAX`.
  - Non-UTF-8 byte sequences in a `str`.


[2]: https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html
[3]: https://doc.rust-lang.org/std/ptr/fn.write_unaligned.html
[4]: http://llvm.org/docs/LangRef.html#undefined-values
[5]: http://llvm.org/docs/LangRef.html#pointer-aliasing-rules
[6]: http://llvm.org/docs/LangRef.html#noalias
[7]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
[8]: https://doc.rust-lang.org/std/primitive.pointer.html#method.offset
[9]: https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html
