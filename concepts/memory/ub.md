# Undefined Behavior

`pub unsafe fn offset(self, count: isize) -> *const T`
Calculates the offset from a pointer.

If any of the following conditions are violated, the result is Undefined Behavior:
- Both the starting and resulting pointer must be either in bounds or one byte past the end of an allocated object.
- The computed offset, in bytes, cannot overflow an isize.
- The offset being in bounds cannot rely on "wrapping around" the address space. That is, the infinite-precision sum, in bytes must fit in a usize.

The compiler and standard library generally tries to ensure allocations never reach a size where an offset is a concern. For instance, Vec and Box ensure they never allocate more than isize::MAX bytes, so vec.as_ptr().offset(vec.len() as isize) is always safe.

Most platforms fundamentally can't even construct such an allocation. For instance, no known 64-bit platform can ever serve a request for 263 bytes due to page-table limitations or splitting the address space. However, some 32-bit and 16-bit platforms may successfully serve a request for more than isize::MAX bytes with things like Physical Address Extension. As such, memory acquired directly from allocators or memory mapped files may be too large to handle with this function.

Consider using wrapping_offset instead if these constraints are difficult to satisfy. The only advantage of this method is that it enables more aggressive compiler optimizations.


---

https://doc.rust-lang.org/nightly/std/primitive.pointer.html
