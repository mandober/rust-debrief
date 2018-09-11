fn last(self) -> Option<Self::Item>;
/**
Consumes the iterator, returning the last element.

This method will evaluate the iterator until it returns None.
While doing so, it keeps track of the current element.
After None is returned, last() will then return the last element it saw.
*/
let a = [1, 2, 3];
assert_eq!(a.iter().last(), Some(&3));

let a = [1, 2, 3, 4, 5];
assert_eq!(a.iter().last(), Some(&5));
