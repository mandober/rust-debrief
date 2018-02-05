# Literal

* A literal is a compile-time evaluated expression that immediately and directly denotes the value it evaluates to.
* Character literals
  - supported characters: all unicode
  - supported escapes: Whitespace, Null, Backslash, Quote, ASCII, Unicode
* String literals
  - supported characters: all unicode
  - supported escapes: Whitespace, Null, Backslash, Quote, ASCII, Unicode
  - special escape: line continuation
* Raw string literals



<!-- TOC -->

- [Literals](#literals)
- [Escapes](#escapes)
- [Character literals](#character-literals)
- [String literals](#string-literals)
- [Raw string literals](#raw-string-literals)
- [Byte literals](#byte-literals)

<!-- /TOC -->



## Literals
A literal is an expression consisting of a single token, rather than a sequence of tokens, that immediately and directly denotes the value it evaluates to, rather than referring to it by name or some other evaluation rule. A literal is a form of constant expression, so is evaluated (primarily) at compile time.

It directly describes a number, character, string, or boolean value.


## Escapes

* **Whitespace escape**
  - `\n` newline
  - `\r` carriage return
  - `\t` tab
* **Null escape**
  - `\0` null
* **Backslash escape**
  - `\\` backslash
* **Quote escapes**
  - `\'` single quote
  - `\"` double quote
* **ASCII escapes**
  - `\x41` 7-bit char code, exactly 2 hex digits, range: `\x00-\x7f`
* **Byte escapes**
  - `\x7F` 8-bit character code, exactly 2 hex digits
* **Unicode escapes**
  - `\u{7FFF}` 24-bit Unicode character code, up to 6 hex digits


## Character literals
- a single unicode character delimited with single-quotes.
- single quote must be escaped with a backslash to represent itself
- supported characters: all unicode
- supported escapes: Whitespace, Null, Backslash, Quote, ASCII, Unicode
- examples:

```rust
'❤'       // single character only
'\t'       // <tab>
'\x0b'     // ASCII escape: <vtab>
'\''       // quote escape: '
'\u{02a4}' // Unicode escape: ʤ
```

## String literals
- a sequence of Unicode characters delimited with double-quotes
- double-quote must be escaped with a backslash to represent itself
- line-break characters are allowed
  - they represent themselves, except for
  - line continuation: when a backslash occurs immediately before the newline, then the line is continued; the backslash, the newline, and all whitespace at the beginning of the next line is ignored.
- supported characters: all unicode
- supported escapes: Whitespace, Null, Backslash, Quote, ASCII, Unicode
- examples:

```rust
"\"Rustafari\""
"\x22quote\x22"
"col1\tcol2\tcol3\tcol4"
"same \
    line"
```


## Raw string literals

- raw string literals do not process any escapes.
- they are introduced with `r` letter and delimited with zero or more `#` characters and double-quotes; `r(#*)".*?"\1`
- supported characters: all unicode
- supported escapes: none
- examples of string literals vs raw string literals:

```rust
"\"Rustafari\""     r#""Rustafari""#     // "Rustafari"
"\\x52"             r"\x52"              // \x52
"r#\"___\"#"        r##"r#"___"#"##      // r#"___"#
"r(#*)\".*?\"\\1"   r##"r(#*)".*?"\1"##  // r(#*)".*?"\1
```


## Byte literals

