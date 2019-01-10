// ! filter

fn filter<P>(self, predicate: P) -> Filter<Self, P>
   where P: FnMut(&Self::Item) -> bool;
/**
- Creates an iterator which uses a closure to determine
  if an element should be yielded.
  The closure must return true or false.

- filter() creates an iterator which calls this closure on each element.
  If the closure returns true, then the element is returned.

- If the closure returns false, it will try again,
  and call the closure on the next element, seeing if it passes the test.

*/
let a = [0i32, 1, 2];
let mut iter = a.into_iter().filter(|x| x.is_positive());
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);

/**
Because the closure passed to filter() takes a reference,
and many iterators iterate over references,
this leads to a possibly confusing situation,
where the type of the closure is a double reference:
*/
let a = [0, 1, 2];
let mut iter = a.into_iter().filter(|x| **x > 1); // need two *s!
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);

// It's common to instead use destructuring on the argument to strip away one:
let a = [0, 1, 2];
let mut iter = a.into_iter().filter(|&x| *x > 1); // both & and *
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);

// or both:
let a = [0, 1, 2];
let mut iter = a.into_iter().filter(|&&x| x > 1); // two &s
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);
// of these layers.
