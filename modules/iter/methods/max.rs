fn max(self) -> Option<Self::Item>
   where Self::Item: Ord;
/**
Returns the maximum element of an iterator.

If several elements are equally maximum, the last element is returned.

If the iterator is empty, None is returned.

*/
let a = [1, 2, 3];
assert_eq!(a.iter().max(), Some(&3));

let b: Vec<u32> = Vec::new();
assert_eq!(b.iter().max(), None);
