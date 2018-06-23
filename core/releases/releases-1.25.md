# Version 1.25.0

Rust version 1.25.0, released 2018-03-29

## Language
- [Stabilised `#[repr(align(x))]`](https://github.com/rust-lang/rust/pull/47006), [RFC 1358](https://github.com/rust-lang/rfcs/pull/1358)
- [You can now use nested groups of imports.][47948]
  e.g. `use std::{fs::File, io::Read, path::{Path, PathBuf}};`
- [You can now have `|` at the start of a match arm.][47947] e.g.


```rust
enum Foo { A, B, C }

fn main() {
    let x = Foo::A;
    match x {
        | Foo::A
        | Foo::B => println!("AB"),
        | Foo::C => println!("C"),
    }
}
```

## Compiler
- [Upgraded to LLVM 6.][47828]
- [Added `-C lto=val` option.][47521]
- [Added `i586-unknown-linux-musl` target][47282]

## Libraries
- [Impl Send for `process::Command` on Unix.][47760]
- [Impl PartialEq and Eq for `ParseCharError`.][47790]
- [`UnsafeCell::into_inner` is now safe.][47204]
- [Implement libstd for CloudABI.][47268]
- [`Float::{from_bits, to_bits}` is now available in libcore.][46931]
- [Implement `AsRef<Path>` for Component][46985]
- [Implemented `Write` for `Cursor<&mut Vec<T>>`][46830]
- [Moved `Duration` to libcore.][46666]


## Stabilized APIs
- [`Location::column`]
- [`ptr::NonNull`]

The following functions can now be used in a constant expression.
eg. `static MINUTE: Duration = Duration::from_secs(60);`
- [`Duration::new`][47300]
- [`Duration::from_secs`][47300]
- [`Duration::from_millis`][47300]
- [`Duration::from_micros`][47300]
- [`Duration::from_nanos`][47300]


## Cargo
- [`cargo new` no longer removes `rust` or `rs` prefixs/suffixs.][cargo/5013]
- [`cargo new` now defaults to creating a binary crate, instead of a library crate.](https://github.com/rust-lang/cargo/pull/5029)


## Misc
- [Rust by example is now shipped with new releases][46196]

## Compatibility Notes
- [Deprecated `net::lookup_host`.][47510]
- [`rustdoc` has switched to pulldown as the default markdown renderer.][47398]
- The borrow checker was sometimes incorrectly permitting overlapping borrows around indexing operations (see [#47349][47349]). This has been fixed (which also enabled some correct code that used to cause errors (e.g. [#33903][33903] and [#46095][46095]).
- [Removed deprecated unstable attribute `#[simd]`.][47251]



[33903]: https://github.com/rust-lang/rust/pull/33903
[47947]: https://github.com/rust-lang/rust/pull/47947
[47948]: https://github.com/rust-lang/rust/pull/47948
[47760]: https://github.com/rust-lang/rust/pull/47760
[47790]: https://github.com/rust-lang/rust/pull/47790
[47828]: https://github.com/rust-lang/rust/pull/47828
[47398]: https://github.com/rust-lang/rust/pull/47398
[47510]: https://github.com/rust-lang/rust/pull/47510
[47521]: https://github.com/rust-lang/rust/pull/47521
[47204]: https://github.com/rust-lang/rust/pull/47204
[47251]: https://github.com/rust-lang/rust/pull/47251
[47268]: https://github.com/rust-lang/rust/pull/47268
[47282]: https://github.com/rust-lang/rust/pull/47282
[47300]: https://github.com/rust-lang/rust/pull/47300
[47349]: https://github.com/rust-lang/rust/pull/47349
[46931]: https://github.com/rust-lang/rust/pull/46931
[46985]: https://github.com/rust-lang/rust/pull/46985
[47006]: https://github.com/rust-lang/rust/pull/47006
[46830]: https://github.com/rust-lang/rust/pull/46830
[46095]: https://github.com/rust-lang/rust/pull/46095
[46666]: https://github.com/rust-lang/rust/pull/46666
[46196]: https://github.com/rust-lang/rust/pull/46196
[cargo/5013]: https://github.com/rust-lang/cargo/pull/5013
[cargo/5029]: https://github.com/rust-lang/cargo/pull/5029
[RFC 1358]: https://github.com/rust-lang/rfcs/pull/1358
[`Location::column`]: https://doc.rust-lang.org/std/panic/struct.Location.html#method.column
[`ptr::NonNull`]: https://doc.rust-lang.org/std/ptr/struct.NonNull.html

