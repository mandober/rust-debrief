fn for_each<F>(self, f: F)
   where F: FnMut(Self::Item) -> ();
/**
since: Rust 1.21.0 (2017-10-12)

Calls a closure on each element of an iterator.

This is equivalent to using a for loop on the iterator, although break and
continue are not possible from a closure. It's generally more idiomatic to use
a for loop, but for_each may be more legible when processing items at the end of
longer iterator chains. In some cases for_each may also be faster than a loop,
because it will use internal iteration on adaptors like Chain.
*/
use std::sync::mpsc::channel;
let (tx, rx) = channel();
(0..5).map(|x| x * 2 + 1).for_each(move |x| tx.send(x).unwrap());
let v: Vec<_> = rx.iter().collect();
assert_eq!(v, vec![1, 3, 5, 7, 9]);

// For such a small example, a for loop may be cleaner, but for_each
// might be preferable to keep a functional style with longer iterators:
(0..5).flat_map(|x| x*100 .. x*110)
      .enumerate()
      .filter(|&(i, x)| (i + x) % 3 == 0)
      .for_each(|(i, x)| println!("{}:{}", i, x));
