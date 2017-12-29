fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
   where P: FnMut(&Self::Item) -> bool;
/**
Creates an iterator that yields elements based on a predicate.

take_while takes a closure as an argument.
It will call this closure on each element of the
iterator, and yield elements while it returns true.

After false is returned, take_while's job is
over, and the rest of the elements are ignored.
*/

let a = [-1i32, 0, 1];

let mut iter = a.into_iter().take_while(|x| x.is_negative());

assert_eq!(iter.next(), Some(&-1));
assert_eq!(iter.next(), None);
/**
Because the closure passed to take_while() takes a reference, and many
iterators iterate over references, this leads to a possibly confusing
situation, where the type of the closure is a double reference:
*/
let a = [-1, 0, 1];

let mut iter = a.into_iter().take_while(|x| **x < 0); // need two *s!

assert_eq!(iter.next(), Some(&-1));
assert_eq!(iter.next(), None);

// Stopping after an initial false:

let a = [-1, 0, 1, -2];

let mut iter = a.into_iter().take_while(|x| **x < 0);

assert_eq!(iter.next(), Some(&-1));

// We have more elements that are less than zero, but since
// we already got a false, take_while isn't used any more
assert_eq!(iter.next(), None);


// Because take_while needs to look at the value in order to see if it
// should be included or not, consuming iterators'll see that it is removed:

let a = [1, 2, 3, 4];
let mut iter = a.into_iter();

let result: Vec<i32> = iter.by_ref()
                           .take_while(|n| **n != 3)
                           .cloned()
                           .collect();

assert_eq!(result, &[1, 2]);

let result: Vec<i32> = iter.cloned().collect();

assert_eq!(result, &[4]);

// 3 is no longer there, cuz it was consumed in order to see if the iteration
// should stop, but wasn't placed back into the iterator or some similar thing.
