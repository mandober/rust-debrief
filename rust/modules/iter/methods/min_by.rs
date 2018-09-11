fn min_by<F>(self, compare: F) -> Option<Self::Item>
  where
    F: FnMut(&Self::Item, &Self::Item) -> Ordering,
1.15.0

Returns the element that gives the minimum value
with respect to the specified comparison function.

If several elements are equally minimum, the first element is returned.
If the iterator is empty, None is returned.

EXAMPLES:

let a = [-3_i32, 0, 1, 5, -10];
assert_eq!(*a.iter().min_by(|x, y| x.cmp(y)).unwrap(), -10);
