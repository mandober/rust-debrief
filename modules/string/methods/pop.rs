fn pop(&mut self) -> Option<char>

Removes the last character from the string buffer and returns it.

Returns None if this String is empty.

EXAMPLES:

let mut s = String::from("foo");

assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('f'));

assert_eq!(s.pop(), None);
