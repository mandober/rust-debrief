fn escape_unicode(self) -> EscapeUnicode[src]
[−]

Returns an iterator that yields the hexadecimal Unicode escape of a character as chars.

This will escape characters with the Rust syntax of the form \u{NNNNNN} where NNNNNN is a hexadecimal representation.
Examples

As an iterator:

for c in '❤'.escape_unicode() {
    print!("{}", c);
}
println!();
Run

Using println! directly:

println!("{}", '❤'.escape_unicode());
Run

Both are equivalent to:

println!("\\u{{2764}}");
Run

Using to_string:

assert_eq!('❤'.escape_unicode().to_string(), "\\u{2764}");
Run

fn escape_debug(self) -> EscapeDebug1.20.0
[src]
[−]

Returns an iterator that yields the literal escape code of a character as chars.

This will escape the characters similar to the Debug implementations of str or char.
Examples

As an iterator:

for c in '\n'.escape_debug() {
    print!("{}", c);
}
println!();
Run

Using println! directly:

println!("{}", '\n'.escape_debug());
Run

Both are equivalent to:

println!("\\n");
Run

Using to_string:

assert_eq!('\n'.escape_debug().to_string(), "\\n");
Run

fn escape_default(self) -> EscapeDefault[src]
[−]

Returns an iterator that yields the literal escape code of a character as chars.

The default is chosen with a bias toward producing literals that are legal in a variety of languages, including C++11 and similar C-family languages. The exact rules are:

    Tab is escaped as \t.
    Carriage return is escaped as \r.
    Line feed is escaped as \n.
    Single quote is escaped as \'.
    Double quote is escaped as \".
    Backslash is escaped as \\.
    Any character in the 'printable ASCII' range 0x20 .. 0x7e inclusive is not escaped.
    All other characters are given hexadecimal Unicode escapes; see escape_unicode.

Examples

As an iterator:

for c in '"'.escape_default() {
    print!("{}", c);
}
println!();
Run

Using println! directly:

println!("{}", '"'.escape_default());
Run

Both are equivalent to:

println!("\\\"");
Run

Using to_string:

assert_eq!('"'.escape_default().to_string(), "\\\"");