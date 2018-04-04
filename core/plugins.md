# Compiler Plugins

`rustc` can load compiler plugins, which are user-provided libraries that extend the compiler's behavior with new syntax extensions, lint checks, etc.

A plugin is a dynamic library crate with a designated registrar function that registers extensions with `rustc`. Other crates can load these extensions using the crate attribute `#![plugin(...)]`. See the rustc_plugin documentation for more about the mechanics of defining and loading a plugin.

If present, arguments passed as `#![plugin(foo(... args ...))]` are not interpreted by rustc itself. They are provided to the plugin through the Registry's args method.

In the vast majority of cases, a plugin should only be used through `#![plugin]` and not through an extern crate item. Linking a plugin would pull in all of `libsyntax` and `librustc` as dependencies of your crate. This is generally unwanted unless you are building another plugin. The `plugin_as_library` lint checks these guidelines.

The usual practice is to put compiler plugins in their own crate, separate from any `macro_rules!` macros or ordinary Rust code meant to be used by consumers of a library.

