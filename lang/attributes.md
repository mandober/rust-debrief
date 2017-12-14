# Attributes

An attribute is a general, free-form metadata that is interpreted according to name, convention, language and compiler version. Any item declaration may have an attribute applied to it. An attribute beginning with a `#` applies to the item following it, while an attribute beginning with a `#!` applies to the outer, enclosing, item.

Attributes may appear as any of:
* single identifier - the attribute name
* identifier, equals sign, and a literal - providing a key/value pair
* identifier followed by a parenthesized list of sub-attribute arguments


## Attribute Index

- Crate-only: `crate_name, crate_type, feature, no_builtins, no_main, no_start, no_std, plugin, recursion_limit, windows_subsystem`
- Module-only: `no_implicit_prelude, path`
- Function-only: `main, plugin_registrar, start, test, should_panic, cold, naked`
- Static-only: `thread_local`
- FFI: `repr, link, link_args, linked_from, link_name, linkage`
- Macro-related: `macro_use, macro_reexport, macro_export, no_link`
- Miscellaneous: `doc, must_use, deprecated, export_name, link_section, no_mangle, simd, unsafe_destructor_blind_to_params, rustc_on_unimplemented`
- Conditional compilation: `cfg, cfg_attr`
- Language items: `lang, inline, derive`


## Linting

Command to see lint checks supported by the compiler: `rustc -W help`.
Run `rustc -W help foo.rs` to see a list of lints known to rustc, including those provided by plugins loaded by foo.rs file.

Linting actions:
  - `allow`  overrides the check so that violations will go unreported.
  - `warn`   warns about violations, but continues compilation.
  - `deny`   signals an error after encountering a violation.
  - `forbid` same as deny, but also forbids changing the lint level afterwards.


## Related links

- [Rust reference: Attributes](https://doc.rust-lang.org/reference/attributes.html)
- [Rust reference: Linkage](https://doc.rust-lang.org/reference/linkage.html)
- [LLVM: Linkage type](http://llvm.org/docs/LangRef.html#linkage-types)
- [About linting plugins](https://doc.rust-lang.org/unstable-book/language-features/plugin.html#lint-plugins)
- [Code: rustc's built-in lints](https://github.com/rust-lang/rust/blob/master/src/librustc/lint/builtin.rs)