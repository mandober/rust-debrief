fn fold<B, F>(self, init: B, f: F) -> B
   where F: FnMut(B, Self::Item) -> B;
/**
An iterator adaptor that applies a function, producing a single, final value.

fold takes 2 arguments:
1. an initial value
2. a closure with 2 arguments:
   1) an accumulator
   2) an element

The closure returns the value that the
accumulator should have for the next iteration.

The initial value is the value the
accumulator will have on the first call.

After applying this closure to every element of
the iterator, fold() returns the accumulator.

This operation is sometimes called 'reduce' or 'inject'.

Folding is useful whenever you have a collection of
something, and want to produce a single value from it.
*/
let a = [1, 2, 3];
// the sum of all of the elements of a
let sum = a.iter().fold(0, |acc, &x| acc + x);
assert_eq!(sum, 6);

/**
Let's walk through each step of the iteration here:

el  acc x   result
    0
1:  0   1   1
2:  1   2   3
3:  3   3   6

And so, our final result, 6.

It's common for people who haven't used iterators a lot to use a for loop with
a list of things to build up a result. Those can be turned into fold()s:
*/
let numbers = [1, 2, 3, 4, 5];

let mut result = 0;
for i in &numbers {
    result = result + i;
}

let result2 = numbers.iter().fold(0, |acc, &x| acc + x);

// they're the same
assert_eq!(result, result2);
