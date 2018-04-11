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
