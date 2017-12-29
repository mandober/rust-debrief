fn enumerate(self) -> Enumerate<Self>;
/**
Creates an iterator which gives the current
iteration count as well as the next value.

The iterator returned yields pairs (i, val), where i is the current
index of iteration and val is the value returned by the iterator.

enumerate() keeps its count as a usize.

If you want to count by a different sized integer,
the zip function provides similar functionality.

Overflow Behavior:
The method does no guarding against overflows, so enumerating more
than usize::MAX elements either produces the wrong result or panics.
If debug assertions are enabled, a panic is guaranteed.

Panics:
The returned iterator might panic if the
to-be-returned index would overflow a usize.

EXAMPLES:
*/
let a = ['a', 'b', 'c'];

let mut iter = a.iter().enumerate();

assert_eq!(iter.next(), Some((0, &'a')));
assert_eq!(iter.next(), Some((1, &'b')));
assert_eq!(iter.next(), Some((2, &'c')));
assert_eq!(iter.next(), None);
