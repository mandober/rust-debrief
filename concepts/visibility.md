# Visibility

- in Rust everything is private by default
- associated items in a public trait are public
- variants of a public enum are public
- The keyword `pub` is visibility (privacy) qualifier 
- it makes the item visible outside its namespace.


## The pub modifier
The keyword `pub` is the only (at the moment) visibility qualifier which makes an item visible outside its namespace. In Rust, everything is private by default, although:
- associated items in a public trait are public
- variants of a public enum are public
There is no "private" modifier keyword active in Rust now, although the keyword `priv` is reserved.


## Visibility restrictions

- `pub(crate)`: visible only on the current crate
- `pub(super)`: visible only in the current module's parent
- `pub(in path::to::module)`: visible only on the specified path, e.g.:
  `pub(in r#dll_ci::list)` visible only in module

Make this visible only to module `r#mod` with `in`: `pub(in r#mod)`
add `#![feature(raw_identifiers)]` to the crate attributes to enable them so
you can use current module's raw identifier: `pub(in r#dll_ci::list)`




The `pub` can be used on:
- modules
- functions
- import: `pub use`, known as re-exporting
- struct
- struct's individual fields
- enum, but not on its variants which inherit enum's visibility
- trait declarations
- type alias (deprecated)
- constants
- statics
