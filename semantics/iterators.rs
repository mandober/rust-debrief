/*
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

every impl of Iterator must specify both next method, which defines how we
iterate, as well as the type Item, which defines what kind of values this
iterator produces. The two items are linked, since the return value of next()
is Self::Item.

The notation `Self::Item` means "the Item defined in the impl for the type Self"
in other words, the Item type defined for this iterator. This is actually short
for more explicit notation: `<Self as Iterator>::Item`. Here we are saying
"the Item type defined in the implementation of Iterator for the type Self"

Now we can use the iterator trait to write generic code:
*/

pub fn position<I>(mut iterator: I, value: I::Item) -> Option<usize>
    where I: Iterator, I::Item: PartialEq,
{
    let mut index = 0;
    while let Some(v) = iterator.next() {
        if value == v {
            println! ("found it at position: {}", index);
            return Some(index); // found it!
        }
        index += 1;
    }
    println! ("not found");
    None // did not find it
}

/*
The first argument, `iterator` is of type I, which is a generic type parameter;
the where clause also declares that `I: Iterator`. So basically we just know
that iterator's type is some kind of iterator. The second param has the type
`I::Item`, also a kind of generic type. We're saying that value is "whatever
kind of item I produces".

We can also write this slightly different, using two generic parameters:
*/

pub fn pos<I, T>(mut iterator: I, value: T) -> Option<usize>
    where I: Iterator<Item=T>, T: PartialEq
{
    let mut index = 0;
    while let Some(v) = iterator.next() {
        if value == v {
            println! ("found it at position: {}", index);
            return Some(index); // found it!
        }
        index += 1;
    }
    println! ("not found");
    None // did not find it
}

/*
Here the where clause states that `I: Iterator<Item=T>`.
This means "I is some sort of iterator producing values of type T".
*/
