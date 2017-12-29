fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
   where F: FnMut(&mut St, Self::Item) -> Option<B>;
/**
An iterator adaptor similar to fold that holds
internal state and produces a new iterator.

takes 2 arguments:
1. initial value which seeds the internal state
2. closure with 2 arguments:
   1) mutable reference to the internal state
   2) an iterator element

The closure can assign to the internal
state to share state between iterations.

On iteration, the closure will be applied to each element of the iterator
and the return value from the closure, an Option, is yielded by the iterator.

*/
let a = [1, 2, 3];

let mut iter = a.iter()
                .scan(1, |state, &x| {
                    // on each iteration, multiply state by element
                    *state = *state * x;
                    // the value passed on to the next iteration
                    Some(*state)
                });

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
