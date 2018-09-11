fn cycle(self) -> Cycle<Self>
   where Self: Clone;
/**
Repeats an iterator endlessly.

Instead of stopping at None, the iterator
will instead start again, from the beginning.

After iterating again, it will start at the
beginning again. And again. And again. Forever.

EXAMPLES:
*/
let a = [1, 2, 3];
let mut it = a.iter().cycle();

assert_eq!(it.next(), Some(&1));
assert_eq!(it.next(), Some(&2));
assert_eq!(it.next(), Some(&3));
assert_eq!(it.next(), Some(&1));
assert_eq!(it.next(), Some(&2));
assert_eq!(it.next(), Some(&3));
assert_eq!(it.next(), Some(&1));
