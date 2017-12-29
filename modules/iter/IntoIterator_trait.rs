// ! IntoIterator trait
/**
Trait std::iter::IntoIterator
1.0.0
https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
*/

pub trait IntoIterator where
    <Self::IntoIter as Iterator>::Item == Self::Item,
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

/**
|> Conversion into an Iterator

By implementing IntoIterator for a type, you define how it will be converted to
an iterator. This is common for types which describe a collection of some kind.

One benefit of implementing IntoIterator is that
your type will work with Rust's `for` loop syntax.

EXAMPLES:
*/
let v = vec![1, 2, 3];
let mut iter = v.into_iter();
assert_eq!(Some(1), iter.next());
assert_eq!(Some(2), iter.next());
assert_eq!(Some(3), iter.next());
assert_eq!(None, iter.next());

/**
|> Implementing IntoIterator for your type:

A sample collection, that's just a wrapper over Vec<T> */
#[derive(Debug)]
struct MyCollection(Vec<i32>);

/// Let's give it some methods so we can create one and add things to it
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

// and we'll implement IntoIterator
impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = ::std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Now we can make a new collection
let mut c = MyCollection::new();

// add some stuff to it
c.add(0);
c.add(1);
c.add(2);

// and then turn it into an Iterator:
for (i, n) in c.into_iter().enumerate() {
    assert_eq!(i as i32, n);
}

/**
|> Associated Types

type Item
The type of the elements being iterated over. */
type IntoIter: Iterator;
/** Which kind of iterator are we turning this into?

|> Required Methods
*/
fn into_iter(self) -> Self::IntoIter;

/**
Creates an iterator from a value.

See the module-level documentation for more.

|> Examples
*/
let v = vec![1, 2, 3];
let mut iter = v.into_iter();

assert_eq!(Some(1), iter.next());
assert_eq!(Some(2), iter.next());
assert_eq!(Some(3), iter.next());
