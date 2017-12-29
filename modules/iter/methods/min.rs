fn min(self) -> Option<Self::Item>
  where
    Self::Item: Ord

Returns the minimum element of an iterator.

If several elements are equally minimum, the first element is returned.
If the iterator is empty, None is returned.

EXAMPLES:

let a = [1, 2, 3];
let b: Vec<u32> = Vec::new();

assert_eq!(a.iter().min(), Some(&1));
assert_eq!(b.iter().min(), None);
