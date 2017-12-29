fn step_by(self, step: usize) -> StepBy<Self>;
/**
[LAB] This is a nightly-only experimental API.
(iterator_step_by #27741)

Creates an iterator starting at the same point,
but stepping by the given amount at each iteration.

Note that it will always return the first element
of the iterator, regardless of the step given.

Panics
The method will panic if the given step is 0.
*/

#![feature(iterator_step_by)]
let a = [0, 1, 2, 3, 4, 5];
let mut iter = a.into_iter().step_by(2);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), None);
