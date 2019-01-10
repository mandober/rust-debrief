fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
   where
    FromA: Default + Extend<A>,
    FromB: Default + Extend<B>,
    Self: Iterator<Item = (A, B)>;

/**
Converts an iterator of pairs into a pair of containers.

unzip consumes an entire iterator of pairs, producing 2 collections:
1. a collections from the left elements
2. a collections from the right elements

This function is, in some sense, the opposite of zip.
*/

let a = [(1, 2), (3, 4)];

let (left, right): (Vec<_>, Vec<_>) = a.iter().cloned().unzip();

assert_eq!(left, [1, 3]);
assert_eq!(right, [2, 4]);
