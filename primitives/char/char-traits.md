# Implemented traits on char


## Implemented traits
- Display
- Debug
- Default
- Hash
- From
- FromStr
- TryFrom
- Eq
- PartialEq
- Ord
- PartialOrd
- Pattern
- AsciiExt


## Display
Formats the value using the given formatter.

```rust
impl Display for char {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
}
```


## AsciiExt trait

```rust
impl AsciiExt for char {
  // Container type for copied ASCII characters.
  type Owned = char;

  // Checks if the value is within the ASCII range.
  fn is_ascii(&self) -> bool;

  fn to_ascii_uppercase(&self) -> Self::Owned;
  // Makes a copy of the value in its ASCII upper case equivalent.

  fn to_ascii_lowercase(&self) -> Self::Owned;
  // Makes a copy of the value in its ASCII lower case equivalent.

  fn eq_ignore_ascii_case(&self, o: &Self) -> bool;
  // Checks that two values are an ASCII case-insensitive match.

  fn make_ascii_uppercase(&mut self);
  // Converts this type to its ASCII upper case equivalent in-place.

  fn make_ascii_lowercase(&mut self);
  // Converts this type to its ASCII lower case equivalent in-place.


  // nightly-only experimental APIs (ascii_ctype #39658):
  // ===================================================

  fn is_ascii_alphabetic(&self) -> bool;
  // Checks if the value is an ASCII alphabetic character:
  // U+0041 'A' ... U+005A 'Z'
  // U+0061 'a' ... U+007A 'z'
  // For strings, true if all characters in the string are ASCII alphabetic.

  fn is_ascii_uppercase(&self) -> bool;
  // Checks if the value is an ASCII uppercase character: 
  // U+0041 'A' ... U+005A 'Z'.
  // For strings, true if all characters in the string are ASCII uppercase.

  fn is_ascii_lowercase(&self) -> bool;
  // Checks if the value is an ASCII lowercase character:
  // U+0061 'a' ... U+007A 'z'.
  // For strings, true if all characters in the string are ASCII lowercase.

  fn is_ascii_alphanumeric(&self) -> bool;
  // Checks if the value is an ASCII alphanumeric character:
  // U+0041 'A' ... U+005A 'Z'
  // U+0061 'a' ... U+007A 'z'
  // U+0030 '0' ... U+0039 '9'
  // For strings, true if all characters in the string are ASCII alphanumeric.

  fn is_ascii_digit(&self) -> bool;
  // Checks if the value is an ASCII decimal digit:
  // U+0030 '0' ... U+0039 '9'.
  // For strings, true if all characters in the string are ASCII digits.

  fn is_ascii_hexdigit(&self) -> bool;
  // Checks if the value is an ASCII hexadecimal digit:
  // U+0030 '0' ... U+0039 '9'
  // U+0041 'A' ... U+0046 'F'
  // U+0061 'a' ... U+0066 'f'
  // For strings, true if all characters in the string are ASCII hex digits.

  fn is_ascii_punctuation(&self) -> bool;
  // Checks if the value is an ASCII punctuation character:

  fn is_ascii_graphic(&self) -> bool;
  // Checks if the value is an ASCII graphic character:
  // U+0021 '!' ... U+007E '~'.
  // For strings, true if all chars in the string are ASCII graphic chars.

  fn is_ascii_whitespace(&self) -> bool;
  // Checks if the value is an ASCII whitespace character:
  // U+0020 SPACE
  // U+0009 HORIZONTAL TAB
  // U+000A LINE FEED
  // U+000C FORM FEED
  // U+000D CARRIAGE RETURN.
  // For strings, true if all characters in the string are ASCII whitespace.

  fn is_ascii_control(&self) -> bool;
  // Checks if the value is an ASCII control character:
  // U+0000 NUL ... U+001F UNIT SEPARATOR
  // U+007F DELETE
  // Note that most ASCII whitespace chars are control chars, except SPACE
}
```



```rust
From<u8>::from
FromStr::from_str
TryFrom<u32>::try_from
PartialEq<char>::{eq, ne}
Ord::{cpm, max, min}
PartialOrd<char>::{partial_cmp, lt, le, ge, gt}
Pattern<'a>::{into_searcher, is_contained_in, is_prefix_of, is_suffix_of}
```
