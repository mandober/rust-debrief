# Character

- primitive, UTF-8 encoded 4 bytes long Unicode Scalar Value,`char`


## Characters

A value of type `char` is a Unicode scalar value (non-surrogate code point),
represented as a 32-bit unsigned word in the
`0x0000 to 0xD7FF` or `0xE000 to 0x10FFFF` range.

An array of chars is effectively an `UCS-4/UTF-32` string.


## ID
name: character
type: `char`
sample: 'a', 'ß', '❤️', '❤', '💝'
sized: yes
size: 32b (4B)
storage: stack
primitive: yes
scalar: yes
module: yes
ownership: Copy


More specifically, since character isn't a well-defined concept in Unicode,
`char` is a unicode scalar value, which is similar to, but not the same as,
a unicode code point.

- char is always 4 bytes in size.
- char is a UNICODE SCALAR VALUE
- unicode scalar values: from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
- char is always 4 bytes in size (4b)
- create chars always in single quotes


```rust {.line-numbers}
let c = 'ß';
// type annotated
let hearts: char = '❤️';

if c.is_alphabetic() {
    println!("Alphabetical");
} else if c.is_numeric() {
    println!("Numerical");
} else {
    println!("Neither");
}
```


## std links
char primitive: https://doc.rust-lang.org/std/primitive.char.html
`core::char`: https://doc.rust-lang.org/core/char/
`std::char` : https://doc.rust-lang.org/std/char/
`std_unicode::char`: https://doc.rust-lang.org/std_unicode/char/


---

noun     | verb
---------|-------
licence  | license
practice | practise
advice   | advise



- 1 week's notice
- 1 year's work/3 years' work
- 5 days' credit[^1]


:smile:


> acknowledgement,
> not acknowledgment


Critic mark
Don't go around saying{-- to people that--} the world owes you a living. The world owes you nothing. It was here first. {~~One~>Only one~~} thing is impossible for God: To find {++any++} sense in any copyright law on the planet. {==Truth is stranger than fiction==}{>>strange but true<<}, but it is because Fiction is obliged to stick to possibilities; Truth isn’t.


Tasks
- [x] #refs, [links](), **formatting**
- [ ] and <kbd>tags</kbd> supported
- [x] list syntax required
- [x] this is a complete item <code>tags</code> 
- [ ] this is an incomplete item


LISTING
- Abbreviation: The HTML specification is maintained by the W3C.
- mark: ==marked text==
- Superscript: 30^th^
- Subscript: H~2~O



*[HTML]: Hyper Text Markup Language
*[W3C]:  World Wide Web Consortium

[^1]: Hi! This is a footnote