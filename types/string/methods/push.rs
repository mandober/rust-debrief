fn push(&mut self, ch: char)

Appends the given char to the end of this String.

EXAMPLES:

let mut s = String::from("abc");

s.push('1');
s.push('2');
s.push('3');

assert_eq!("abc123", s);