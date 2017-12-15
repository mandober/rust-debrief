unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> String[src]
[−]

Creates a new String from a length, capacity, and pointer.
Safety

This is highly unsafe, due to the number of invariants that aren't checked:

    The memory at ptr needs to have been previously allocated by the same allocator the standard library uses.
    length needs to be less than or equal to capacity.
    capacity needs to be the correct value.

Violating these may cause problems like corrupting the allocator's internal data structures.

The ownership of ptr is effectively transferred to the String which may then deallocate, reallocate or change the contents of memory pointed to by the pointer at will. Ensure that nothing else uses the pointer after calling this function.
Examples

Basic usage:

use std::mem;

unsafe {
    let s = String::from("hello");
    let ptr = s.as_ptr();
    let len = s.len();
    let capacity = s.capacity();

    mem::forget(s);

    let s = String::from_raw_parts(ptr as *mut _, len, capacity);

    assert_eq!(String::from("hello"), s);
}