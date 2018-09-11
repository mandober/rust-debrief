- Array: `[T; N]`
- contiguous, fixed-size, sequence of **homogenous** elements.
- primitive, structural, compound, sequential, generic
- elements on the stack (if primitives), otherwise on the heap
- literal array expression
- type annotation: `[T; N]`
  - `N` is the length of the array and a __part of its type__.
  - `N` is a non-negative compile-time constant integer
  - `N` is in the range `isize::MAX >= N >= 0`
  - although `N` is `usize`, in practice it can't be larger than `isize::MAX`
  - traits are implemented only for `32 >= N >= 0`
- coerces to a slice; all methods are on slices. Not iterable, slice is.
- array of any size is `Copy` if the elements' type is `Copy`
- impossible to move elements out of the array (use `mem::replace`)
- indexing is bounds-checked at run-time, out-of-bounds indexing causes panic
- online std docs: [primitive.array](https://doc.rust-lang.org/std/primitive.array.html) (no std module)


