# Methods on char

<!-- TOC -->

- [Inherent methods](#inherent-methods)
- [Methods from traits](#methods-from-traits)

<!-- /TOC -->


## Inherent methods

```rust
encode_utf16
encode_utf8
eq_ignore_ascii_case
escape_debug
escape_default
escape_unicode
is_alphabetic
is_alphanumeric
is_ascii
is_ascii_alphabetic
is_ascii_alphanumeric
is_ascii_control
is_ascii_digit
is_ascii_graphic
is_ascii_hexdigit
is_ascii_lowercase
is_ascii_punctuation
is_ascii_uppercase
is_ascii_whitespace
is_control
is_digit
is_lowercase
is_numeric
is_uppercase
is_whitespace
is_xid_continue // nightly
is_xid_start // nightly
len_utf16
len_utf8
make_ascii_lowercase
make_ascii_uppercase
to_ascii_lowercase
to_ascii_uppercase
to_digit
to_lowercase
to_uppercase
```

## Methods from traits

```rust
try_from // TryFrom<u32>
from_str // FromStr
from // From<u8>
eq // PartialEq<char>
ne // PartialEq<char>
cpm // Ord
max // Ord
min // Ord
partial_cmp // PartialOrd<char>
lt // PartialOrd<char>
le // PartialOrd<char>
ge // PartialOrd<char>
gt // PartialOrd<char>
into_searcher // Pattern<'a> (nightly)
is_contained_in // Pattern<'a> (nightly)
is_prefix_of // Pattern<'a> (nightly)
is_suffix_of // Pattern<'a> (nightly)
is_ascii // AsciiExt
is_ascii_alphabetic // AsciiExt (nightly)
is_ascii_alphanumeric // AsciiExt (nightly)
is_ascii_control // AsciiExt (nightly)
is_ascii_digit // AsciiExt (nightly)
is_ascii_graphic // AsciiExt (nightly)
is_ascii_hexdigit // AsciiExt (nightly)
is_ascii_lowercase // AsciiExt (nightly)
is_ascii_punctuation // AsciiExt (nightly)
is_ascii_uppercase // AsciiExt (nightly)
is_ascii_whitespace // AsciiExt (nightly)
make_ascii_lowercase // AsciiExt
make_ascii_uppercase // AsciiExt
to_ascii_lowercase // AsciiExt
to_ascii_uppercase // AsciiExt
eq_ignore_ascii_case // AsciiExt
```
