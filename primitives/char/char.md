# Characters

- primitive, UTF-8 encoded 4 bytes long Unicode Scalar Value (USV),`char`
- range: 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF
- always in single quotes
- online docs: [primitive][pchar], [module][mchar]



[pchar]: https://doc.rust-lang.org/std/primitive.char.html
[mchar]: https://doc.rust-lang.org/std/char/



## Character

A value of type `char` is a Unicode Scalar Value (non-surrogate code point), which is similar to, but not the same as, a Unicode Code Point. It is represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF range. It always appears in single quotes.

Character is always 4 bytes in size, while the owned UTF-8 string type has variable size per character (1-4 bytes).

An array of characters is effectively an UCS-4/UTF-32 string. 


## ID
colloquial: character
type: `char`
sample: 'a', 'ß', '❤️'
sized: yes
size: 32b (4B)
storage: stack
module: `std::char`
notes: nominal, primitive, scalar, Copy



## Example

```rust
let v = vec!['h', 'e', 'l', 'l', 'o'];

// five elements times four bytes for each element
assert_eq!(20, v.len() * std::mem::size_of::<char>());

let s = String::from("hello");


// five elements times one byte per element
assert_eq!(5, s.len() * std::mem::size_of::<u8>());

/**
As always, remember that a human intuition for character may not map to
Unicode's definitions. For example, despite looking similar, the 'é' character
is one Unicode code point while 'é' is two Unicode code points:
*/

// char as 1 unicode point: é as \u{00e9}
let mut chars = "é".chars();
// U+00e9: latin small letter e with acute
assert_eq!(Some('\u{00e9}'), chars.next());
assert_eq!(None, chars.next());

// char é as 2 unicode points: é as \u{0065} \u{0301}
let mut chars = "é".chars();
// U+0065: latin small letter e
assert_eq!(Some('\u{0065}'), chars.next());
// U+0301: combining acute accent
assert_eq!(Some('\u{0301}'), chars.next());
assert_eq!(None, chars.next());

/**
This means that the contents of the first string above will fit into a char
while the contents of the second string will not. Trying to create a char
literal with the contents of the second string gives an error:
    error: character literal may only contain one codepoint: é

Another implication of the 4-byte fixed size of a char is
that per-char processing can end up using a lot more memory:
*/
let s = String::from("love: ❤️");
let v: Vec<char> = s.chars().collect();

assert_eq!(12, s.len() * std::mem::size_of::<u8>());
assert_eq!(32, v.len() * std::mem::size_of::<char>());
```


## Methods

```rust
is_alphabetic
is_numeric
is_whitespace
is_lowercase
```
