# Character primitive

- `char` primitive is a Unicode Scalar Value
- it must be in the Unicode range `0-D7FF` and `E000-10FFFF`
- `char` is 4 bytes long, it can be cast to `u32` (not vice versa)
- array of chars is effectively an UCS-4/UTF-32 string
- a character is always in single quotes
- effectively`char` array is an UCS-4/UTF-32 string
- online docs: [`char` primitive][pchar], [`std::char` module][mchar]


| name                          | from |     to |
|-------------------------------|-----:|-------:|
| Unicode Code Points           |    0 | 10FFFF |
| ASCII                         |    0 |     7F |
| **Unicode Scalar Values** (1) |    0 |   D7FF |
| Surrogate Code Points         | D800 |   DFFF |
| **Unicode Scalar Values** (2) | E000 | 10FFFF |



[pchar]: https://doc.rust-lang.org/std/primitive.char.html
[mchar]: https://doc.rust-lang.org/std/char/


## Character

The `char` type represents a single character; more precisely, `char` is a Unicode Scalar Value, which is a subset of Unicode Code Points.

__Unicode code point__ (UCP) is any value in the Unicode codespace, `0-10FFFF`, but not all code points are assigned to encoded characters. __Unicode scalar values__ (USV) are Unicode code points in the range `0-D7FF` and `E000-10FFFF` inclusive. The hole in their range, `D800-DFFF`, represents __surrogate code points__, which are disallowed in `char`.

`char` is always 4 bytes in size and in the Unicode range `0-D7FF` and `E000-10FFFF`, while a `String` uses UTF-8, so it has variable size per character, from 1 to 4 bytes.

Sometimes it's impossible to tell whether a glyph is one or more Unicode scalar values. For example:
- `é` is 1 Unicode scalar value:
  - `U+00E9`: `LATIN SMALL LETTER E WITH ACUTE`, but
- `é` is 2 Unicode scalar values:
  - `U+0065`: `LATIN SMALL LETTER E`
  - `U+0301`: `COMBINING ACUTE ACCENT`

Since `char` can only hold a single Unicode scalar value, the former will fit inside `char`, while trying to fit the latter will raise an error:

```rust
let acute = 'é'; // ok
let combining = 'é'; // error
// error: character literal may only contain one codepoint: 'é'

// natuarally, it can always be given as a string slice:
let c = "é";
```

> When backspacing over a combining glyph, such as the one above, some text editors will only delete the second byte on the first key press, leaving a valid character: `é => e`

`char` is always 4 bytes in size. This is a different representation than a given character would have as part of a `String`, because `String` uses UTF-8, so it has variable size per character, from 1 to 4 bytes. Effectively, an array of characters is an UCS-4/UTF-32 string.

```rust
let v = vec!['h', 'e', 'l', 'l', 'o'];
// 5 elements times 4 bytes for each element is 20 bytes
let s = String::from("hello");
// while this string takes just 5 bytes
```

Due to this, processing characters can use a lot more memory then processing strings:

```rust
let s = String::from("❤️ me do");

let s_size = s.len() * std::mem::size_of::<u8>();
assert_eq!(12, s_size);

let v: Vec<char> = s.chars().collect();
assert_eq!(32, v.len() * std::mem::size_of::<char>());

```
