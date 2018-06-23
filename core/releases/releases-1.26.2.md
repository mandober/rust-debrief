Version 1.26.2 (2018-06-05)
==========================

Compatibility Notes
-------------------

- [The borrow checker was fixed to avoid unsoundness when using match ergonomics][51117]

[51117]: https://github.com/rust-lang/rust/issues/51117





Version 1.26.1 (2018-05-29)
==========================

Tools
-----

- [RLS now works on Windows][50646]
- [Rustfmt stopped badly formatting text in some cases][rustfmt/2695]

Compatibility Notes
--------

- [`fn main() -> impl Trait` no longer works for non-Termination
  trait][50656]
  This reverts an accidental stabilization.
- [`NaN > NaN` no longer returns true in const-fn contexts][50812]
- [Prohibit using turbofish for `impl Trait` in method arguments][50950]

[50646]: https://github.com/rust-lang/rust/issues/50646
[50656]: https://github.com/rust-lang/rust/pull/50656
[50812]: https://github.com/rust-lang/rust/pull/50812
[50950]: https://github.com/rust-lang/rust/issues/50950
[rustfmt/2695]: https://github.com/rust-lang-nursery/rustfmt/issues/2695




Version 1.26.0 (2018-05-10)
==========================

Language
--------
- [Closures now implement `Copy` and/or `Clone` if all captured variables
  implement either or both traits.][49299]
- [The inclusive range syntax e.g. `for x in 0..=10` is now stable.][47813]
- [Stablise `'_`. The underscore lifetime can be used anywhere where a
  lifetime can be elided.][49458]
- [`impl Trait` is now stable allowing you to have abstract types in returns
   or in function parameters.][49255] e.g. `fn foo() -> impl Iterator<Item=u8>` or
  `fn open(path: impl AsRef<Path>)`.
- [Pattern matching will now automatically apply dereferences.][49394]
- [128-bit integers in the form of `u128` and `i128` are now stable.][49101]
- [`main` can now return `Result<(), E: Debug>`][49162] in addition to `()`.
- [A lot of operations are now available in a const context.][46882] E.g. You
  can now index into constant arrays, reference and dereference into constants,
  and use Tuple struct constructors.
- [Fixed entry slice patterns are now stable.][48516] e.g.
  ```rust
  let points = [1, 2, 3, 4];
  match points {
      [1, 2, 3, 4] => println!("All points were sequential."),
      _ => println!("Not all points were sequential."),
  }
  ```


Compiler
--------
- [LLD is now used as the default linker for `wasm32-unknown-unknown`.][48125]
- [Fixed exponential projection complexity on nested types.][48296]
  This can provide up to a ~12% reduction in compile times for certain crates.
- [Added the `--remap-path-prefix` option to rustc.][48359] Allowing you
  to remap path prefixes outputted by the compiler.
- [Added `powerpc-unknown-netbsd` target.][48281]

Libraries
---------
- [Implemented `From<u16> for usize` & `From<{u8, i16}> for isize`.][49305]
- [Added hexadecimal formatting for integers with fmt::Debug][48978]
  e.g. `assert!(format!("{:02x?}", b"Foo\0") == "[46, 6f, 6f, 00]")`
- [Implemented `Default, Hash` for `cmp::Reverse`.][48628]
- [Optimized `str::repeat` being 8x faster in large cases.][48657]
- [`ascii::escape_default` is now available in libcore.][48735]
- [Trailing commas are now supported in std and core macros.][48056]
- [Implemented `Copy, Clone` for `cmp::Reverse`][47379]
- [Implemented `Clone` for `char::{ToLowercase, ToUppercase}`.][48629]

Stabilized APIs
---------------
- [`*const T::add`]
- [`*const T::copy_to_nonoverlapping`]
- [`*const T::copy_to`]
- [`*const T::read_unaligned`]
- [`*const T::read_volatile`]
- [`*const T::read`]
- [`*const T::sub`]
- [`*const T::wrapping_add`]
- [`*const T::wrapping_sub`]
- [`*mut T::add`]
- [`*mut T::copy_to_nonoverlapping`]
- [`*mut T::copy_to`]
- [`*mut T::read_unaligned`]
- [`*mut T::read_volatile`]
- [`*mut T::read`]
- [`*mut T::replace`]
- [`*mut T::sub`]
- [`*mut T::swap`]
- [`*mut T::wrapping_add`]
- [`*mut T::wrapping_sub`]
- [`*mut T::write_bytes`]
- [`*mut T::write_unaligned`]
- [`*mut T::write_volatile`]
- [`*mut T::write`]
- [`Box::leak`]
- [`FromUtf8Error::as_bytes`]
- [`LocalKey::try_with`]
- [`Option::cloned`]
- [`btree_map::Entry::and_modify`]
- [`fs::read_to_string`]
- [`fs::read`]
- [`fs::write`]
- [`hash_map::Entry::and_modify`]
- [`iter::FusedIterator`]
- [`ops::RangeInclusive`]
- [`ops::RangeToInclusive`]
- [`process::id`]
- [`slice::rotate_left`]
- [`slice::rotate_right`]
- [`String::retain`]


Cargo
-----
- [Cargo will now output path to custom commands when `-v` is
  passed with `--list`][cargo/5041]
- [The Cargo binary version is now the same as the Rust version][cargo/5083]
- [`Cargo.lock` files are now included in published crates.][cargo/5093]

