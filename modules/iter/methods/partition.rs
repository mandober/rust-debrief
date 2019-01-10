fn partition<B, F>(self, f: F) -> (B, B)
   where B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool;
/**
Consumes an iterator, creating two collections from it.

The predicate passed to partition() can return true, or false.

partition returns a pair, all of the elements for which it
returned true, and all of the elements for which it returned false.

*/
let a = [1, 2, 3];

let (even, odd): (Vec<i32>, Vec<i32>) = a.into_iter()
                                         .partition(|&n| n % 2 == 0);

assert_eq!(even, vec![2]);
assert_eq!(odd, vec![1, 3]);
