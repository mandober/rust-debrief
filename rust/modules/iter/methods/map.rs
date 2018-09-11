fn map<B, F>(self, f: F) -> Map<Self, F>
   where F: FnMut(Self::Item) -> B;
/**
Takes a closure and creates an iterator
which calls that closure on each element.

map() transforms one iterator into another, by means
of its argument: something that implements `FnMut`.

It produces a new iterator which calls this
closure on each element of the original iterator.

If you are good at thinking in types, you can think of map() like this:
If you have an iterator that gives you elements of some type A,
and you want an iterator of some other type B,
you can use map(), passing a closure that takes an A and returns a B.

map() is conceptually similar to a for loop.
However, as map() is lazy, it is best used when you're already working with
other iterators. If you're doing some sort of looping for a side effect,
it's considered more idiomatic to use for than map().
*/
let a = [1, 2, 3];
let mut iter = a.into_iter().map(|x| 2 * x);
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);

// If you're doing some sort of side effect, prefer `for` to map():
// don't do this:
(0..5).map(|x| println!("{}", x));
// it won't even execute, as it is lazy. Rust will warn you about this.
// Instead, use for:
for x in 0..5 {
    println!("{}", x);
}
