# Rust releases

<!-- TOC -->

- [Version 1.24.1 (2018-03-01)](#version-1241-2018-03-01)
- [Version 1.24.0 (2018-02-15)](#version-1240-2018-02-15)
- [Version 1.23.0 (2018-01-04)](#version-1230-2018-01-04)
- [Version 1.22.1 (2017-11-22)](#version-1221-2017-11-22)
- [Version 1.22.0 (2017-11-22)](#version-1220-2017-11-22)
- [Version 1.21.0 (2017-10-12)](#version-1210-2017-10-12)
- [Version 1.20.0 (2017-08-31)](#version-1200-2017-08-31)
- [Version 1.19.0 (2017-07-20)](#version-1190-2017-07-20)
- [Version 1.18.0 (2017-06-08)](#version-1180-2017-06-08)
- [Version 1.17.0 (2017-04-27)](#version-1170-2017-04-27)
- [Version 1.16.0 (2017-03-16)](#version-1160-2017-03-16)
- [Version 1.15.1 (2017-02-09)](#version-1151-2017-02-09)
- [Version 1.15.0 (2017-02-02)](#version-1150-2017-02-02)
- [Version 1.14.0 (2016-12-22)](#version-1140-2016-12-22)

<!-- /TOC -->




## Version 1.24.1 (2018-03-01)

 - [Do not abort when unwinding through FFI][48251]
 - [Emit UTF-16 files for linker arguments on Windows][48318]
 - [Make the error index generator work again][48308]
 - [Cargo will warn on Windows 7 if an update is needed][cargo/5069].

[48251]: https://github.com/rust-lang/rust/issues/48251
[48308]: https://github.com/rust-lang/rust/issues/48308
[48318]: https://github.com/rust-lang/rust/issues/48318
[cargo/5069]: https://github.com/rust-lang/cargo/pull/5069


## Version 1.24.0 (2018-02-15)

Language
--------
- [External `sysv64` ffi is now available.][46528]
  eg. `extern "sysv64" fn foo () {}`

Compiler
--------
- [rustc now uses 16 codegen units by default for release builds.][46910]
  For the fastest builds, utilize `codegen-units=1`.
- [Added `armv4t-unknown-linux-gnueabi` target.][47018]
- [Add `aarch64-unknown-openbsd` support][46760]

Libraries
---------
- [`str::find::<char>` now uses memchr.][46735] This should lead to a 10x
  improvement in performance in the majority of cases.
- [`OsStr`'s `Debug` implementation is now lossless and consistent
  with Windows.][46798]
- [`time::{SystemTime, Instant}` now implement `Hash`.][46828]
- [impl `From<bool>` for `AtomicBool`][46293]
- [impl `From<{CString, &CStr}>` for `{Arc<CStr>, Rc<CStr>}`][45990]
- [impl `From<{OsString, &OsStr}>` for `{Arc<OsStr>, Rc<OsStr>}`][45990]
- [impl `From<{PathBuf, &Path}>` for `{Arc<Path>, Rc<Path>}`][45990]
- [float::from_bits now just uses transmute.][46012] This provides
  some optimisations from LLVM.
- [Copied `AsciiExt` methods onto `char`][46077]
- [Remove `T: Sized` requirement on `ptr::is_null()`][46094]
- [impl `From<RecvError>` for `{TryRecvError, RecvTimeoutError}`][45506]
- [Optimised `f32::{min, max}` to generate more efficient x86 assembly][47080]
- [`[u8]::contains` now uses memchr which provides a 3x speed improvement][46713]

Stabilized APIs
---------------
- [`RefCell::replace`]
- [`RefCell::swap`]
- [`atomic::spin_loop_hint`]

The following functions can now be used in a constant expression.
eg. `let buffer: [u8; size_of::<usize>()];`, `static COUNTER: AtomicUsize = AtomicUsize::new(1);`

- [`AtomicBool::new`][46287]
- [`AtomicUsize::new`][46287]
- [`AtomicIsize::new`][46287]
- [`AtomicPtr::new`][46287]
- [`Cell::new`][46287]
- [`{integer}::min_value`][46287]
- [`{integer}::max_value`][46287]
- [`mem::size_of`][46287]
- [`mem::align_of`][46287]
- [`ptr::null`][46287]
- [`ptr::null_mut`][46287]
- [`RefCell::new`][46287]
- [`UnsafeCell::new`][46287]

Cargo
-----
- [Added a `workspace.default-members` config that
  overrides implied `--all` in virtual workspaces.][cargo/4743]
- [Enable incremental by default on development builds.][cargo/4817] Also added
  configuration keys to `Cargo.toml` and `.cargo/config` to disable on a
  per-project or global basis respectively.

Misc
----

Compatibility Notes
-------------------
- [Floating point types `Debug` impl now always prints a decimal point.][46831]
- [`Ipv6Addr` now rejects superfluous `::`'s in IPv6 addresses][46671] This is
  in accordance with IETF RFC 4291 §2.2.
- [Unwinding will no longer go past FFI boundaries, and will instead abort.][46833]
- [`Formatter::flags` method is now deprecated.][46284] The `sign_plus`,
  `sign_minus`, `alternate`, and `sign_aware_zero_pad` should be used instead.
- [Leading zeros in tuple struct members is now an error][47084]
- [`column!()` macro is one-based instead of zero-based][46977]
- [`fmt::Arguments` can no longer be shared across threads][45198]
- [Access to `#[repr(packed)]` struct fields is now unsafe][44884]
- [Cargo sets a different working directory for the compiler][cargo/4788]

[44884]: https://github.com/rust-lang/rust/pull/44884
[45198]: https://github.com/rust-lang/rust/pull/45198
[45506]: https://github.com/rust-lang/rust/pull/45506
[45904]: https://github.com/rust-lang/rust/pull/45904
[45990]: https://github.com/rust-lang/rust/pull/45990
[46012]: https://github.com/rust-lang/rust/pull/46012
[46077]: https://github.com/rust-lang/rust/pull/46077
[46094]: https://github.com/rust-lang/rust/pull/46094
[46284]: https://github.com/rust-lang/rust/pull/46284
[46287]: https://github.com/rust-lang/rust/pull/46287
[46293]: https://github.com/rust-lang/rust/pull/46293
[46528]: https://github.com/rust-lang/rust/pull/46528
[46671]: https://github.com/rust-lang/rust/pull/46671
[46713]: https://github.com/rust-lang/rust/pull/46713
[46735]: https://github.com/rust-lang/rust/pull/46735
[46749]: https://github.com/rust-lang/rust/pull/46749
[46760]: https://github.com/rust-lang/rust/pull/46760
[46798]: https://github.com/rust-lang/rust/pull/46798
[46828]: https://github.com/rust-lang/rust/pull/46828
[46831]: https://github.com/rust-lang/rust/pull/46831
[46833]: https://github.com/rust-lang/rust/pull/46833
[46910]: https://github.com/rust-lang/rust/pull/46910
[46977]: https://github.com/rust-lang/rust/pull/46977
[47018]: https://github.com/rust-lang/rust/pull/47018
[47080]: https://github.com/rust-lang/rust/pull/47080
[47084]: https://github.com/rust-lang/rust/pull/47084
[cargo/4743]: https://github.com/rust-lang/cargo/pull/4743
[cargo/4788]: https://github.com/rust-lang/cargo/pull/4788
[cargo/4817]: https://github.com/rust-lang/cargo/pull/4817
[`RefCell::replace`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.replace
[`RefCell::swap`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.swap
[`atomic::spin_loop_hint`]: https://doc.rust-lang.org/std/sync/atomic/fn.spin_loop_hint.html


## Version 1.23.0 (2018-01-04)

Language
--------
- [Arbitrary `auto` traits are now permitted in trait objects.][45772]
- [rustc now uses subtyping on the left hand side of binary operations.][45435]
  Which should fix some confusing errors in some operations.

Compiler
--------
- [Enabled `TrapUnreachable` in LLVM which should mitigate the impact of
  undefined behaviour.][45920]
- [rustc now suggests renaming import if names clash.][45660]
- [Display errors/warnings correctly when there are zero-width or
  wide characters.][45711]
- [rustc now avoids unnecessary copies of arguments that are
  simple bindings][45380] This should improve memory usage on average by 5-10%.
- [Updated musl used to build musl rustc to 1.1.17][45393]

Libraries
---------
- [Allow a trailing comma in `assert_eq/ne` macro][45887]
- [Implement Hash for raw pointers to unsized types][45483]
- [impl `From<*mut T>` for `AtomicPtr<T>`][45610]
- [impl `From<usize/isize>` for `AtomicUsize/AtomicIsize`.][45610]
- [Removed the `T: Sync` requirement for `RwLock<T>: Send`][45267]
- [Removed `T: Sized` requirement for `{<*const T>, <*mut T>}::as_ref`
  and `<*mut T>::as_mut`][44932]
- [Optimized `Thread::{park, unpark}` implementation][45524]
- [Improved `SliceExt::binary_search` performance.][45333]
- [impl `FromIterator<()>` for `()`][45379]
- [Copied `AsciiExt` trait methods to primitive types.][44042] Use of `AsciiExt`
  is now deprecated.

Stabilized APIs
---------------

Cargo
-----
- [Cargo now supports uninstallation of multiple packages][cargo/4561]
  eg. `cargo uninstall foo bar` uninstalls `foo` and `bar`.
- [Added unit test checking to `cargo check`][cargo/4592]
- [Cargo now lets you install a specific version
  using `cargo install --version`][cargo/4637]

Misc
----
- [Releases now ship with the Cargo book documentation.][45692]
- [rustdoc now prints rendering warnings on every run.][45324]

Compatibility Notes
-------------------
- [Changes have been made to type equality to make it more correct,
  in rare cases this could break some code.][45853] [Tracking issue for
  further information][45852]
- [`char::escape_debug` now uses Unicode 10 over 9.][45571]
- [Upgraded Android SDK to 27, and NDK to r15c.][45580] This drops support for
  Android 9, the minimum supported version is Android 14.
- [Bumped the minimum LLVM to 3.9][45326]

[44042]: https://github.com/rust-lang/rust/pull/44042
[44932]: https://github.com/rust-lang/rust/pull/44932
[45267]: https://github.com/rust-lang/rust/pull/45267
[45324]: https://github.com/rust-lang/rust/pull/45324
[45326]: https://github.com/rust-lang/rust/pull/45326
[45333]: https://github.com/rust-lang/rust/pull/45333
[45379]: https://github.com/rust-lang/rust/pull/45379
[45380]: https://github.com/rust-lang/rust/pull/45380
[45393]: https://github.com/rust-lang/rust/pull/45393
[45435]: https://github.com/rust-lang/rust/pull/45435
[45483]: https://github.com/rust-lang/rust/pull/45483
[45524]: https://github.com/rust-lang/rust/pull/45524
[45571]: https://github.com/rust-lang/rust/pull/45571
[45580]: https://github.com/rust-lang/rust/pull/45580
[45610]: https://github.com/rust-lang/rust/pull/45610
[45660]: https://github.com/rust-lang/rust/pull/45660
[45692]: https://github.com/rust-lang/rust/pull/45692
[45711]: https://github.com/rust-lang/rust/pull/45711
[45772]: https://github.com/rust-lang/rust/pull/45772
[45852]: https://github.com/rust-lang/rust/issues/45852
[45853]: https://github.com/rust-lang/rust/pull/45853
[45887]: https://github.com/rust-lang/rust/pull/45887
[45920]: https://github.com/rust-lang/rust/pull/45920
[cargo/4561]: https://github.com/rust-lang/cargo/pull/4561
[cargo/4592]: https://github.com/rust-lang/cargo/pull/4592
[cargo/4637]: https://github.com/rust-lang/cargo/pull/4637


## Version 1.22.1 (2017-11-22)


- [Update Cargo to fix an issue with macOS 10.13 "High Sierra"][46183]

[46183]: https://github.com/rust-lang/rust/pull/46183

## Version 1.22.0 (2017-11-22)


Language
--------
- [`non_snake_case` lint now allows extern no-mangle functions][44966]
- [Now accepts underscores in unicode escapes][43716]
- [`T op= &T` now works for numeric types.][44287] eg. `let mut x = 2; x += &8;`
- [types that impl `Drop` are now allowed in `const` and `static` types][44456]

Compiler
--------
- [rustc now defaults to having 16 codegen units at debug on supported platforms.][45064]
- [rustc will no longer inline in codegen units when compiling for debug][45075]
  This should decrease compile times for debug builds.
- [strict memory alignment now enabled on ARMv6][45094]
- [Remove support for the PNaCl target `le32-unknown-nacl`][45041]

Libraries
---------
- [Allow atomic operations up to 32 bits
  on `armv5te_unknown_linux_gnueabi`][44978]
- [`Box<Error>` now impls `From<Cow<str>>`][44466]
- [`std::mem::Discriminant` is now guaranteed to be `Send + Sync`][45095]
- [`fs::copy` now returns the length of the main stream on NTFS.][44895]
- [Properly detect overflow in `Instant += Duration`.][44220]
- [impl `Hasher` for `{&mut Hasher, Box<Hasher>}`][44015]
- [impl `fmt::Debug` for `SplitWhitespace`.][44303]
- [`Option<T>` now impls `Try`][42526] This allows for using `?` with `Option` types.

Stabilized APIs
---------------

Cargo
-----
- [Cargo will now build multi file examples in subdirectories of the `examples`
  folder that have a `main.rs` file.][cargo/4496]
- [Changed `[root]` to `[package]` in `Cargo.lock`][cargo/4571] Packages with
  the old format will continue to work and can be updated with `cargo update`.
- [Now supports vendoring git repositories][cargo/3992]

Misc
----
- [`libbacktrace` is now available on Apple platforms.][44251]
- [Stabilised the `compile_fail` attribute for code fences in doc-comments.][43949]
  This now lets you specify that a given code example will fail to compile.

Compatibility Notes
-------------------
- [The minimum Android version that rustc can build for has been bumped
  to `4.0` from `2.3`][45656]
- [Allowing `T op= &T` for numeric types has broken some type
  inference cases][45480]


[42526]: https://github.com/rust-lang/rust/pull/42526
[43017]: https://github.com/rust-lang/rust/pull/43017
[43716]: https://github.com/rust-lang/rust/pull/43716
[43949]: https://github.com/rust-lang/rust/pull/43949
[44015]: https://github.com/rust-lang/rust/pull/44015
[44220]: https://github.com/rust-lang/rust/pull/44220
[44251]: https://github.com/rust-lang/rust/pull/44251
[44287]: https://github.com/rust-lang/rust/pull/44287
[44303]: https://github.com/rust-lang/rust/pull/44303
[44456]: https://github.com/rust-lang/rust/pull/44456
[44466]: https://github.com/rust-lang/rust/pull/44466
[44895]: https://github.com/rust-lang/rust/pull/44895
[44966]: https://github.com/rust-lang/rust/pull/44966
[44978]: https://github.com/rust-lang/rust/pull/44978
[45041]: https://github.com/rust-lang/rust/pull/45041
[45064]: https://github.com/rust-lang/rust/pull/45064
[45075]: https://github.com/rust-lang/rust/pull/45075
[45094]: https://github.com/rust-lang/rust/pull/45094
[45095]: https://github.com/rust-lang/rust/pull/45095
[45480]: https://github.com/rust-lang/rust/issues/45480
[45656]: https://github.com/rust-lang/rust/pull/45656
[cargo/3992]: https://github.com/rust-lang/cargo/pull/3992
[cargo/4496]: https://github.com/rust-lang/cargo/pull/4496
[cargo/4571]: https://github.com/rust-lang/cargo/pull/4571






## Version 1.21.0 (2017-10-12)

Language
--------
- [You can now use static references for literals.][43838]
  Example:
  ```rust
  fn main() {
      let x: &'static u32 = &0;
  }
  ```
- [Relaxed path syntax. Optional `::` before `<` is now allowed in all contexts.][43540]
  Example:
  ```rust
  my_macro!(Vec<i32>::new); // Always worked
  my_macro!(Vec::<i32>::new); // Now works
  ```

Compiler
--------
- [Upgraded jemalloc to 4.5.0][43911]
- [Enabled unwinding panics on Redox][43917]
- [Now runs LLVM in parallel during translation phase.][43506]
  This should reduce peak memory usage.

Libraries
---------
- [Generate builtin impls for `Clone` for all arrays and tuples that
  are `T: Clone`][43690]
- [`Stdin`, `Stdout`, and `Stderr` now implement `AsRawFd`.][43459]
- [`Rc` and `Arc` now implement `From<&[T]> where T: Clone`, `From<str>`,
  `From<String>`, `From<Box<T>> where T: ?Sized`, and `From<Vec<T>>`.][42565]

Stabilized APIs
---------------

[`std::mem::discriminant`]

Cargo
-----
- [You can now call `cargo install` with multiple package names][cargo/4216]
- [Cargo commands inside a virtual workspace will now implicitly
  pass `--all`][cargo/4335]
- [Added a `[patch]` section to `Cargo.toml` to handle
  prepublication dependencies][cargo/4123] [RFC 1969]
- [`include` & `exclude` fields in `Cargo.toml` now accept gitignore
  like patterns][cargo/4270]
- [Added the `--all-targets` option][cargo/4400]
- [Using required dependencies as a feature is now deprecated and emits
  a warning][cargo/4364]


Misc
----
- [Cargo docs are moving][43916]
  to [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo)
- [The rustdoc book is now available][43863]
  at [doc.rust-lang.org/rustdoc](https://doc.rust-lang.org/rustdoc)
- [Added a preview of RLS has been made available through rustup][44204]
  Install with `rustup component add rls-preview`
- [`std::os` documentation for Unix, Linux, and Windows now appears on doc.rust-lang.org][43348]
  Previously only showed `std::os::unix`.

Compatibility Notes
-------------------
- [Changes in method matching against higher-ranked types][43880] This may cause
  breakage in subtyping corner cases. [A more in-depth explanation is available.][info/43880]
- [rustc's JSON error output's byte position start at top of file.][42973]
  Was previously relative to the rustc's internal `CodeMap` struct which
  required the unstable library `libsyntax` to correctly use.
- [`unused_results` lint no longer ignores booleans][43728]

[42565]: https://github.com/rust-lang/rust/pull/42565
[42973]: https://github.com/rust-lang/rust/pull/42973
[43348]: https://github.com/rust-lang/rust/pull/43348
[43459]: https://github.com/rust-lang/rust/pull/43459
[43506]: https://github.com/rust-lang/rust/pull/43506
[43540]: https://github.com/rust-lang/rust/pull/43540
[43690]: https://github.com/rust-lang/rust/pull/43690
[43728]: https://github.com/rust-lang/rust/pull/43728
[43838]: https://github.com/rust-lang/rust/pull/43838
[43863]: https://github.com/rust-lang/rust/pull/43863
[43880]: https://github.com/rust-lang/rust/pull/43880
[43911]: https://github.com/rust-lang/rust/pull/43911
[43916]: https://github.com/rust-lang/rust/pull/43916
[43917]: https://github.com/rust-lang/rust/pull/43917
[44204]: https://github.com/rust-lang/rust/pull/44204
[cargo/4123]: https://github.com/rust-lang/cargo/pull/4123
[cargo/4216]: https://github.com/rust-lang/cargo/pull/4216
[cargo/4270]: https://github.com/rust-lang/cargo/pull/4270
[cargo/4335]: https://github.com/rust-lang/cargo/pull/4335
[cargo/4364]: https://github.com/rust-lang/cargo/pull/4364
[cargo/4400]: https://github.com/rust-lang/cargo/pull/4400
[RFC 1969]: https://github.com/rust-lang/rfcs/pull/1969
[info/43880]: https://github.com/rust-lang/rust/issues/44224#issuecomment-330058902
[`std::mem::discriminant`]: https://doc.rust-lang.org/std/mem/fn.discriminant.html


## Version 1.20.0 (2017-08-31)

Language
--------
- [Associated constants are now stabilised.][42809]
- [A lot of macro bugs are now fixed.][42913]

Compiler
--------

- [Struct fields are now properly coerced to the expected field type.][42807]
- [Enabled wasm LLVM backend][42571] WASM can now be built with the
  `wasm32-experimental-emscripten` target.
- [Changed some of the error messages to be more helpful.][42033]
- [Add support for RELRO(RELocation Read-Only) for platforms that support
  it.][43170]
- [rustc now reports the total number of errors on compilation failure][43015]
  previously this was only the number of errors in the pass that failed.
- [Expansion in rustc has been sped up 29x.][42533]
- [added `msp430-none-elf` target.][43099]
- [rustc will now suggest one-argument enum variant to fix type mismatch when
  applicable][43178]
- [Fixes backtraces on Redox][43228]
- [rustc now identifies different versions of same crate when absolute paths of
  different types match in an error message.][42826]

Libraries
---------


- [Relaxed Debug constraints on `{HashMap,BTreeMap}::{Keys,Values}`.][42854]
- [Impl `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Debug`, `Hash` for unsized
  tuples.][43011]
- [Impl `fmt::{Display, Debug}` for `Ref`, `RefMut`, `MutexGuard`,
  `RwLockReadGuard`, `RwLockWriteGuard`][42822]
- [Impl `Clone` for `DefaultHasher`.][42799]
- [Impl `Sync` for `SyncSender`.][42397]
- [Impl `FromStr` for `char`][42271]
- [Fixed how `{f32, f64}::{is_sign_negative, is_sign_positive}` handles
  NaN.][42431]
- [allow messages in the `unimplemented!()` macro.][42155]
  ie. `unimplemented!("Waiting for 1.21 to be stable")`
- [`pub(restricted)` is now supported in the `thread_local!` macro.][43185]
- [Upgrade to Unicode 10.0.0][42999]
- [Reimplemented `{f32, f64}::{min, max}` in Rust instead of using CMath.][42430]
- [Skip the main thread's manual stack guard on Linux][43072]
- [Iterator::nth for `ops::{Range, RangeFrom}` is now done in O(1) time][43077]
- [`#[repr(align(N))]` attribute max number is now 2^31 - 1.][43097] This was
  previously 2^15.
- [`{OsStr, Path}::Display` now avoids allocations where possible][42613]

Stabilized APIs
---------------

- [`CStr::into_c_string`]
- [`CString::as_c_str`]
- [`CString::into_boxed_c_str`]
- [`Chain::get_mut`]
- [`Chain::get_ref`]
- [`Chain::into_inner`]
- [`Option::get_or_insert_with`]
- [`Option::get_or_insert`]
- [`OsStr::into_os_string`]
- [`OsString::into_boxed_os_str`]
- [`Take::get_mut`]
- [`Take::get_ref`]
- [`Utf8Error::error_len`]
- [`char::EscapeDebug`]
- [`char::escape_debug`]
- [`compile_error!`]
- [`f32::from_bits`]
- [`f32::to_bits`]
- [`f64::from_bits`]
- [`f64::to_bits`]
- [`mem::ManuallyDrop`]
- [`slice::sort_unstable_by_key`]
- [`slice::sort_unstable_by`]
- [`slice::sort_unstable`]
- [`str::from_boxed_utf8_unchecked`]
- [`str::as_bytes_mut`]
- [`str::as_bytes_mut`]
- [`str::from_utf8_mut`]
- [`str::from_utf8_unchecked_mut`]
- [`str::get_mut`]
- [`str::get_unchecked_mut`]
- [`str::get_unchecked`]
- [`str::get`]
- [`str::into_boxed_bytes`]


Cargo
-----
- [Cargo API token location moved from `~/.cargo/config` to
  `~/.cargo/credentials`.][cargo/3978]
- [Cargo will now build `main.rs` binaries that are in sub-directories of
  `src/bin`.][cargo/4214] ie. Having `src/bin/server/main.rs` and
  `src/bin/client/main.rs` generates `target/debug/server` and `target/debug/client`
- [You can now specify version of a binary when installed through
  `cargo install` using `--vers`.][cargo/4229]
- [Added `--no-fail-fast` flag to cargo to run all benchmarks regardless of
  failure.][cargo/4248]
- [Changed the convention around which file is the crate root.][cargo/4259]
- [The `include`/`exclude` property in `Cargo.toml` now accepts gitignore paths
  instead of glob patterns][cargo/4270]. Glob patterns are now deprecated.

Compatibility Notes
-------------------

- [Functions with `'static` in their return types will now not be as usable as
  if they were using lifetime parameters instead.][42417]
- [The reimplementation of `{f32, f64}::is_sign_{negative, positive}` now
  takes the sign of NaN into account where previously didn't.][42430]

[42033]: https://github.com/rust-lang/rust/pull/42033
[42155]: https://github.com/rust-lang/rust/pull/42155
[42271]: https://github.com/rust-lang/rust/pull/42271
[42397]: https://github.com/rust-lang/rust/pull/42397
[42417]: https://github.com/rust-lang/rust/pull/42417
[42430]: https://github.com/rust-lang/rust/pull/42430
[42431]: https://github.com/rust-lang/rust/pull/42431
[42533]: https://github.com/rust-lang/rust/pull/42533
[42571]: https://github.com/rust-lang/rust/pull/42571
[42613]: https://github.com/rust-lang/rust/pull/42613
[42799]: https://github.com/rust-lang/rust/pull/42799
[42807]: https://github.com/rust-lang/rust/pull/42807
[42809]: https://github.com/rust-lang/rust/pull/42809
[42822]: https://github.com/rust-lang/rust/pull/42822
[42826]: https://github.com/rust-lang/rust/pull/42826
[42854]: https://github.com/rust-lang/rust/pull/42854
[42913]: https://github.com/rust-lang/rust/pull/42913
[42999]: https://github.com/rust-lang/rust/pull/42999
[43011]: https://github.com/rust-lang/rust/pull/43011
[43015]: https://github.com/rust-lang/rust/pull/43015
[43072]: https://github.com/rust-lang/rust/pull/43072
[43077]: https://github.com/rust-lang/rust/pull/43077
[43097]: https://github.com/rust-lang/rust/pull/43097
[43099]: https://github.com/rust-lang/rust/pull/43099
[43170]: https://github.com/rust-lang/rust/pull/43170
[43178]: https://github.com/rust-lang/rust/pull/43178
[43185]: https://github.com/rust-lang/rust/pull/43185
[43228]: https://github.com/rust-lang/rust/pull/43228
[cargo/3978]: https://github.com/rust-lang/cargo/pull/3978
[cargo/4214]: https://github.com/rust-lang/cargo/pull/4214
[cargo/4229]: https://github.com/rust-lang/cargo/pull/4229
[cargo/4248]: https://github.com/rust-lang/cargo/pull/4248
[cargo/4259]: https://github.com/rust-lang/cargo/pull/4259
[cargo/4270]: https://github.com/rust-lang/cargo/pull/4270
[`CStr::into_c_string`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.into_c_string
[`CString::as_c_str`]: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_c_str
[`CString::into_boxed_c_str`]: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_boxed_c_str
[`Chain::get_mut`]: https://doc.rust-lang.org/std/io/struct.Chain.html#method.get_mut
[`Chain::get_ref`]: https://doc.rust-lang.org/std/io/struct.Chain.html#method.get_ref
[`Chain::into_inner`]: https://doc.rust-lang.org/std/io/struct.Chain.html#method.into_inner
[`Option::get_or_insert_with`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with
[`Option::get_or_insert`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert
[`OsStr::into_os_string`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.into_os_string
[`OsString::into_boxed_os_str`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html#method.into_boxed_os_str
[`Take::get_mut`]: https://doc.rust-lang.org/std/io/struct.Take.html#method.get_mut
[`Take::get_ref`]: https://doc.rust-lang.org/std/io/struct.Take.html#method.get_ref
[`Utf8Error::error_len`]: https://doc.rust-lang.org/std/str/struct.Utf8Error.html#method.error_len
[`char::EscapeDebug`]: https://doc.rust-lang.org/std/char/struct.EscapeDebug.html
[`char::escape_debug`]: https://doc.rust-lang.org/std/primitive.char.html#method.escape_debug
[`compile_error!`]: https://doc.rust-lang.org/std/macro.compile_error.html
[`f32::from_bits`]: https://doc.rust-lang.org/std/primitive.f32.html#method.from_bits
[`f32::to_bits`]: https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits
[`f64::from_bits`]: https://doc.rust-lang.org/std/primitive.f64.html#method.from_bits
[`f64::to_bits`]: https://doc.rust-lang.org/std/primitive.f64.html#method.to_bits
[`mem::ManuallyDrop`]: https://doc.rust-lang.org/std/mem/union.ManuallyDrop.html
[`slice::sort_unstable_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by_key
[`slice::sort_unstable_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
[`slice::sort_unstable`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable
[`str::from_boxed_utf8_unchecked`]: https://doc.rust-lang.org/std/str/fn.from_boxed_utf8_unchecked.html
[`str::as_bytes_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes_mut
[`str::from_utf8_mut`]: https://doc.rust-lang.org/std/str/fn.from_utf8_mut.html
[`str::from_utf8_unchecked_mut`]: https://doc.rust-lang.org/std/str/fn.from_utf8_unchecked_mut.html
[`str::get_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.get_mut
[`str::get_unchecked_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.get_unchecked_mut
[`str::get_unchecked`]: https://doc.rust-lang.org/std/primitive.str.html#method.get_unchecked
[`str::get`]: https://doc.rust-lang.org/std/primitive.str.html#method.get
[`str::into_boxed_bytes`]: https://doc.rust-lang.org/std/primitive.str.html#method.into_boxed_bytes


## Version 1.19.0 (2017-07-20)

Language
--------

- [Numeric fields can now be used for creating tuple structs.][41145] [RFC 1506]
  For example `struct Point(u32, u32); let x = Point { 0: 7, 1: 0 };`.
- [Macro recursion limit increased to 1024 from 64.][41676]
- [Added lint for detecting unused macros.][41907]
- [`loop` can now return a value with `break`.][42016] [RFC 1624]
  For example: `let x = loop { break 7; };`
- [C compatible `union`s are now available.][42068] [RFC 1444] They can only
  contain `Copy` types and cannot have a `Drop` implementation.
  Example: `union Foo { bar: u8, baz: usize }`
- [Non capturing closures can now be coerced into `fn`s,][42162] [RFC 1558]
  Example: `let foo: fn(u8) -> u8 = |v: u8| { v };`

Compiler
--------

- [Add support for bootstrapping the Rust compiler toolchain on Android.][41370]
- [Change `arm-linux-androideabi` to correspond to the `armeabi`
  official ABI.][41656] If you wish to continue targeting the `armeabi-v7a` ABI
  you should use `--target armv7-linux-androideabi`.
- [Fixed ICE when removing a source file between compilation sessions.][41873]
- [Minor optimisation of string operations.][42037]
- [Compiler error message is now `aborting due to previous error(s)` instead of
  `aborting due to N previous errors`][42150] This was previously inaccurate and
  would only count certain kinds of errors.
- [The compiler now supports Visual Studio 2017][42225]
- [The compiler is now built against LLVM 4.0.1 by default][42948]
- [Added a lot][42264] of [new error codes][42302]
- [Added `target-feature=+crt-static` option][37406] [RFC 1721] Which allows
  libraries with C Run-time Libraries(CRT) to be statically linked.
- [Fixed various ARM codegen bugs][42740]

Libraries
---------

- [`String` now implements `FromIterator<Cow<'a, str>>` and
  `Extend<Cow<'a, str>>`][41449]
- [`Vec` now implements `From<&mut [T]>`][41530]
- [`Box<[u8]>` now implements `From<Box<str>>`][41258]
- [`SplitWhitespace` now implements `Clone`][41659]
- [`[u8]::reverse` is now 5x faster and `[u16]::reverse` is now
  1.5x faster][41764]
- [`eprint!` and `eprintln!` macros added to prelude.][41192] Same as the `print!`
  macros, but for printing to stderr.

Stabilized APIs
---------------

- [`OsString::shrink_to_fit`]
- [`cmp::Reverse`]
- [`Command::envs`]
- [`thread::ThreadId`]

Cargo
-----

- [Build scripts can now add environment variables to the environment
  the crate is being compiled in.
  Example: `println!("cargo:rustc-env=FOO=bar");`][cargo/3929]
- [Subcommands now replace the current process rather than spawning a new
  child process][cargo/3970]
- [Workspace members can now accept glob file patterns][cargo/3979]
- [Added `--all` flag to the `cargo bench` subcommand to run benchmarks of all
  the members in a given workspace.][cargo/3988]
- [Updated `libssh2-sys` to 0.2.6][cargo/4008]
- [Target directory path is now in the cargo metadata][cargo/4022]
- [Cargo no longer checks out a local working directory for the
  crates.io index][cargo/4026] This should provide smaller file size for the
  registry, and improve cloning times, especially on Windows machines.
- [Added an `--exclude` option for excluding certain packages when using the
  `--all` option][cargo/4031]
- [Cargo will now automatically retry when receiving a 5xx error
  from crates.io][cargo/4032]
- [The `--features` option now accepts multiple comma or space
  delimited values.][cargo/4084]
- [Added support for custom target specific runners][cargo/3954]

Misc
----

- [Added `rust-windbg.cmd`][39983] for loading rust `.natvis` files in the
  Windows Debugger.
- [Rust will now release XZ compressed packages][rust-installer/57]
- [rustup will now prefer to download rust packages with
  XZ compression][rustup/1100] over GZip packages.
- [Added the ability to escape `#` in rust documentation][41785] By adding
  additional `#`'s ie. `##` is now `#`

Compatibility Notes
-------------------

- [`MutexGuard<T>` may only be `Sync` if `T` is `Sync`.][41624]
- [`-Z` flags are now no longer allowed to be used on the stable
  compiler.][41751] This has been a warning for a year previous to this.
- [As a result of the `-Z` flag change, the `cargo-check` plugin no
  longer works][42844]. Users should migrate to the built-in `check`
  command, which has been available since 1.16.
- [Ending a float literal with `._` is now a hard error.
  Example: `42._` .][41946]
- [Any use of a private `extern crate` outside of its module is now a
  hard error.][36886] This was previously a warning.
- [`use ::self::foo;` is now a hard error.][36888] `self` paths are always
  relative while the `::` prefix makes a path absolute, but was ignored and the
  path was relative regardless.
- [Floating point constants in match patterns is now a hard error][36890]
  This was previously a warning.
- [Struct or enum constants that don't derive `PartialEq` & `Eq` used
  match patterns is now a hard error][36891] This was previously a warning.
- [Lifetimes named `'_` are no longer allowed.][36892] This was previously
  a warning.
- [From the pound escape, lines consisting of multiple `#`s are
  now visible][41785]
- [It is an error to re-export private enum variants][42460]. This is
  known to break a number of crates that depend on an older version of
  mustache.
- [On Windows, if `VCINSTALLDIR` is set incorrectly, `rustc` will try
  to use it to find the linker, and the build will fail where it did
  not previously][42607]

[36886]: https://github.com/rust-lang/rust/issues/36886
[36888]: https://github.com/rust-lang/rust/issues/36888
[36890]: https://github.com/rust-lang/rust/issues/36890
[36891]: https://github.com/rust-lang/rust/issues/36891
[36892]: https://github.com/rust-lang/rust/issues/36892
[37406]: https://github.com/rust-lang/rust/issues/37406
[39983]: https://github.com/rust-lang/rust/pull/39983
[41145]: https://github.com/rust-lang/rust/pull/41145
[41192]: https://github.com/rust-lang/rust/pull/41192
[41258]: https://github.com/rust-lang/rust/pull/41258
[41370]: https://github.com/rust-lang/rust/pull/41370
[41449]: https://github.com/rust-lang/rust/pull/41449
[41530]: https://github.com/rust-lang/rust/pull/41530
[41624]: https://github.com/rust-lang/rust/pull/41624
[41656]: https://github.com/rust-lang/rust/pull/41656
[41659]: https://github.com/rust-lang/rust/pull/41659
[41676]: https://github.com/rust-lang/rust/pull/41676
[41751]: https://github.com/rust-lang/rust/pull/41751
[41764]: https://github.com/rust-lang/rust/pull/41764
[41785]: https://github.com/rust-lang/rust/pull/41785
[41873]: https://github.com/rust-lang/rust/pull/41873
[41907]: https://github.com/rust-lang/rust/pull/41907
[41946]: https://github.com/rust-lang/rust/pull/41946
[42016]: https://github.com/rust-lang/rust/pull/42016
[42037]: https://github.com/rust-lang/rust/pull/42037
[42068]: https://github.com/rust-lang/rust/pull/42068
[42150]: https://github.com/rust-lang/rust/pull/42150
[42162]: https://github.com/rust-lang/rust/pull/42162
[42225]: https://github.com/rust-lang/rust/pull/42225
[42264]: https://github.com/rust-lang/rust/pull/42264
[42302]: https://github.com/rust-lang/rust/pull/42302
[42460]: https://github.com/rust-lang/rust/issues/42460
[42607]: https://github.com/rust-lang/rust/issues/42607
[42740]: https://github.com/rust-lang/rust/pull/42740
[42844]: https://github.com/rust-lang/rust/issues/42844
[42948]: https://github.com/rust-lang/rust/pull/42948
[RFC 1444]: https://github.com/rust-lang/rfcs/pull/1444
[RFC 1506]: https://github.com/rust-lang/rfcs/pull/1506
[RFC 1558]: https://github.com/rust-lang/rfcs/pull/1558
[RFC 1624]: https://github.com/rust-lang/rfcs/pull/1624
[RFC 1721]: https://github.com/rust-lang/rfcs/pull/1721
[`Command::envs`]: https://doc.rust-lang.org/std/process/struct.Command.html#method.envs
[`OsString::shrink_to_fit`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html#method.shrink_to_fit
[`cmp::Reverse`]: https://doc.rust-lang.org/std/cmp/struct.Reverse.html
[`thread::ThreadId`]: https://doc.rust-lang.org/std/thread/struct.ThreadId.html
[cargo/3929]: https://github.com/rust-lang/cargo/pull/3929
[cargo/3954]: https://github.com/rust-lang/cargo/pull/3954
[cargo/3970]: https://github.com/rust-lang/cargo/pull/3970
[cargo/3979]: https://github.com/rust-lang/cargo/pull/3979
[cargo/3988]: https://github.com/rust-lang/cargo/pull/3988
[cargo/4008]: https://github.com/rust-lang/cargo/pull/4008
[cargo/4022]: https://github.com/rust-lang/cargo/pull/4022
[cargo/4026]: https://github.com/rust-lang/cargo/pull/4026
[cargo/4031]: https://github.com/rust-lang/cargo/pull/4031
[cargo/4032]: https://github.com/rust-lang/cargo/pull/4032
[cargo/4084]: https://github.com/rust-lang/cargo/pull/4084
[rust-installer/57]: https://github.com/rust-lang/rust-installer/pull/57
[rustup/1100]: https://github.com/rust-lang-nursery/rustup.rs/pull/1100


## Version 1.18.0 (2017-06-08)

Language
--------

- [Stabilize pub(restricted)][40556] `pub` can now accept a module path to
  make the item visible to just that module tree. Also accepts the keyword
  `crate` to make something public to the whole crate but not users of the
  library. Example: `pub(crate) mod utils;`. [RFC 1422].
- [Stabilize `#![windows_subsystem]` attribute][40870] conservative exposure of the
  `/SUBSYSTEM` linker flag on Windows platforms. [RFC 1665].
- [Refactor of trait object type parsing][40043] Now `ty` in macros can accept
  types like `Write + Send`, trailing `+` are now supported in trait objects,
  and better error reporting for trait objects starting with `?Sized`.
- [0e+10 is now a valid floating point literal][40589]
- [Now warns if you bind a lifetime parameter to 'static][40734]
- [Tuples, Enum variant fields, and structs with no `repr` attribute or with
  `#[repr(Rust)]` are reordered to minimize padding and produce a smaller
  representation in some cases.][40377]

Compiler
--------

- [rustc can now emit mir with `--emit mir`][39891]
- [Improved LLVM IR for trivial functions][40367]
- [Added explanation for E0090(Wrong number of lifetimes are supplied)][40723]
- [rustc compilation is now 15%-20% faster][41469] Thanks to optimisation
  opportunities found through profiling
- [Improved backtrace formatting when panicking][38165]

Libraries
---------

- [Specialized `Vec::from_iter` being passed `vec::IntoIter`][40731] if the
  iterator hasn't been advanced the original `Vec` is reassembled with no actual
  iteration or reallocation.
- [Simplified HashMap Bucket interface][40561] provides performance
  improvements for iterating and cloning.
- [Specialize Vec::from_elem to use calloc][40409]
- [Fixed Race condition in fs::create_dir_all][39799]
- [No longer caching stdio on Windows][40516]
- [Optimized insertion sort in slice][40807] insertion sort in some cases
  2.50%~ faster and in one case now 12.50% faster.
- [Optimized `AtomicBool::fetch_nand`][41143]

Stabilized APIs
---------------

- [`Child::try_wait`]
- [`HashMap::retain`]
- [`HashSet::retain`]
- [`PeekMut::pop`]
- [`TcpStream::peek`]
- [`UdpSocket::peek`]
- [`UdpSocket::peek_from`]

Cargo
-----

- [Added partial Pijul support][cargo/3842] Pijul is a version control system in Rust.
  You can now create new cargo projects with Pijul using `cargo new --vcs pijul`
- [Now always emits build script warnings for crates that fail to build][cargo/3847]
- [Added Android build support][cargo/3885]
- [Added `--bins` and `--tests` flags][cargo/3901] now you can build all programs
  of a certain type, for example `cargo build --bins` will build all
  binaries.
- [Added support for haiku][cargo/3952]

Misc
----

- [rustdoc can now use pulldown-cmark with the `--enable-commonmark` flag][40338]
- [Added rust-windbg script for better debugging on Windows][39983]
- [Rust now uses the official cross compiler for NetBSD][40612]
- [rustdoc now accepts `#` at the start of files][40828]
- [Fixed jemalloc support for musl][41168]

Compatibility Notes
-------------------

- [Changes to how the `0` flag works in format!][40241] Padding zeroes are now
  always placed after the sign if it exists and before the digits. With the `#`
  flag the zeroes are placed after the prefix and before the digits.
- [Due to the struct field optimisation][40377], using `transmute` on structs
  that have no `repr` attribute or `#[repr(Rust)]` will no longer work. This has
  always been undefined behavior, but is now more likely to break in practice.
- [The refactor of trait object type parsing][40043] fixed a bug where `+` was
  receiving the wrong priority parsing things like `&for<'a> Tr<'a> + Send` as
  `&(for<'a> Tr<'a> + Send)` instead of `(&for<'a> Tr<'a>) + Send`
- [Overlapping inherent `impl`s are now a hard error][40728]
- [`PartialOrd` and `Ord` must agree on the ordering.][41270]
- [`rustc main.rs -o out --emit=asm,llvm-ir`][41085] Now will output
  `out.asm` and `out.ll` instead of only one of the filetypes.
- [ calling a function that returns `Self` will no longer work][41805] when
  the size of `Self` cannot be statically determined.
- [rustc now builds with a "pthreads" flavour of MinGW for Windows GNU][40805]
  this has caused a few regressions namely:

  - Changed the link order of local static/dynamic libraries (respecting the
    order on given rather than having the compiler reorder).
  - Changed how MinGW is linked, native code linked to dynamic libraries
    may require manually linking to the gcc support library (for the native
    code itself)

[38165]: https://github.com/rust-lang/rust/pull/38165
[39799]: https://github.com/rust-lang/rust/pull/39799
[39891]: https://github.com/rust-lang/rust/pull/39891
[39983]: https://github.com/rust-lang/rust/pull/39983
[40043]: https://github.com/rust-lang/rust/pull/40043
[40241]: https://github.com/rust-lang/rust/pull/40241
[40338]: https://github.com/rust-lang/rust/pull/40338
[40367]: https://github.com/rust-lang/rust/pull/40367
[40377]: https://github.com/rust-lang/rust/pull/40377
[40409]: https://github.com/rust-lang/rust/pull/40409
[40516]: https://github.com/rust-lang/rust/pull/40516
[40556]: https://github.com/rust-lang/rust/pull/40556
[40561]: https://github.com/rust-lang/rust/pull/40561
[40589]: https://github.com/rust-lang/rust/pull/40589
[40612]: https://github.com/rust-lang/rust/pull/40612
[40723]: https://github.com/rust-lang/rust/pull/40723
[40728]: https://github.com/rust-lang/rust/pull/40728
[40731]: https://github.com/rust-lang/rust/pull/40731
[40734]: https://github.com/rust-lang/rust/pull/40734
[40805]: https://github.com/rust-lang/rust/pull/40805
[40807]: https://github.com/rust-lang/rust/pull/40807
[40828]: https://github.com/rust-lang/rust/pull/40828
[40870]: https://github.com/rust-lang/rust/pull/40870
[41085]: https://github.com/rust-lang/rust/pull/41085
[41143]: https://github.com/rust-lang/rust/pull/41143
[41168]: https://github.com/rust-lang/rust/pull/41168
[41270]: https://github.com/rust-lang/rust/issues/41270
[41469]: https://github.com/rust-lang/rust/pull/41469
[41805]: https://github.com/rust-lang/rust/issues/41805
[RFC 1422]: https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md
[RFC 1665]: https://github.com/rust-lang/rfcs/blob/master/text/1665-windows-subsystem.md
[`Child::try_wait`]: https://doc.rust-lang.org/std/process/struct.Child.html#method.try_wait
[`HashMap::retain`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain
[`HashSet::retain`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.retain
[`PeekMut::pop`]: https://doc.rust-lang.org/std/collections/binary_heap/struct.PeekMut.html#method.pop
[`TcpStream::peek`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.peek
[`UdpSocket::peek_from`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peek_from
[`UdpSocket::peek`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peek
[cargo/3842]: https://github.com/rust-lang/cargo/pull/3842
[cargo/3847]: https://github.com/rust-lang/cargo/pull/3847
[cargo/3885]: https://github.com/rust-lang/cargo/pull/3885
[cargo/3901]: https://github.com/rust-lang/cargo/pull/3901
[cargo/3952]: https://github.com/rust-lang/cargo/pull/3952


## Version 1.17.0 (2017-04-27)

Language
--------

* [The lifetime of statics and consts defaults to `'static`][39265]. [RFC 1623]
* [Fields of structs may be initialized without duplicating the field/variable
  names][39761]. [RFC 1682]
* [`Self` may be included in the `where` clause of `impls`][38864]. [RFC 1647]
* [When coercing to an unsized type lifetimes must be equal][40319]. That is,
  there is no subtyping between `T` and `U` when `T: Unsize<U>`. For example,
  coercing `&mut [&'a X; N]` to `&mut [&'b X]` requires `'a` be equal to
  `'b`. Soundness fix.
* [Values passed to the indexing operator, `[]`, automatically coerce][40166]
* [Static variables may contain references to other statics][40027]

Compiler
--------

* [Exit quickly on only `--emit dep-info`][40336]
* [Make `-C relocation-model` more correctly determine whether the linker
  creates a position-independent executable][40245]
* [Add `-C overflow-checks` to directly control whether integer overflow
  panics][40037]
* [The rustc type checker now checks items on demand instead of in a single
  in-order pass][40008]. This is mostly an internal refactoring in support of
  future work, including incremental type checking, but also resolves [RFC
  1647], allowing `Self` to appear in `impl` `where` clauses.
* [Optimize vtable loads][39995]
* [Turn off vectorization for Emscripten targets][39990]
* [Provide suggestions for unknown macros imported with `use`][39953]
* [Fix ICEs in path resolution][39939]
* [Strip exception handling code on Emscripten when `panic=abort`][39193]
* [Add clearer error message using `&str + &str`][39116]

Stabilized APIs
---------------

* [`Arc::into_raw`]
* [`Arc::from_raw`]
* [`Arc::ptr_eq`]
* [`Rc::into_raw`]
* [`Rc::from_raw`]
* [`Rc::ptr_eq`]
* [`Ordering::then`]
* [`Ordering::then_with`]
* [`BTreeMap::range`]
* [`BTreeMap::range_mut`]
* [`collections::Bound`]
* [`process::abort`]
* [`ptr::read_unaligned`]
* [`ptr::write_unaligned`]
* [`Result::expect_err`]
* [`Cell::swap`]
* [`Cell::replace`]
* [`Cell::into_inner`]
* [`Cell::take`]

Libraries
---------

* [`BTreeMap` and `BTreeSet` can iterate over ranges][27787]
* [`Cell` can store non-`Copy` types][39793]. [RFC 1651]
* [`String` implements `FromIterator<&char>`][40028]
* `Box` [implements][40009] a number of new conversions:
  `From<Box<str>> for String`,
  `From<Box<[T]>> for Vec<T>`,
  `From<Box<CStr>> for CString`,
  `From<Box<OsStr>> for OsString`,
  `From<Box<Path>> for PathBuf`,
  `Into<Box<str>> for String`,
  `Into<Box<[T]>> for Vec<T>`,
  `Into<Box<CStr>> for CString`,
  `Into<Box<OsStr>> for OsString`,
  `Into<Box<Path>> for PathBuf`,
  `Default for Box<str>`,
  `Default for Box<CStr>`,
  `Default for Box<OsStr>`,
  `From<&CStr> for Box<CStr>`,
  `From<&OsStr> for Box<OsStr>`,
  `From<&Path> for Box<Path>`
* [`ffi::FromBytesWithNulError` implements `Error` and `Display`][39960]
* [Specialize `PartialOrd<A> for [A] where A: Ord`][39642]
* [Slightly optimize `slice::sort`][39538]
* [Add `ToString` trait specialization for `Cow<'a, str>` and `String`][39440]
* [`Box<[T]>` implements `From<&[T]> where T: Copy`,
  `Box<str>` implements `From<&str>`][39438]
* [`IpAddr` implements `From` for various arrays. `SocketAddr` implements
  `From<(I, u16)> where I: Into<IpAddr>`][39372]
* [`format!` estimates the needed capacity before writing a string][39356]
* [Support unprivileged symlink creation in Windows][38921]
* [`PathBuf` implements `Default`][38764]
* [Implement `PartialEq<[A]>` for `VecDeque<A>`][38661]
* [`HashMap` resizes adaptively][38368] to guard against DOS attacks
  and poor hash functions.

Cargo
-----

* [Add `cargo check --all`][cargo/3731]
* [Add an option to ignore SSL revocation checking][cargo/3699]
* [Add `cargo run --package`][cargo/3691]
* [Add `required_features`][cargo/3667]
* [Assume `build.rs` is a build script][cargo/3664]
* [Find workspace via `workspace_root` link in containing member][cargo/3562]

Misc
----

* [Documentation is rendered with mdbook instead of the obsolete, in-tree
  `rustbook`][39633]
* [The "Unstable Book" documents nightly-only features][ubook]
* [Improve the style of the sidebar in rustdoc output][40265]
* [Configure build correctly on 64-bit CPU's with the armhf ABI][40261]
* [Fix MSP430 breakage due to `i128`][40257]
* [Preliminary Solaris/SPARCv9 support][39903]
* [`rustc` is linked statically on Windows MSVC targets][39837], allowing it to
  run without installing the MSVC runtime.
* [`rustdoc --test` includes file names in test names][39788]
* This release includes builds of `std` for `sparc64-unknown-linux-gnu`,
  `aarch64-unknown-linux-fuchsia`, and `x86_64-unknown-linux-fuchsia`.
* [Initial support for `aarch64-unknown-freebsd`][39491]
* [Initial support for `i686-unknown-netbsd`][39426]
* [This release no longer includes the old makefile build system][39431]. Rust
  is built with a custom build system, written in Rust, and with Cargo.
* [Add Debug implementations for libcollection structs][39002]
* [`TypeId` implements `PartialOrd` and `Ord`][38981]
* [`--test-threads=0` produces an error][38945]
* [`rustup` installs documentation by default][40526]
* [The Rust source includes NatVis visualizations][39843]. These can be used by
  WinDbg and Visual Studio to improve the debugging experience.

Compatibility Notes
-------------------

* [Rust 1.17 does not correctly detect the MSVC 2017 linker][38584]. As a
  workaround, either use MSVC 2015 or run vcvars.bat.
* [When coercing to an unsized type lifetimes must be equal][40319]. That is,
  disallow subtyping between `T` and `U` when `T: Unsize<U>`, e.g. coercing
  `&mut [&'a X; N]` to `&mut [&'b X]` requires `'a` be equal to `'b`. Soundness
  fix.
* [`format!` and `Display::to_string` panic if an underlying formatting
  implementation returns an error][40117]. Previously the error was silently
  ignored. It is incorrect for `write_fmt` to return an error when writing
  to a string.
* [In-tree crates are verified to be unstable][39851]. Previously, some minor
  crates were marked stable and could be accessed from the stable toolchain.
* [Rust git source no longer includes vendored crates][39728]. Those that need
  to build with vendored crates should build from release tarballs.
* [Fix inert attributes from `proc_macro_derives`][39572]
* [During crate resolution, rustc prefers a crate in the sysroot if two crates
  are otherwise identical][39518]. Unlikely to be encountered outside the Rust
  build system.
* [Fixed bugs around how type inference interacts with dead-code][39485]. The
  existing code generally ignores the type of dead-code unless a type-hint is
  provided; this can cause surprising inference interactions particularly around
  defaulting. The new code uniformly ignores the result type of dead-code.
* [Tuple-struct constructors with private fields are no longer visible][38932]
* [Lifetime parameters that do not appear in the arguments are now considered
  early-bound][38897], resolving a soundness bug (#[32330]). The
  `hr_lifetime_in_assoc_type` future-compatibility lint has been in effect since
  April of 2016.
* [rustdoc: fix doctests with non-feature crate attributes][38161]
* [Make transmuting from fn item types to pointer-sized types a hard
  error][34198]

[27787]: https://github.com/rust-lang/rust/issues/27787
[32330]: https://github.com/rust-lang/rust/issues/32330
[34198]: https://github.com/rust-lang/rust/pull/34198
[38161]: https://github.com/rust-lang/rust/pull/38161
[38368]: https://github.com/rust-lang/rust/pull/38368
[38584]: https://github.com/rust-lang/rust/issues/38584
[38661]: https://github.com/rust-lang/rust/pull/38661
[38764]: https://github.com/rust-lang/rust/pull/38764
[38864]: https://github.com/rust-lang/rust/issues/38864
[38897]: https://github.com/rust-lang/rust/pull/38897
[38921]: https://github.com/rust-lang/rust/pull/38921
[38932]: https://github.com/rust-lang/rust/pull/38932
[38945]: https://github.com/rust-lang/rust/pull/38945
[38981]: https://github.com/rust-lang/rust/pull/38981
[39002]: https://github.com/rust-lang/rust/pull/39002
[39116]: https://github.com/rust-lang/rust/pull/39116
[39193]: https://github.com/rust-lang/rust/pull/39193
[39265]: https://github.com/rust-lang/rust/pull/39265
[39356]: https://github.com/rust-lang/rust/pull/39356
[39372]: https://github.com/rust-lang/rust/pull/39372
[39426]: https://github.com/rust-lang/rust/pull/39426
[39431]: https://github.com/rust-lang/rust/pull/39431
[39438]: https://github.com/rust-lang/rust/pull/39438
[39440]: https://github.com/rust-lang/rust/pull/39440
[39485]: https://github.com/rust-lang/rust/pull/39485
[39491]: https://github.com/rust-lang/rust/pull/39491
[39518]: https://github.com/rust-lang/rust/pull/39518
[39538]: https://github.com/rust-lang/rust/pull/39538
[39572]: https://github.com/rust-lang/rust/pull/39572
[39633]: https://github.com/rust-lang/rust/pull/39633
[39642]: https://github.com/rust-lang/rust/pull/39642
[39728]: https://github.com/rust-lang/rust/pull/39728
[39761]: https://github.com/rust-lang/rust/pull/39761
[39788]: https://github.com/rust-lang/rust/pull/39788
[39793]: https://github.com/rust-lang/rust/pull/39793
[39837]: https://github.com/rust-lang/rust/pull/39837
[39843]: https://github.com/rust-lang/rust/pull/39843
[39851]: https://github.com/rust-lang/rust/pull/39851
[39903]: https://github.com/rust-lang/rust/pull/39903
[39939]: https://github.com/rust-lang/rust/pull/39939
[39953]: https://github.com/rust-lang/rust/pull/39953
[39960]: https://github.com/rust-lang/rust/pull/39960
[39990]: https://github.com/rust-lang/rust/pull/39990
[39995]: https://github.com/rust-lang/rust/pull/39995
[40008]: https://github.com/rust-lang/rust/pull/40008
[40009]: https://github.com/rust-lang/rust/pull/40009
[40027]: https://github.com/rust-lang/rust/pull/40027
[40028]: https://github.com/rust-lang/rust/pull/40028
[40037]: https://github.com/rust-lang/rust/pull/40037
[40117]: https://github.com/rust-lang/rust/pull/40117
[40166]: https://github.com/rust-lang/rust/pull/40166
[40245]: https://github.com/rust-lang/rust/pull/40245
[40257]: https://github.com/rust-lang/rust/pull/40257
[40261]: https://github.com/rust-lang/rust/pull/40261
[40265]: https://github.com/rust-lang/rust/pull/40265
[40319]: https://github.com/rust-lang/rust/pull/40319
[40336]: https://github.com/rust-lang/rust/pull/40336
[40526]: https://github.com/rust-lang/rust/pull/40526
[RFC 1623]: https://github.com/rust-lang/rfcs/blob/master/text/1623-static.md
[RFC 1647]: https://github.com/rust-lang/rfcs/blob/master/text/1647-allow-self-in-where-clauses.md
[RFC 1651]: https://github.com/rust-lang/rfcs/blob/master/text/1651-movecell.md
[RFC 1682]: https://github.com/rust-lang/rfcs/blob/master/text/1682-field-init-shorthand.md
[`Arc::from_raw`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw
[`Arc::into_raw`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.into_raw
[`Arc::ptr_eq`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.ptr_eq
[`BTreeMap::range_mut`]: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.range_mut
[`BTreeMap::range`]: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.range
[`Cell::into_inner`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.into_inner
[`Cell::replace`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.replace
[`Cell::swap`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.swap
[`Cell::take`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.take
[`Ordering::then_with`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with
[`Ordering::then`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then
[`Rc::from_raw`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.from_raw
[`Rc::into_raw`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.into_raw
[`Rc::ptr_eq`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq
[`Result::expect_err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err
[`collections::Bound`]: https://doc.rust-lang.org/std/collections/enum.Bound.html
[`process::abort`]: https://doc.rust-lang.org/std/process/fn.abort.html
[`ptr::read_unaligned`]: https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html
[`ptr::write_unaligned`]: https://doc.rust-lang.org/std/ptr/fn.write_unaligned.html
[cargo/3562]: https://github.com/rust-lang/cargo/pull/3562
[cargo/3664]: https://github.com/rust-lang/cargo/pull/3664
[cargo/3667]: https://github.com/rust-lang/cargo/pull/3667
[cargo/3691]: https://github.com/rust-lang/cargo/pull/3691
[cargo/3699]: https://github.com/rust-lang/cargo/pull/3699
[cargo/3731]: https://github.com/rust-lang/cargo/pull/3731
[mdbook]: https://crates.io/crates/mdbook
[ubook]: https://doc.rust-lang.org/unstable-book/


## Version 1.16.0 (2017-03-16)

Language
--------

* [The compiler's `dead_code` lint now accounts for type aliases][38051].
* [Uninhabitable enums (those without any variants) no longer permit wildcard
  match patterns][38069]
* [Clean up semantics of `self` in an import list][38313]
* [`Self` may appear in `impl` headers][38920]
* [`Self` may appear in struct expressions][39282]

Compiler
--------

* [`rustc` now supports `--emit=metadata`, which causes rustc to emit
  a `.rmeta` file containing only crate metadata][38571]. This can be
  used by tools like the Rust Language Service to perform
  metadata-only builds.
* [Levenshtein based typo suggestions now work in most places, while
  previously they worked only for fields and sometimes for local
  variables][38927]. Together with the overhaul of "no
  resolution"/"unexpected resolution" errors (#[38154]) they result in
  large and systematic improvement in resolution diagnostics.
* [Fix `transmute::<T, U>` where `T` requires a bigger alignment than
  `U`][38670]
* [rustc: use -Xlinker when specifying an rpath with ',' in it][38798]
* [`rustc` no longer attempts to provide "consider using an explicit
  lifetime" suggestions][37057]. They were inaccurate.

Stabilized APIs
---------------

* [`VecDeque::truncate`]
* [`VecDeque::resize`]
* [`String::insert_str`]
* [`Duration::checked_add`]
* [`Duration::checked_sub`]
* [`Duration::checked_div`]
* [`Duration::checked_mul`]
* [`str::replacen`]
* [`str::repeat`]
* [`SocketAddr::is_ipv4`]
* [`SocketAddr::is_ipv6`]
* [`IpAddr::is_ipv4`]
* [`IpAddr::is_ipv6`]
* [`Vec::dedup_by`]
* [`Vec::dedup_by_key`]
* [`Result::unwrap_or_default`]
* [`<*const T>::wrapping_offset`]
* [`<*mut T>::wrapping_offset`]
* `CommandExt::creation_flags`
* [`File::set_permissions`]
* [`String::split_off`]

Libraries
---------

* [`[T]::binary_search` and `[T]::binary_search_by_key` now take
  their argument by `Borrow` parameter][37761]
* [All public types in std implement `Debug`][38006]
* [`IpAddr` implements `From<Ipv4Addr>` and `From<Ipv6Addr>`][38327]
* [`Ipv6Addr` implements `From<[u16; 8]>`][38131]
* [Ctrl-Z returns from `Stdin.read()` when reading from the console on
  Windows][38274]
* [std: Fix partial writes in `LineWriter`][38062]
* [std: Clamp max read/write sizes on Unix][38062]
* [Use more specific panic message for `&str` slicing errors][38066]
* [`TcpListener::set_only_v6` is deprecated][38304]. This
  functionality cannot be achieved in std currently.
* [`writeln!`, like `println!`, now accepts a form with no string
  or formatting arguments, to just print a newline][38469]
* [Implement `iter::Sum` and `iter::Product` for `Result`][38580]
* [Reduce the size of static data in `std_unicode::tables`][38781]
* [`char::EscapeDebug`, `EscapeDefault`, `EscapeUnicode`,
  `CaseMappingIter`, `ToLowercase`, `ToUppercase`, implement
  `Display`][38909]
* [`Duration` implements `Sum`][38712]
* [`String` implements `ToSocketAddrs`][39048]

Cargo
-----

* [The `cargo check` command does a type check of a project without
  building it][cargo/3296]
* [crates.io will display CI badges from Travis and AppVeyor, if
  specified in Cargo.toml][cargo/3546]
* [crates.io will display categories listed in Cargo.toml][cargo/3301]
* [Compilation profiles accept integer values for `debug`, in addition
  to `true` and `false`. These are passed to `rustc` as the value to
  `-C debuginfo`][cargo/3534]
* [Implement `cargo --version --verbose`][cargo/3604]
* [All builds now output 'dep-info' build dependencies compatible with
  make and ninja][cargo/3557]
* [Build all workspace members with `build --all`][cargo/3511]
* [Document all workspace members with `doc --all`][cargo/3515]
* [Path deps outside workspace are not members][cargo/3443]

Misc
----

* [`rustdoc` has a `--sysroot` argument that, like `rustc`, specifies
  the path to the Rust implementation][38589]
* [The `armv7-linux-androideabi` target no longer enables NEON
  extensions, per Google's ABI guide][38413]
* [The stock standard library can be compiled for Redox OS][38401]
* [Rust has initial SPARC support][38726]. Tier 3. No builds
  available.
* [Rust has experimental support for Nvidia PTX][38559]. Tier 3. No
  builds available.
* [Fix backtraces on i686-pc-windows-gnu by disabling FPO][39379]

Compatibility Notes
-------------------

* [Uninhabitable enums (those without any variants) no longer permit wildcard
  match patterns][38069]
* In this release, references to uninhabited types can not be
  pattern-matched. This was accidentally allowed in 1.15.
* [The compiler's `dead_code` lint now accounts for type aliases][38051].
* [Ctrl-Z returns from `Stdin.read()` when reading from the console on
  Windows][38274]
* [Clean up semantics of `self` in an import list][38313]
* Reimplemented lifetime elision. This change was almost entirely compatible
  with existing code, but it did close a number of small bugs and loopholes,
  as well as being more accepting in some other [cases][41105].

[37057]: https://github.com/rust-lang/rust/pull/37057
[37761]: https://github.com/rust-lang/rust/pull/37761
[38006]: https://github.com/rust-lang/rust/pull/38006
[38051]: https://github.com/rust-lang/rust/pull/38051
[38062]: https://github.com/rust-lang/rust/pull/38062
[38062]: https://github.com/rust-lang/rust/pull/38622
[38066]: https://github.com/rust-lang/rust/pull/38066
[38069]: https://github.com/rust-lang/rust/pull/38069
[38131]: https://github.com/rust-lang/rust/pull/38131
[38154]: https://github.com/rust-lang/rust/pull/38154
[38274]: https://github.com/rust-lang/rust/pull/38274
[38304]: https://github.com/rust-lang/rust/pull/38304
[38313]: https://github.com/rust-lang/rust/pull/38313
[38314]: https://github.com/rust-lang/rust/pull/38314
[38327]: https://github.com/rust-lang/rust/pull/38327
[38401]: https://github.com/rust-lang/rust/pull/38401
[38413]: https://github.com/rust-lang/rust/pull/38413
[38469]: https://github.com/rust-lang/rust/pull/38469
[38559]: https://github.com/rust-lang/rust/pull/38559
[38571]: https://github.com/rust-lang/rust/pull/38571
[38580]: https://github.com/rust-lang/rust/pull/38580
[38589]: https://github.com/rust-lang/rust/pull/38589
[38670]: https://github.com/rust-lang/rust/pull/38670
[38712]: https://github.com/rust-lang/rust/pull/38712
[38726]: https://github.com/rust-lang/rust/pull/38726
[38781]: https://github.com/rust-lang/rust/pull/38781
[38798]: https://github.com/rust-lang/rust/pull/38798
[38909]: https://github.com/rust-lang/rust/pull/38909
[38920]: https://github.com/rust-lang/rust/pull/38920
[38927]: https://github.com/rust-lang/rust/pull/38927
[39048]: https://github.com/rust-lang/rust/pull/39048
[39282]: https://github.com/rust-lang/rust/pull/39282
[39379]: https://github.com/rust-lang/rust/pull/39379
[41105]: https://github.com/rust-lang/rust/issues/41105
[`<*const T>::wrapping_offset`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset
[`<*mut T>::wrapping_offset`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset
[`Duration::checked_add`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_add
[`Duration::checked_div`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_div
[`Duration::checked_mul`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_mul
[`Duration::checked_sub`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_sub
[`File::set_permissions`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.set_permissions
[`IpAddr::is_ipv4`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_ipv4
[`IpAddr::is_ipv6`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_ipv6
[`Result::unwrap_or_default`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default
[`SocketAddr::is_ipv4`]: https://doc.rust-lang.org/std/net/enum.SocketAddr.html#method.is_ipv4
[`SocketAddr::is_ipv6`]: https://doc.rust-lang.org/std/net/enum.SocketAddr.html#method.is_ipv6
[`String::insert_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.insert_str
[`String::split_off`]: https://doc.rust-lang.org/std/string/struct.String.html#method.split_off
[`Vec::dedup_by_key`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by_key
[`Vec::dedup_by`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by
[`VecDeque::resize`]:  https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html#method.resize
[`VecDeque::truncate`]: https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html#method.truncate
[`str::repeat`]: https://doc.rust-lang.org/std/primitive.str.html#method.repeat
[`str::replacen`]: https://doc.rust-lang.org/std/primitive.str.html#method.replacen
[cargo/3296]: https://github.com/rust-lang/cargo/pull/3296
[cargo/3301]: https://github.com/rust-lang/cargo/pull/3301
[cargo/3443]: https://github.com/rust-lang/cargo/pull/3443
[cargo/3511]: https://github.com/rust-lang/cargo/pull/3511
[cargo/3515]: https://github.com/rust-lang/cargo/pull/3515
[cargo/3534]: https://github.com/rust-lang/cargo/pull/3534
[cargo/3546]: https://github.com/rust-lang/cargo/pull/3546
[cargo/3557]: https://github.com/rust-lang/cargo/pull/3557
[cargo/3604]: https://github.com/rust-lang/cargo/pull/3604
[RFC 1623]: https://github.com/rust-lang/rfcs/blob/master/text/1623-static.md


## Version 1.15.1 (2017-02-09)

* [Fix IntoIter::as_mut_slice's signature][39466]
* [Compile compiler builtins with `-fPIC` on 32-bit platforms][39523]

[39466]: https://github.com/rust-lang/rust/pull/39466
[39523]: https://github.com/rust-lang/rust/pull/39523


## Version 1.15.0 (2017-02-02)

Language
--------

* Basic procedural macros allowing custom `#[derive]`, aka "macros 1.1", are
  stable. This allows popular code-generating crates like Serde and Diesel to
  work ergonomically. [RFC 1681].
* [Tuple structs may be empty. Unary and empty tuple structs may be instantiated
  with curly braces][36868]. Part of [RFC 1506].
* [A number of minor changes to name resolution have been activated][37127].
  They add up to more consistent semantics, allowing for future evolution of
  Rust macros. Specified in [RFC 1560], see its section on ["changes"] for
  details of what is different. The breaking changes here have been transitioned
  through the [`legacy_imports`] lint since 1.14, with no known regressions.
* [In `macro_rules`, `path` fragments can now be parsed as type parameter
  bounds][38279]
* [`?Sized` can be used in `where` clauses][37791]
* [There is now a limit on the size of monomorphized types and it can be
  modified with the `#![type_size_limit]` crate attribute, similarly to
  the `#![recursion_limit]` attribute][37789]

Compiler
--------

* [On Windows, the compiler will apply dllimport attributes when linking to
  extern functions][37973]. Additional attributes and flags can control which
  library kind is linked and its name. [RFC 1717].
* [Rust-ABI symbols are no longer exported from cdylibs][38117]
* [The `--test` flag works with procedural macro crates][38107]
* [Fix `extern "aapcs" fn` ABI][37814]
* [The `-C no-stack-check` flag is deprecated][37636]. It does nothing.
* [The `format!` expander recognizes incorrect `printf` and shell-style
  formatting directives and suggests the correct format][37613].
* [Only report one error for all unused imports in an import list][37456]

Compiler Performance
--------------------

* [Avoid unnecessary `mk_ty` calls in `Ty::super_fold_with`][37705]
* [Avoid more unnecessary `mk_ty` calls in `Ty::super_fold_with`][37979]
* [Don't clone in `UnificationTable::probe`][37848]
* [Remove `scope_auxiliary` to cut RSS by 10%][37764]
* [Use small vectors in type walker][37760]
* [Macro expansion performance was improved][37701]
* [Change `HirVec<P<T>>` to `HirVec<T>` in `hir::Expr`][37642]
* [Replace FNV with a faster hash function][37229]

Stabilized APIs
---------------

* [`std::iter::Iterator::min_by`]
* [`std::iter::Iterator::max_by`]
* [`std::os::*::fs::FileExt`]
* [`std::sync::atomic::Atomic*::get_mut`]
* [`std::sync::atomic::Atomic*::into_inner`]
* [`std::vec::IntoIter::as_slice`]
* [`std::vec::IntoIter::as_mut_slice`]
* [`std::sync::mpsc::Receiver::try_iter`]
* [`std::os::unix::process::CommandExt::before_exec`]
* [`std::rc::Rc::strong_count`]
* [`std::rc::Rc::weak_count`]
* [`std::sync::Arc::strong_count`]
* [`std::sync::Arc::weak_count`]
* [`std::char::encode_utf8`]
* [`std::char::encode_utf16`]
* [`std::cell::Ref::clone`]
* [`std::io::Take::into_inner`]

Libraries
---------

* [The standard sorting algorithm has been rewritten for dramatic performance
  improvements][38192]. It is a hybrid merge sort, drawing influences from
  Timsort. Previously it was a naive merge sort.
* [`Iterator::nth` no longer has a `Sized` bound][38134]
* [`Extend<&T>` is specialized for `Vec` where `T: Copy`][38182] to improve
  performance.
* [`chars().count()` is much faster][37888] and so are [`chars().last()`
  and `char_indices().last()`][37882]
* [Fix ARM Objective-C ABI in `std::env::args`][38146]
* [Chinese characters display correctly in `fmt::Debug`][37855]
* [Derive `Default` for `Duration`][37699]
* [Support creation of anonymous pipes on WinXP/2k][37677]
* [`mpsc::RecvTimeoutError` implements `Error`][37527]
* [Don't pass overlapped handles to processes][38835]

Cargo
-----

* [In this release, Cargo build scripts no longer have access to the `OUT_DIR`
  environment variable at build time via `env!("OUT_DIR")`][cargo/3368]. They
  should instead check the variable at runtime with `std::env`. That the value
  was set at build time was a bug, and incorrect when cross-compiling. This
  change is known to cause breakage.
* [Add `--all` flag to `cargo test`][cargo/3221]
* [Compile statically against the MSVC CRT][cargo/3363]
* [Mix feature flags into fingerprint/metadata shorthash][cargo/3102]
* [Link OpenSSL statically on OSX][cargo/3311]
* [Apply new fingerprinting to build dir outputs][cargo/3310]
* [Test for bad path overrides with summaries][cargo/3336]
* [Require `cargo install --vers` to take a semver version][cargo/3338]
* [Fix retrying crate downloads for network errors][cargo/3348]
* [Implement string lookup for `build.rustflags` config key][cargo/3356]
* [Emit more info on --message-format=json][cargo/3319]
* [Assume `build.rs` in the same directory as `Cargo.toml` is a build script][cargo/3361]
* [Don't ignore errors in workspace manifest][cargo/3409]
* [Fix `--message-format JSON` when rustc emits non-JSON warnings][cargo/3410]

Tooling
-------

* [Test runners (binaries built with `--test`) now support a `--list` argument
  that lists the tests it contains][38185]
* [Test runners now support a `--exact` argument that makes the test filter
  match exactly, instead of matching only a substring of the test name][38181]
* [rustdoc supports a `--playground-url` flag][37763]
* [rustdoc provides more details about `#[should_panic]` errors][37749]

Misc
----

* [The Rust build system is now written in Rust][37817]. The Makefiles may
  continue to be used in this release by passing `--disable-rustbuild` to the
  configure script, but they will be deleted soon. Note that the new build
  system uses a different on-disk layout that will likely affect any scripts
  building Rust.
* [Rust supports i686-unknown-openbsd][38086]. Tier 3 support. No testing or
  releases.
* [Rust supports the MSP430][37627]. Tier 3 support. No testing or releases.
* [Rust supports the ARMv5TE architecture][37615]. Tier 3 support. No testing or
  releases.

Compatibility Notes
-------------------

* [A number of minor changes to name resolution have been activated][37127].
  They add up to more consistent semantics, allowing for future evolution of
  Rust macros. Specified in [RFC 1560], see its section on ["changes"] for
  details of what is different. The breaking changes here have been transitioned
  through the [`legacy_imports`] lint since 1.14, with no known regressions.
* [In this release, Cargo build scripts no longer have access to the `OUT_DIR`
  environment variable at build time via `env!("OUT_DIR")`][cargo/3368]. They
  should instead check the variable at runtime with `std::env`. That the value
  was set at build time was a bug, and incorrect when cross-compiling. This
  change is known to cause breakage.
* [Higher-ranked lifetimes are no longer allowed to appear _only_ in associated
  types][33685]. The [`hr_lifetime_in_assoc_type` lint] has been a warning since
  1.10 and is now an error by default. It will become a hard error in the near
  future.
* [The semantics relating modules to file system directories are changing in
  minor ways][37602]. This is captured in the new `legacy_directory_ownership`
  lint, which is a warning in this release, and will become a hard error in the
  future.
* [Rust-ABI symbols are no longer exported from cdylibs][38117]
* [Once `Peekable` peeks a `None` it will return that `None` without re-querying
  the underlying iterator][37834]

["changes"]: https://github.com/rust-lang/rfcs/blob/master/text/1560-name-resolution.md#changes-to-name-resolution-rules
[33685]: https://github.com/rust-lang/rust/issues/33685
[36868]: https://github.com/rust-lang/rust/pull/36868
[37127]: https://github.com/rust-lang/rust/pull/37127
[37229]: https://github.com/rust-lang/rust/pull/37229
[37456]: https://github.com/rust-lang/rust/pull/37456
[37527]: https://github.com/rust-lang/rust/pull/37527
[37602]: https://github.com/rust-lang/rust/pull/37602
[37613]: https://github.com/rust-lang/rust/pull/37613
[37615]: https://github.com/rust-lang/rust/pull/37615
[37636]: https://github.com/rust-lang/rust/pull/37636
[37627]: https://github.com/rust-lang/rust/pull/37627
[37642]: https://github.com/rust-lang/rust/pull/37642
[37677]: https://github.com/rust-lang/rust/pull/37677
[37699]: https://github.com/rust-lang/rust/pull/37699
[37701]: https://github.com/rust-lang/rust/pull/37701
[37705]: https://github.com/rust-lang/rust/pull/37705
[37749]: https://github.com/rust-lang/rust/pull/37749
[37760]: https://github.com/rust-lang/rust/pull/37760
[37763]: https://github.com/rust-lang/rust/pull/37763
[37764]: https://github.com/rust-lang/rust/pull/37764
[37789]: https://github.com/rust-lang/rust/pull/37789
[37791]: https://github.com/rust-lang/rust/pull/37791
[37814]: https://github.com/rust-lang/rust/pull/37814
[37817]: https://github.com/rust-lang/rust/pull/37817
[37834]: https://github.com/rust-lang/rust/pull/37834
[37848]: https://github.com/rust-lang/rust/pull/37848
[37855]: https://github.com/rust-lang/rust/pull/37855
[37882]: https://github.com/rust-lang/rust/pull/37882
[37888]: https://github.com/rust-lang/rust/pull/37888
[37973]: https://github.com/rust-lang/rust/pull/37973
[37979]: https://github.com/rust-lang/rust/pull/37979
[38086]: https://github.com/rust-lang/rust/pull/38086
[38107]: https://github.com/rust-lang/rust/pull/38107
[38117]: https://github.com/rust-lang/rust/pull/38117
[38134]: https://github.com/rust-lang/rust/pull/38134
[38146]: https://github.com/rust-lang/rust/pull/38146
[38181]: https://github.com/rust-lang/rust/pull/38181
[38182]: https://github.com/rust-lang/rust/pull/38182
[38185]: https://github.com/rust-lang/rust/pull/38185
[38192]: https://github.com/rust-lang/rust/pull/38192
[38279]: https://github.com/rust-lang/rust/pull/38279
[38835]: https://github.com/rust-lang/rust/pull/38835
[RFC 1492]: https://github.com/rust-lang/rfcs/blob/master/text/1492-dotdot-in-patterns.md
[RFC 1506]: https://github.com/rust-lang/rfcs/blob/master/text/1506-adt-kinds.md
[RFC 1560]: https://github.com/rust-lang/rfcs/blob/master/text/1560-name-resolution.md
[RFC 1681]: https://github.com/rust-lang/rfcs/blob/master/text/1681-macros-1.1.md
[RFC 1717]: https://github.com/rust-lang/rfcs/blob/master/text/1717-dllimport.md
[`hr_lifetime_in_assoc_type` lint]: https://github.com/rust-lang/rust/issues/33685
[`legacy_imports`]: https://github.com/rust-lang/rust/pull/38271
[cargo/3102]: https://github.com/rust-lang/cargo/pull/3102
[cargo/3221]: https://github.com/rust-lang/cargo/pull/3221
[cargo/3310]: https://github.com/rust-lang/cargo/pull/3310
[cargo/3311]: https://github.com/rust-lang/cargo/pull/3311
[cargo/3319]: https://github.com/rust-lang/cargo/pull/3319
[cargo/3336]: https://github.com/rust-lang/cargo/pull/3336
[cargo/3338]: https://github.com/rust-lang/cargo/pull/3338
[cargo/3348]: https://github.com/rust-lang/cargo/pull/3348
[cargo/3356]: https://github.com/rust-lang/cargo/pull/3356
[cargo/3361]: https://github.com/rust-lang/cargo/pull/3361
[cargo/3363]: https://github.com/rust-lang/cargo/pull/3363
[cargo/3368]: https://github.com/rust-lang/cargo/issues/3368
[cargo/3409]: https://github.com/rust-lang/cargo/pull/3409
[cargo/3410]: https://github.com/rust-lang/cargo/pull/3410
[`std::iter::Iterator::min_by`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by
[`std::iter::Iterator::max_by`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by
[`std::os::*::fs::FileExt`]: https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html
[`std::sync::atomic::Atomic*::get_mut`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU8.html#method.get_mut
[`std::sync::atomic::Atomic*::into_inner`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU8.html#method.into_inner
[`std::vec::IntoIter::as_slice`]: https://doc.rust-lang.org/std/vec/struct.IntoIter.html#method.as_slice
[`std::vec::IntoIter::as_mut_slice`]: https://doc.rust-lang.org/std/vec/struct.IntoIter.html#method.as_mut_slice
[`std::sync::mpsc::Receiver::try_iter`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.try_iter
[`std::os::unix::process::CommandExt::before_exec`]: https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.before_exec
[`std::rc::Rc::strong_count`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.strong_count
[`std::rc::Rc::weak_count`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.weak_count
[`std::sync::Arc::strong_count`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.strong_count
[`std::sync::Arc::weak_count`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.weak_count
[`std::char::encode_utf8`]: https://doc.rust-lang.org/std/primitive.char.html#method.encode_utf8
[`std::char::encode_utf16`]: https://doc.rust-lang.org/std/primitive.char.html#method.encode_utf16
[`std::cell::Ref::clone`]: https://doc.rust-lang.org/std/cell/struct.Ref.html#method.clone
[`std::io::Take::into_inner`]: https://doc.rust-lang.org/std/io/struct.Take.html#method.into_inner


## Version 1.14.0 (2016-12-22)

Language
--------

* [`..` matches multiple tuple fields in enum variants, structs
  and tuples][36843]. [RFC 1492].
* [Safe `fn` items can be coerced to `unsafe fn` pointers][37389]
* [`use *` and `use ::*` both glob-import from the crate root][37367]
* [It's now possible to call a `Vec<Box<Fn()>>` without explicit
  dereferencing][36822]

Compiler
--------

* [Mark enums with non-zero discriminant as non-zero][37224]
* [Lower-case `static mut` names are linted like other
  statics and consts][37162]
* [Fix ICE on some macros in const integer positions
   (e.g. `[u8; m!()]`)][36819]
* [Improve error message and snippet for "did you mean `x`"][36798]
* [Add a panic-strategy field to the target specification][36794]
* [Include LLVM version in `--version --verbose`][37200]

Compile-time Optimizations
--------------------------

* [Improve macro expansion performance][37569]
* [Shrink `Expr_::ExprInlineAsm`][37445]
* [Replace all uses of SHA-256 with BLAKE2b][37439]
* [Reduce the number of bytes hashed by `IchHasher`][37427]
* [Avoid more allocations when compiling html5ever][37373]
* [Use `SmallVector` in `CombineFields::instantiate`][37322]
* [Avoid some allocations in the macro parser][37318]
* [Use a faster deflate setting][37298]
* [Add `ArrayVec` and `AccumulateVec` to reduce heap allocations
  during interning of slices][37270]
* [Optimize `write_metadata`][37267]
* [Don't process obligation forest cycles when stalled][37231]
* [Avoid many `CrateConfig` clones][37161]
* [Optimize `Substs::super_fold_with`][37108]
* [Optimize `ObligationForest`'s `NodeState` handling][36993]
* [Speed up `plug_leaks`][36917]

Libraries
---------

* [`println!()`, with no arguments, prints newline][36825].
  Previously, an empty string was required to achieve the same.
* [`Wrapping` impls standard binary and unary operators, as well as
   the `Sum` and `Product` iterators][37356]
* [Implement `From<Cow<str>> for String` and `From<Cow<[T]>> for
  Vec<T>`][37326]
* [Improve `fold` performance for `chain`, `cloned`, `map`, and
  `VecDeque` iterators][37315]
* [Improve `SipHasher` performance on small values][37312]
* [Add Iterator trait TrustedLen to enable better FromIterator /
  Extend][37306]
* [Expand `.zip()` specialization to `.map()` and `.cloned()`][37230]
* [`ReadDir` implements `Debug`][37221]
* [Implement `RefUnwindSafe` for atomic types][37178]
* [Specialize `Vec::extend` to `Vec::extend_from_slice`][37094]
* [Avoid allocations in `Decoder::read_str`][37064]
* [`io::Error` implements `From<io::ErrorKind>`][37037]
* [Impl `Debug` for raw pointers to unsized data][36880]
* [Don't reuse `HashMap` random seeds][37470]
* [The internal memory layout of `HashMap` is more cache-friendly, for
  significant improvements in some operations][36692]
* [`HashMap` uses less memory on 32-bit architectures][36595]
* [Impl `Add<{str, Cow<str>}>` for `Cow<str>`][36430]

Cargo
-----

* [Expose rustc cfg values to build scripts][cargo/3243]
* [Allow cargo to work with read-only `CARGO_HOME`][cargo/3259]
* [Fix passing --features when testing multiple packages][cargo/3280]
* [Use a single profile set per workspace][cargo/3249]
* [Load `replace` sections from lock files][cargo/3220]
* [Ignore `panic` configuration for test/bench profiles][cargo/3175]

Tooling
-------

* [rustup is the recommended Rust installation method][1.14rustup]
* This release includes host (rustc) builds for Linux on MIPS, PowerPC, and
  S390x. These are [tier 2] platforms and may have major defects. Follow the
  instructions on the website to install, or add the targets to an existing
  installation with `rustup target add`. The new target triples are:
  - `mips-unknown-linux-gnu`
  - `mipsel-unknown-linux-gnu`
  - `mips64-unknown-linux-gnuabi64`
  - `mips64el-unknown-linux-gnuabi64 `
  - `powerpc-unknown-linux-gnu`
  - `powerpc64-unknown-linux-gnu`
  - `powerpc64le-unknown-linux-gnu`
  - `s390x-unknown-linux-gnu `
* This release includes target (std) builds for ARM Linux running MUSL
  libc. These are [tier 2] platforms and may have major defects. Add the
  following triples to an existing rustup installation with `rustup target add`:
  - `arm-unknown-linux-musleabi`
  - `arm-unknown-linux-musleabihf`
  - `armv7-unknown-linux-musleabihf`
* This release includes [experimental support for WebAssembly][1.14wasm], via
  the `wasm32-unknown-emscripten` target. This target is known to have major
  defects. Please test, report, and fix.
* rustup no longer installs documentation by default. Run `rustup
  component add rust-docs` to install.
* [Fix line stepping in debugger][37310]
* [Enable line number debuginfo in releases][37280]

Misc
----

* [Disable jemalloc on aarch64/powerpc/mips][37392]
* [Add support for Fuchsia OS][37313]
* [Detect local-rebuild by only MAJOR.MINOR version][37273]

Compatibility Notes
-------------------

* [A number of forward-compatibility lints used by the compiler
  to gradually introduce language changes have been converted
  to deny by default][36894]:
  - ["use of inaccessible extern crate erroneously allowed"][36886]
  - ["type parameter default erroneously allowed in invalid location"][36887]
  - ["detects super or self keywords at the beginning of global path"][36888]
  - ["two overlapping inherent impls define an item with the same name
    were erroneously allowed"][36889]
  - ["floating-point constants cannot be used in patterns"][36890]
  - ["constants of struct or enum type can only be used in a pattern if
     the struct or enum has `#[derive(PartialEq, Eq)]`"][36891]
  - ["lifetimes or labels named `'_` were erroneously allowed"][36892]
* [Prohibit patterns in trait methods without bodies][37378]
* [The atomic `Ordering` enum may not be matched exhaustively][37351]
* [Future-proofing `#[no_link]` breaks some obscure cases][37247]
* [The `$crate` macro variable is accepted in fewer locations][37213]
* [Impls specifying extra region requirements beyond the trait
  they implement are rejected][37167]
* [Enums may not be unsized][37111]. Unsized enums are intended to
  work but never have. For now they are forbidden.
* [Enforce the shadowing restrictions from RFC 1560 for today's macros][36767]

[tier 2]: https://forge.rust-lang.org/platform-support.html
[1.14rustup]: https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/204
[1.14wasm]: https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627
[36430]: https://github.com/rust-lang/rust/pull/36430
[36595]: https://github.com/rust-lang/rust/pull/36595
[36595]: https://github.com/rust-lang/rust/pull/36595
[36692]: https://github.com/rust-lang/rust/pull/36692
[36767]: https://github.com/rust-lang/rust/pull/36767
[36794]: https://github.com/rust-lang/rust/pull/36794
[36798]: https://github.com/rust-lang/rust/pull/36798
[36819]: https://github.com/rust-lang/rust/pull/36819
[36822]: https://github.com/rust-lang/rust/pull/36822
[36825]: https://github.com/rust-lang/rust/pull/36825
[36843]: https://github.com/rust-lang/rust/pull/36843
[36880]: https://github.com/rust-lang/rust/pull/36880
[36886]: https://github.com/rust-lang/rust/issues/36886
[36887]: https://github.com/rust-lang/rust/issues/36887
[36888]: https://github.com/rust-lang/rust/issues/36888
[36889]: https://github.com/rust-lang/rust/issues/36889
[36890]: https://github.com/rust-lang/rust/issues/36890
[36891]: https://github.com/rust-lang/rust/issues/36891
[36892]: https://github.com/rust-lang/rust/issues/36892
[36894]: https://github.com/rust-lang/rust/pull/36894
[36917]: https://github.com/rust-lang/rust/pull/36917
[36993]: https://github.com/rust-lang/rust/pull/36993
[37037]: https://github.com/rust-lang/rust/pull/37037
[37064]: https://github.com/rust-lang/rust/pull/37064
[37094]: https://github.com/rust-lang/rust/pull/37094
[37108]: https://github.com/rust-lang/rust/pull/37108
[37111]: https://github.com/rust-lang/rust/pull/37111
[37161]: https://github.com/rust-lang/rust/pull/37161
[37162]: https://github.com/rust-lang/rust/pull/37162
[37167]: https://github.com/rust-lang/rust/pull/37167
[37178]: https://github.com/rust-lang/rust/pull/37178
[37200]: https://github.com/rust-lang/rust/pull/37200
[37213]: https://github.com/rust-lang/rust/pull/37213
[37221]: https://github.com/rust-lang/rust/pull/37221
[37224]: https://github.com/rust-lang/rust/pull/37224
[37230]: https://github.com/rust-lang/rust/pull/37230
[37231]: https://github.com/rust-lang/rust/pull/37231
[37247]: https://github.com/rust-lang/rust/pull/37247
[37267]: https://github.com/rust-lang/rust/pull/37267
[37270]: https://github.com/rust-lang/rust/pull/37270
[37273]: https://github.com/rust-lang/rust/pull/37273
[37280]: https://github.com/rust-lang/rust/pull/37280
[37298]: https://github.com/rust-lang/rust/pull/37298
[37306]: https://github.com/rust-lang/rust/pull/37306
[37310]: https://github.com/rust-lang/rust/pull/37310
[37312]: https://github.com/rust-lang/rust/pull/37312
[37313]: https://github.com/rust-lang/rust/pull/37313
[37315]: https://github.com/rust-lang/rust/pull/37315
[37318]: https://github.com/rust-lang/rust/pull/37318
[37322]: https://github.com/rust-lang/rust/pull/37322
[37326]: https://github.com/rust-lang/rust/pull/37326
[37351]: https://github.com/rust-lang/rust/pull/37351
[37356]: https://github.com/rust-lang/rust/pull/37356
[37367]: https://github.com/rust-lang/rust/pull/37367
[37373]: https://github.com/rust-lang/rust/pull/37373
[37378]: https://github.com/rust-lang/rust/pull/37378
[37389]: https://github.com/rust-lang/rust/pull/37389
[37392]: https://github.com/rust-lang/rust/pull/37392
[37427]: https://github.com/rust-lang/rust/pull/37427
[37439]: https://github.com/rust-lang/rust/pull/37439
[37445]: https://github.com/rust-lang/rust/pull/37445
[37470]: https://github.com/rust-lang/rust/pull/37470
[37569]: https://github.com/rust-lang/rust/pull/37569
[RFC 1492]: https://github.com/rust-lang/rfcs/blob/master/text/1492-dotdot-in-patterns.md
[cargo/3175]: https://github.com/rust-lang/cargo/pull/3175
[cargo/3220]: https://github.com/rust-lang/cargo/pull/3220
[cargo/3243]: https://github.com/rust-lang/cargo/pull/3243
[cargo/3249]: https://github.com/rust-lang/cargo/pull/3249
[cargo/3259]: https://github.com/rust-lang/cargo/pull/3259
[cargo/3280]: https://github.com/rust-lang/cargo/pull/3280