Misc
----
- [The second edition of "The Rust Programming Language" book is now recommended
  over the first.][48404]

Compatibility Notes
-------------------

- [aliasing a `Fn` trait as `dyn` no longer works.][48481] E.g. the following
  syntax is now invalid.
  ```
  use std::ops::Fn as dyn;
  fn g(_: Box<dyn(std::fmt::Debug)>) {}
  ```
- [The result of dereferences are no longer promoted to `'static`.][47408]
  e.g.
  ```rust
  fn main() {
      const PAIR: &(i32, i32) = &(0, 1);
      let _reversed_pair: &'static _ = &(PAIR.1, PAIR.0); // Doesn't work
  }
  ```
- [Deprecate `AsciiExt` trait in favor of inherent methods.][49109]
- [`".e0"` will now no longer parse as `0.0` and will instead cause
  an error.][48235]
- [Removed hoedown from rustdoc.][48274]
- [Bounds on higher-kinded lifetimes a hard error.][48326]

[46882]: https://github.com/rust-lang/rust/pull/46882
[47379]: https://github.com/rust-lang/rust/pull/47379
[47408]: https://github.com/rust-lang/rust/pull/47408
[47813]: https://github.com/rust-lang/rust/pull/47813
[48056]: https://github.com/rust-lang/rust/pull/48056
[48125]: https://github.com/rust-lang/rust/pull/48125
[48166]: https://github.com/rust-lang/rust/pull/48166
[48235]: https://github.com/rust-lang/rust/pull/48235
[48274]: https://github.com/rust-lang/rust/pull/48274
[48281]: https://github.com/rust-lang/rust/pull/48281
[48296]: https://github.com/rust-lang/rust/pull/48296
[48326]: https://github.com/rust-lang/rust/pull/48326
[48359]: https://github.com/rust-lang/rust/pull/48359
[48404]: https://github.com/rust-lang/rust/pull/48404
[48481]: https://github.com/rust-lang/rust/pull/48481
[48516]: https://github.com/rust-lang/rust/pull/48516
[48628]: https://github.com/rust-lang/rust/pull/48628
[48629]: https://github.com/rust-lang/rust/pull/48629
[48657]: https://github.com/rust-lang/rust/pull/48657
[48735]: https://github.com/rust-lang/rust/pull/48735
[48978]: https://github.com/rust-lang/rust/pull/48978
[49101]: https://github.com/rust-lang/rust/pull/49101
[49109]: https://github.com/rust-lang/rust/pull/49109
[49121]: https://github.com/rust-lang/rust/pull/49121
[49162]: https://github.com/rust-lang/rust/pull/49162
[49184]: https://github.com/rust-lang/rust/pull/49184
[49234]: https://github.com/rust-lang/rust/pull/49234
[49255]: https://github.com/rust-lang/rust/pull/49255
[49299]: https://github.com/rust-lang/rust/pull/49299
[49305]: https://github.com/rust-lang/rust/pull/49305
[49394]: https://github.com/rust-lang/rust/pull/49394
[49458]: https://github.com/rust-lang/rust/pull/49458
[`*const T::add`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.add
[`*const T::copy_to_nonoverlapping`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_to_nonoverlapping
[`*const T::copy_to`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_to
[`*const T::read_unaligned`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned
[`*const T::read_volatile`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile
[`*const T::read`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read
[`*const T::sub`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.sub
[`*const T::wrapping_add`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add
[`*const T::wrapping_sub`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub
[`*mut T::add`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.add-1
[`*mut T::copy_to_nonoverlapping`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_to_nonoverlapping-1
[`*mut T::copy_to`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_to-1
[`*mut T::read_unaligned`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned-1
[`*mut T::read_volatile`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile-1
[`*mut T::read`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.read-1
[`*mut T::replace`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.replace
[`*mut T::sub`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.sub-1
[`*mut T::swap`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.swap
[`*mut T::wrapping_add`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add-1
[`*mut T::wrapping_sub`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub-1
[`*mut T::write_bytes`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write_bytes
[`*mut T::write_unaligned`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write_unaligned
[`*mut T::write_volatile`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write_volatile
[`*mut T::write`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.write
[`Box::leak`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
[`FromUtf8Error::as_bytes`]: https://doc.rust-lang.org/std/string/struct.FromUtf8Error.html#method.as_bytes
[`LocalKey::try_with`]: https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.try_with
[`Option::cloned`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned
[`btree_map::Entry::and_modify`]: https://doc.rust-lang.org/std/collections/btree_map/enum.Entry.html#method.and_modify
[`fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
[`fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
[`fs::write`]: https://doc.rust-lang.org/std/fs/fn.write.html
[`hash_map::Entry::and_modify`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify
[`iter::FusedIterator`]: https://doc.rust-lang.org/std/iter/trait.FusedIterator.html
[`ops::RangeInclusive`]: https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
[`ops::RangeToInclusive`]: https://doc.rust-lang.org/std/ops/struct.RangeToInclusive.html
[`process::id`]: https://doc.rust-lang.org/std/process/fn.id.html
[`slice::rotate_left`]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left
[`slice::rotate_right`]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right
[`String::retain`]: https://doc.rust-lang.org/std/string/struct.String.html#method.retain
[cargo/5041]: https://github.com/rust-lang/cargo/pull/5041
[cargo/5083]: https://github.com/rust-lang/cargo/pull/5083
[cargo/5093]: https://github.com/rust-lang/cargo/pull/5093

