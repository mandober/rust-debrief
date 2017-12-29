fn max_by<F>(self, compare: F) -> Option<Self::Item>
   where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
/** 1.15.0

Returns the element that gives the maximum value
with respect to the specified comparison function.

If several elements are equally maximum, the last element is returned.

If the iterator is empty, None is returned.

*/
let a = [-3_i32, 0, 1, 5, -10];

assert_eq!(*a.iter()
             .max_by(|x, y| x.cmp(y))
             .unwrap(), 5);
