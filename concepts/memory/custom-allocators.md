# Custom Allocators

The compiler currently ships two default allocators, available as crates, `alloc_system` and `alloc_jemalloc`, although some systems don't have jemalloc. 

They contain routines to allocate and deallocate memory. The std doesn't assume either and the compiler will pick one depending on the type of output. Namely, binaries will use `alloc_jemalloc` if available. Dynamic and static libraries will use `alloc_system` by default, using the standard API (malloc, free).

Overriding the compiler's allocator preference by linking to another:

```rust
#![feature(alloc_system)]
extern crate alloc_system;
// allocates from the system allocator
let _a = Box::new(4);
```

Here the binary will not link to jemalloc as it would by default, but to specified system allocator. Conversely, a dynamic library which uses jemalloc by default would instead use the specified allocator, in this case the system's:

```rust
#![feature(alloc_jemalloc)]
#![crate_type = "dylib"]
extern crate alloc_jemalloc;
// allocates from jemalloc
let _a = Box::new(4);
```

To code a custom allocator, one has to implement the allocator API for that new allocator. This example shows a simplified version of `alloc_system`:

```rust
// instruct the compiler this crate is an allocator, so when this is linked in,
// another allocator (like jemalloc) should not be also linked in
#![feature(allocator)]
#![allocator]

// Allocators must not depend on std. On the other hand, std requires an 
// allocator to avoid circular dependencies. This crate can use core (libcore)
#![no_std]

// unique name for this custom allocator
#![crate_name = "my_allocator"]
#![crate_type = "rlib"]

// Our system allocator will use the in-tree libc crate for FFI bindings. Note
// that currently the external (crates.io) libc cannot be used because it links
// to the standard library (e.g. `#![no_std]` isn't stable yet), so that's why
// this specifically requires the in-tree version.
#![feature(libc)]
extern crate libc;


// Listed below are the five allocation functions currently required by custom
// allocators. Their signatures and symbol names are not currently typechecked
// by the compiler, but this is a future extension and are required to match
// what is found below.
//
// Note that the standard `malloc` and `realloc` functions do not provide a way
// to communicate alignment so this implementation would need to be improved
// with respect to alignment in that aspect.

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { libc::malloc(size as libc::size_t) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe { libc::free(ptr as *mut libc::c_void) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        libc::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by libc
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
```


After we compile this crate, it can be used as follows:

```rust
extern crate my_allocator;
// allocates memory via our custom allocator crate
let _a = Box::new(8);
```


Custom allocator limitations

There are a few restrictions when working with custom allocators which may cause compiler errors:

Any one artifact may only be linked to at most one allocator. Binaries, dylibs, and staticlibs must link to exactly one allocator, and if none have been explicitly chosen the compiler will choose one. On the other hand rlibs do not need to link to an allocator (but still can).

A consumer of an allocator is tagged with `#![needs_allocator]` (e.g. the liballoc crate currently) and an `#[allocator]` crate cannot transitively depend on a crate which needs an allocator (e.g. circular dependencies are not allowed). This basically means that allocators must restrict themselves to libcore currently.
