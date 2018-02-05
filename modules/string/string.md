# String

String is a kind of smart pointers. It owns some memory and allows manipulation, 
and has metadata (like capacity) and extra capabilities or guarantees (String 
data will always be valid UTF-8). Like all smart pointer it implements the `Deref` 
and `Drop` traits.

- String is `Clone`, but not `Copy`
- String is a heap-allocated, owning, type.
- data stored on heap
- `String`/`&str` are owned/borrowed variants of string.


String is `Clone`
The implementation of `Clone` for `String` needs to copy the pointed-to string 
buffer in the heap. A simple bitwise copy of `String` values would merely copy 
the pointer, leading to a double free down the line. 
For this reason, `String` is `Clone` but not `Copy`.




```rust
// create it from string literals
let s = String::from("literal");
let s = "literal".to_string();
let s = "literal".to_owned();
let s = "literal".into();

// new empty string construction:
let s = String::new();
// push char into it
s.push('j');
// push string literal into it:
s.push_str("sbx");
```



raw strings
just like regular strings except they are prefixed with an `r` and do not process 
any escape sequences. For example, "\\d" is the same expression as r"\d".

