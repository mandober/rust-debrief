fn sum<S>(self) -> S
   where S: Sum<Self::Item>;
/**
1.11.0

Sums the elements of an iterator.

Takes each element, adds them together, and returns the result.

An empty iterator returns the zero value of the type.

Panics:
When calling sum() and a primitive integer type is being returned, this
method will panic if the computation overflows and debug assertions are enabled.
*/

let a = [1, 2, 3];
let sum: i32 = a.iter().sum();

assert_eq!(sum, 6);
