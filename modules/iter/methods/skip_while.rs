fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
   where P: FnMut(&Self::Item) -> bool;
/**
Creates an iterator that skips elements based on a predicate.

skip_while takes a closure as an argument.
It will call this closure on each element of the
iterator, and ignore elements until it returns false.

After false is returned, skip_while's job is
over, and the rest of the elements are yielded.
*/

let a = [-1i32, 0, 1];
let mut iter = a.into_iter().skip_while(|x| x.is_negative());

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), None);

/**
Because the closure passed to skip_while takes a reference, and many
iterators iterate over references, this leads to a possibly confusing
situation, where the type of the closure is a double reference:
*/
let a = [-1, 0, 1];
let mut iter = a.into_iter().skip_while(|x| **x < 0); // need two *s!

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), None);

// Stopping after an initial false:

let a = [-1, 0, 1, -2];
let mut iter = a.into_iter().skip_while(|x| **x < 0);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&1));

// while this would have been false, since we already got a false,
// skip_while() isn't used any more
assert_eq!(iter.next(), Some(&-2));
assert_eq!(iter.next(), None);