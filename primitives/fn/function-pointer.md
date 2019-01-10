# Primitive: `fn` function pointer

https://doc.rust-lang.org/std/primitive.fn.html

Primitive Type `fn` 1.0.0

Function pointer, as in: `fn(i32, i32) -> i32`

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

let fnptr: fn(i32, i32) -> i32 = add;
```

Function pointers vary by signature, ABI and safety:
- Function pointers vary based on their signature
- whether they point to safe or unsafe functions
- based on what ABI they use

Function pointers, `fn`, is not to be confused with `Fn` trait.


## Acquiring function pointers

Plain function pointers are obtained by *casting* either plain
functions, or closures that don't capture an environment:

```rust
fn add_one(x: usize) -> usize {
    x + 1
}

let ptr: fn(usize) -> usize = add_one;
assert_eq!(ptr(5), 6);

let clos: fn(usize) -> usize = |x| x + 5;
assert_eq!(clos(5), 10);
```


## Safe and `unsafe` function pointers

In addition to varying based on their signature,
function pointers come in two flavors:
- _safe function pointers_
  plain `fn()` function pointers can only point to safe functions
- _unsafe function pointers_
  `unsafe fn()` function pointers can point to safe or unsafe functions.

```rust
fn add_one(x: usize) -> usize {
    x + 1
}

unsafe fn add_one_unsafely(x: usize) -> usize {
    x + 1
}

let safe_ptr: fn(usize) -> usize = add_one;

let bad_ptr: fn(usize) -> usize = add_one_unsafely;
// ERROR: mismatched types: expected normal fn, found unsafe fn

let unsafe_ptr: unsafe fn(usize) -> usize = add_one_unsafely;
let really_safe_ptr: unsafe fn(usize) -> usize = add_one;
```


## ABI variations

On top of that, function pointers can vary based on what ABI they use.

This is achieved by adding the `extern` keyword
to the *type name*, followed by the ABI in question.

For example,
`fn()` is different from 
`extern "C" fn()`, which itself is different from 
`extern "stdcall" fn()`, and so on for the various ABIs that Rust supports. 

Non-extern functions have an ABI of `"Rust"`, and `extern` functions 
without an explicit ABI have an ABI of `"C"`.

[ffi](https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions)


```rust
// type annotations only:

// plain fn
fn(u8) -> bool

// unsafe fn
unsafe fn(usize) -> usize

// non-extern function
fn(usize) -> usize
// non-extern function have an ABI of "Rust".
// Forcibly annotated it'd look like:
extern "Rust" fn(usize) -> usize

// extern functions without an explicit ABI...
extern fn(usize) -> usize
// ...have ABI of "C"
extern "C" fn(usize) -> usize

extern "stdcall" fn(usize) -> usize
```

Extern function declarations with the `"C"` or `"cdecl"` ABIs can also be
variadic, allowing them to be called with a variable number of arguments.
Normal rust functions, even those with an extern "ABI", cannot be variadic. 
For more information, see the nomicon's section on variadic functions:
[variadic](https://doc.rust-lang.org/nomicon/ffi.html#variadic-functions)

These markers can be combined, so unsafe extern "stdcall" fn() is a valid type.

Like references in rust, function pointers are assumed to not be null, so if you
want to pass a function pointer over FFI and be able to accommodate null pointers,
make your type `Option<fn()>` with your required signature.


## Traits

Function pointers implement the following traits:
- `Clone`
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`
- `Hash`
- `Pointer`
- `Debug`

Due to a temporary restriction in Rust's type system, these traits are only
implemented on functions that take *12 arguments or less*, with the `"Rust"`
and `"C"` ABIs. In the future, this may change.

In addition, function pointers of any signature, ABI, or safety are `Copy`,
and all safe function pointers implement `Fn`, `FnMut`, and `FnOnce`.

This works because these traits are specially known to the compiler.
