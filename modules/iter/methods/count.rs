fn count(self) -> usize;
/**
Consumes the iterator, counting the number of iterations and returning it.

This method will evaluate the iterator until its next returns None.
Once None is encountered, count() returns the number of times it called next.

Overflow Behavior:
The method does no guarding against overflows, so counting elements of an
iterator with more than `usize::MAX` elements either produces the wrong result
or panics. If debug assertions are enabled, a panic is guaranteed.

PANICS:
This function might panic if the iterator has more than `usize::MAX elements`.

EXAMPLES:
*/
let a = [1, 2, 3];
assert_eq!(a.iter().count(), 3);

let a = [1, 2, 3, 4, 5];
assert_eq!(a.iter().count(), 5);
