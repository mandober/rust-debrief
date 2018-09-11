fn take(self, n: usize) -> Take<Self>;

// Creates an iterator that yields its first n elements.


let a = [1, 2, 3];

let mut iter = a.iter().take(2);

assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);

// take is often used with an infinite iterator, to make it finite:

let mut iter = (0..).take(3);

assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);
