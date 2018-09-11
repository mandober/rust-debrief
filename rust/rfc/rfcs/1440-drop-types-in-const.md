- Feature Name: `drop_types_in_const`
- Start Date: 2016-01-01
- RFC PR: [rust-lang/rfcs#1440](https://github.com/rust-lang/rfcs/pull/1440)
- Rust Issue: [rust-lang/rust#33156](https://github.com/rust-lang/rust/issues/33156)

# Summary
[summary]: #summary

Allow types with destructors to be used in `static` items, `const` items, and `const` functions.

# Motivation
[motivation]: #motivation

Some of the collection types do not allocate any memory when constructed empty (most notably `Vec`). With the change to make leaking safe, the restriction on `static` or `const` items with destructors
is no longer required to be a hard error (as it is safe and accepted that these destructors may never run).

Allowing types with destructors to be directly used in `const` functions and stored in `static`s or `const`s will remove the need to have
runtime-initialisation for global variables.

# Detailed design
[design]: #detailed-design

- Lift the restriction on types with destructors being used in `static` or `const` items.
 - `static`s containing Drop-types will not run the destructor upon program/thread exit.
 - `const`s containing Drop-types _will_ run the destructor at the appropriate point in the program.
 - (Optionally adding a lint that warn about the possibility of resource leak)
- Alloc instantiating structures with destructors in constant expressions,
- Allow `const fn` to return types with destructors.
- Disallow constant expressions that require destructors to run during compile-time constant evaluation (i.e: a `drop(foo)` in a `const fn`).

## Examples
Assuming that `RwLock` and `Vec` have `const fn new` methods, the following example is possible and avoids runtime validity checks.

```rust
/// Logging output handler
trait LogHandler: Send + Sync {
    // ...
}
/// List of registered logging handlers
static S_LOGGERS: RwLock<Vec< Box<LogHandler> >> = RwLock::new( Vec::new() );

/// Just an empty byte vector.
const EMPTY_BYTE_VEC: Vec<u8> = Vec::new();
```

Disallowed code
```rust
static VAL: usize = (Vec::<u8>::new(), 0).1;	// The `Vec` would be dropped

const fn sample(_v: Vec<u8>) -> usize {
	0	// Discards the input vector, dropping it
}
```

# Drawbacks
[drawbacks]: #drawbacks

Destructors do not run on `static` items (by design), so this can lead to unexpected behavior when a type's destructor has effects outside the program (e.g. a RAII temporary folder handle, which deletes the folder on drop). However, this can already happen using the `lazy_static` crate.

A `const` item's destructor _will_ run at each point where the `const` item is used. If a `const` item is never used, its destructor will never run. These behaviors may be unexpected.

# Alternatives
[alternatives]: #alternatives

- Runtime initialisation of a raw pointer can be used instead (as the `lazy_static` crate currently does on stable)
- On nightly, a bug related to `static` and `UnsafeCell<Option<T>>` can be used to remove the dynamic allocation.
 - Both of these alternatives require runtime initialisation, and incur a checking overhead on subsequent accesses.
- Leaking of objects could be addressed by using C++-style `.dtors` support
 - This is undesirable, as it introduces confusion around destructor execution order.

# Unresolved questions
[unresolved]: #unresolved-questions

- TBD
