fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>
  where
    B: Ord,
    F: FnMut(&Self::Item) -> B,
1.6.0

Returns the element that gives the minimum value from the specified function.

If several elements are equally minimum, the first element is returned.
If the iterator is empty, None is returned.

EXAMPLES:

let a = [-3_i32, 0, 1, 5, -10];
assert_eq!(*a.iter().min_by_key(|x| x.abs()).unwrap(), 0);
