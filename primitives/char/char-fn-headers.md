# Method signatures

```rust
fn is_alphabetic(self) -> bool;
fn is_uppercase(self) -> bool;
fn is_lowercase(self) -> bool;
fn is_alphanumeric(self) -> bool;
fn is_numeric(self) -> bool;
fn is_digit(self, radix: u32) -> bool;
fn is_whitespace(self) -> bool;
fn is_control(self) -> bool;
fn is_xid_start(self) -> bool; // [LAB]
fn is_xid_continue(self) -> bool; // [LAB]
fn encode_utf8(self, dst: &mut [u8]) -> &mut str;
fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];
fn escape_unicode(self) -> EscapeUnicode;
fn len_utf8(self) -> usize;
fn to_lowercase(self) -> ToLowercase; // Returns an iterator
fn to_uppercase(self) -> ToUppercase; // Returns an iterator
fn to_digit(self, radix: u32) -> Option<u32>;
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
