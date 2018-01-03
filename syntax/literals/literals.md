# Literals

<!-- TOC -->

- [Escapes](#escapes)
  - [ASCII escapes](#ascii-escapes)
  - [Byte escapes](#byte-escapes)
  - [Unicode escapes](#unicode-escapes)
  - [Quote escapes](#quote-escapes)
- [Boolean literals](#boolean-literals)
- [Symbols](#symbols)
- [Number literals](#number-literals)
- [Number suffixes](#number-suffixes)
- [Character literals](#character-literals)
- [String literals](#string-literals)
- [Character escapes](#character-escapes)
- [Raw string literals](#raw-string-literals)
- [Byte literals](#byte-literals)
- [Byte string literals](#byte-string-literals)
- [Raw byte string literals](#raw-byte-string-literals)
- [Number literals](#number-literals-1)
- [Integer literals](#integer-literals)
- [Floating-point literals](#floating-point-literals)

<!-- /TOC -->



**Characters and strings**

name           |example    |# sets|characters  |escapes
---------------|-----------|------|------------|------------------
char           |'H'        |      |All Unicode |Quote, Byte, Unicode
string literal |"hello"    |      |All Unicode |Quote, Byte, Unicode
raw string     |r#"hello"# |0...  |All Unicode |
---------------|-----------|------|------------|------------------
byte           |b'H'       |      |All ASCII   |Quote, Byte
byte string    |b"hello"   |      |All ASCII   |Quote, Byte
raw byte string|br#"hello"#|0...  |All ASCII   |




## Escapes

### ASCII escapes

```
\n     newline
\r     carriage return
\t     tab
\\     backslash
\0     null
\x41   7-bit character code (exactly 2 digits, up to 0x7F)
```

### Byte escapes

```
\n     newline
\r     carriage return
\t     tab
\\     backslash
\0     null
\x7F   8-bit character code (exactly 2 digits)
```

### Unicode escapes

```
\u{7FFF}   24-bit Unicode character code (up to 6 digits)
```

### Quote escapes

```
\'    Single quote; backslash: U+005C, single quote: U+0027
\"    Double quote: 
```



## Boolean literals
The two values of the boolean type are written as `true` and `false` literals.


## Symbols
Symbols are a general class of printable tokens that play structural roles in a variety of grammar productions. They are a set of remaining miscellaneous printable tokens that do not otherwise appear as literals, operators or keywords.



## Number literals

Number literals  | Example    | Suffixes  | Exponentiation
-----------------|------------|-----------|---------------
Decimal integer  |98_222      |Int suff.  |
Hex integer      |0xff        |Int suff.  | `0x`
Octal integer    |0o77        |Intsuff.   | `0o`
Binary integer   |0b1111_0000 |Int suff.  | `0b`
Floating-point   |123.0E+77   |Float suff.| optional

All number literals allow `_` as a visual separator: 1_234.0E+18f64


## Number suffixes

Floats: f32, f64
Integer: u8, i8, u16, i16, u32, i32, u64, i64, isize, usize

e.g.: let len = 2020_usize;


## Character literals
A character literal is a single Unicode character enclosed within two U+0027 
(single-quote) characters, with the exception of U+0027 itself, which must be 
escaped by a preceding U+005C character (\). 


## String literals
A string literal is a sequence of any Unicode characters enclosed within two 
U+0022 (double-quote) characters, with the exception of U+0022 itself, which 
must be escaped by a preceding U+005C character (\).

Line-break characters are allowed in string literals. Normally they represent 
themselves (i.e. no translation), but as a special exception, when an unescaped 
U+005C character (\) occurs immediately before the newline (U+000A), the U+005C 
character, the newline, and all whitespace at the beginning of the next line are 
ignored.

```rust
let a = "foobar";
let b = "foo\
         bar";

assert_eq!(a,b);
```

## Character escapes
Some additional escapes are available in either character or non-raw string 
literals. An escape starts with a U+005C (\) and continues with one of the 
following forms:
- An 8-bit code point escape starts with U+0078 (x) and is followed by exactly 
  two hex digits. It denotes the Unicode code point equal to the provided hex value.
- A 24-bit code point escape starts with U+0075 (u) and is followed by up to six
  hex digits surrounded by braces U+007B ({) and U+007D (}). It denotes the 
  Unicode code point equal to the provided hex value.
- A whitespace escape is one of the characters U+006E (n), U+0072 (r), or 
  U+0074 (t), denoting the Unicode values U+000A (LF), U+000D (CR) or 
  U+0009 (HT) respectively.
- The null escape is the character U+0030 (0) and denotes the Unicode value 
  U+0000 (NUL).
- The backslash escape is the character U+005C (\) which must be escaped in 
  order to denote itself.


## Raw string literals
Raw string literals do not process any escapes.
They start with the character U+0072 (r), followed by zero or more of the char
U+0023 (#) and a U+0022 (double-quote) character. The raw string body can contain
any sequence of Unicode characters and is terminated only by another U+0022 
(double-quote) character, followed by the same number of U+0023 (#) characters 
that preceded the opening U+0022 (double-quote) character.

All Unicode characters contained in the raw string body represent themselves, 
the characters U+0022 (double-quote) (except when followed by at least as many 
U+0023 (#) characters as were used to start the raw string literal) or 
U+005C (\) do not have any special meaning.

Examples for string literals:

```rust
"foo"; r"foo";                     // foo
"\"foo\""; r#""foo""#;             // "foo"

"foo #\"# bar";
r##"foo #"# bar"##;                // foo #"# bar

"\x52"; "R"; r"R";                 // R
"\\x52"; r"\x52";                  // \x52
```


## Byte literals
A byte literal is:
- a single ASCII character (in the U+0000 to U+007F range) or 
- a single escape
preceded by the characters U+0062 (b) and U+0027 (single-quote), and 
followed by the character U+0027 (single-quote).

* If the character U+0027 (single-quote) is present within the
  literal, it must be escaped by a preceding U+005C (\) character.

It is equivalent to a u8 literal.

```rust
let byte_literal = b'w';

```


## Byte string literals
A non-raw byte string literal is:
- a sequence of ASCII characters and escapes
preceded by the characters U+0062 (b) and U+0022 (double-quote), and 
followed by the character U+0022. 

* If the character U+0022 is present within the literal,
  it must be escaped by a preceding U+005C (\) character.

Alternatively, a byte string literal can be a *raw byte string literal*.

A byte string literal of length `n` is equivalent to a `&'static [u8; n]`
borrowed fixed-sized array of unsigned 8-bit integers.

Additional escapes are available in either byte or non-raw byte string literals.

An escape starts with a U+005C (\) and continues with one of the following forms:
- *byte escape* starts with U+0078 (x) and is followed by exactly 
  2 hex digits. It denotes the byte equal to the provided hex value.

- *whitespace escape* is one of the characters
  U+006E (n), U+0072 (r), or U+0074 (t), 
  denoting the bytes values 
  0x0A (ASCII LF), 0x0D (ASCII CR) or 0x09 (ASCII HT) respectively.

- *null escape* is the character U+0030 (0) 
  and denotes the byte value 0x00 (ASCII NUL).

- *backslash escape* is the character U+005C (\) which must
  be escaped in order to denote its ASCII encoding 0x5C.



## Raw byte string literals
Raw byte string literals do not process any escapes.
They start with the characters U+0062 (b) and U+0072 (r),
    followed by zero or more of the opening U+0023 (#) characters,
      followed by opening U+0022 (double-quote) character. 
        *The raw string body can contain any sequence of ASCII chars*.
      followed by closing U+0022 (double-quote) character,
    followed by the same number of closing U+0023 (#) characters.

```rust
// if no special chars, no need for hash-fence
let raw_byte_str_lit1 = br"\nosp\e\c";
let raw_byte_str_lit2 = br###"ASCIIs, pound:# "###;

```

A raw byte string literal cannot contain any non-ASCII byte.

All characters contained in the raw string body represent their ASCII encoding, 
the characters U+0022 (double-quote) (except when followed by at least as many 
U+0023 (#) characters as were used to start the raw string literal) or 
U+005C (\) do not have any special meaning.

Examples for byte string literals:

```rust
b"foo";
br"foo";                     // foo

b"\"foo\"";
br#""foo""#;             // "foo"

b"foo #\"# bar";
br##"foo #"# bar"##;                 // foo #"# bar

b"\x52";
b"R";
br"R";                // R

b"\\x52";
br"\x52";                  // \x52
```



## Number literals

A number literal is either an integer literal or a floating-point literal. The grammar for recognizing the two kinds of literals is mixed.


## Integer literals

An integer literal has one of four forms:

    A decimal literal starts with a decimal digit and continues with any mixture of decimal digits and underscores.
    A hex literal starts with the character sequence U+0030 U+0078 (0x) and continues as any mixture of hex digits and underscores.
    An octal literal starts with the character sequence U+0030 U+006F (0o) and continues as any mixture of octal digits and underscores.
    A binary literal starts with the character sequence U+0030 U+0062 (0b) and continues as any mixture of binary digits and underscores.

Like any literal, an integer literal may be followed (immediately, without any spaces) by an integer suffix, which forcibly sets the type of the literal. The integer suffix must be the name of one of the integral types: u8, i8, u16, i16, u32, i32, u64, i64, isize, or usize.

The type of an unsuffixed integer literal is determined by type inference:

    If an integer type can be uniquely determined from the surrounding program context, the unsuffixed integer literal has that type.

    If the program context under-constrains the type, it defaults to the signed 32-bit integer i32.

    If the program context over-constrains the type, it is considered a static type error.

Examples of integer literals of various forms:

123i32;                            // type i32
123u32;                            // type u32
123_u32;                           // type u32
0xff_u8;                           // type u8
0o70_i16;                          // type i16
0b1111_1111_1001_0000_i32;         // type i32
0usize;                            // type usize
Run

Note that the Rust syntax considers -1i8 as an application of the unary minus operator to an integer literal 1i8, rather than a single integer literal.

## Floating-point literals

A floating-point literal has one of two forms:

    A decimal literal followed by a period character U+002E (.). This is optionally followed by another decimal literal, with an optional exponent.
    A single decimal literal followed by an exponent.

Like integer literals, a floating-point literal may be followed by a suffix, so long as the pre-suffix part does not end with U+002E (.). The suffix forcibly sets the type of the literal. There are two valid floating-point suffixes, f32 and f64 (the 32-bit and 64-bit floating point types), which explicitly determine the type of the literal.

The type of an unsuffixed floating-point literal is determined by type inference:

    If a floating-point type can be uniquely determined from the surrounding program context, the unsuffixed floating-point literal has that type.

    If the program context under-constrains the type, it defaults to f64.

    If the program context over-constrains the type, it is considered a static type error.

Examples of floating-point literals of various forms:

123.0f64;        // type f64
0.1f64;          // type f64
0.1f32;          // type f32
12E+99_f64;      // type f64
let x: f64 = 2.; // type f64


This last example is different because it is not possible to use the suffix syntax with a floating point literal ending in a period. 2.f64 would attempt to call a method named f64 on 2.

The representation semantics of floating-point numbers are described in "Machine Types".

