# Attributes

- An attribute is a metadata that is interpreted according to name, convention, language and compiler version.
- Any item declaration may have an attribute applied to it.
- An attribute beginning with a `#` applies to the item following it.
- An attribute beginning with a `#!` applies to the outer, enclosing, item.
- Attributes may appear as any of:
  - single identifier i.e. the attribute's name: `#[test]`
  - key/value pair: `#![crate_type = "lib"]`
  - identifier, with list of args in parenthesis: `#[inline(always)]`
- Grouping: `[allow(dead_code, unused_variables)]`


## Attribute Categories
- Crate-only
- Module-only
- Function-only
- Static-only
- FFI
- Macro-related
- Miscellaneous
- Conditional compilation
- Language items


## Attribute Index
- Crate-only
  - crate_name: `#![crate_name = "std"]`
  - crate_type: `#![crate_type = "lib"]`
  - feature: `#![feature(inclusive_range_syntax)]`
  - no_builtins
  - no_main
  - no_start
  - no_std: `#[no_std]`
  - plugin
  - recursion_limit
  - windows_subsystem
- Module-only
  - no_implicit_prelude
  - path
- Function-only
  - main
  - plugin_registrar
  - start
  - test: `#[test]`
  - should_panic
  - cold
  - naked
- Static-only
  - thread_local
- FFI
  - repr
  - link
  - link_args
  - linked_from
  - link_name
  - linkage
- Macro-related
  - macro_use
  - macro_reexport
  - macro_export
  - no_link
- Miscellaneous
  - doc
  - must_use
  - deprecated
  - export_name
  - link_section
  - no_mangle: `#[no_mangle]`
  - simd
  - unsafe_destructor_blind_to_params
  - rustc_on_unimplemented
- Conditional compilation
  - cfg: `#[cfg(target_os = "linux")]`
  - cfg_attr
- Language items
  - lang: `#[lang = "f64"]`
  - inline: `#[inline(always)]`
  - derive: `#[derive(Debug, Copy, Clone)]`


## Linting Attributes
Command to see lint checks supported by the compiler: `rustc -W help`.
Run `rustc -W help foo.rs` to see a list of lints known to rustc, including those provided by plugins loaded by foo.rs file.

* Linting actions
  - `allow`  overrides the check so that violations go unreported.
  - `warn`   warns about violations, but continues compilation.
  - `deny`   signals an error after encountering a violation.
  - `forbid` same as deny, but forbids changing the lint level afterwards.
* Linting attributes
  - dead_code
  - unused_variables
  - unused_mut
  - unused_assignments
  - non_camel_case_types
  - etc.
