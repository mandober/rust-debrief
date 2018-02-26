# Primitive: character
https://doc.rust-lang.org/stable/std/primitive.char.html

Primitive Type `char` 1.0.0

A character type.

The char type represents a single character.
More specifically, since character isn't a well-defined concept in Unicode,
char is a UNICODE SCALAR VALUE, which is similar to, but not the same as,
a UNICODE CODE POINT.

This documentation describes a number of methods and trait implementations on
the char type. For technical reasons, there is additional, separate documentation
in the `std::char` module as well.

Representation:
char is always 4 bytes in size.
This is a different representation than a given
character would have as part of a `String`.

For example:
*/
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
