// ! Module std::iter

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

/**
An iterator has a `next` method, which when called, returns an `Option<Item>`.
next will return Some(Item) as long as there are elements, and once they've all
been exhausted, will return None to indicate that iteration is finished.

Individual iterators may choose to resume iteration, and so calling next again
may or may not eventually start returning Some(Item) again at some point.

Iterator's full definition includes a number of other methods as well, but they
are default methods, built on top of next, and so you get them for free.

Iterators are composable, and it's common to chain them together to do more
complex forms of processing.


|> Forms of iteration
There are three common methods which can create iterators from a collection:
- iter()       which iterates over &T
- iter_mut()   which iterates over &mut T
- into_iter()  which iterates over T
Various things in the std may implement one or more of the three.


|>Implementing Iterator
Creating an iterator of your own involves two steps:
1. creating a struct to hold the iterator's state
2. implementing Iterator for that struct

This is why there are so many structs in this module:
there is one for each iterator and iterator adapter.

Let's make an iterator named Counter which counts from 1 to 5:
*/

// First, the struct:
struct Counter {
    count: usize,
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `next()`'s implementation below.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Then, we implement `Iterator` for our `Counter`:
impl Iterator for Counter {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<usize> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// And now we can use it!
let mut counter = Counter::new();

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);
// This will print 1 through 5, each on their own line.

/**
Calling next() like this way gets repetitive.
Rust has a construct which can call next() on your iterator,
until it reaches None: `for` loop.


|> `for` loop and IntoIterator
Rust's for loop syntax is actually sugar for iterators.
Here's a basic example of for:
*/
let values = vec![1, 2, 3, 4, 5];
for x in values {
    println!("{}", x);
}
/**
This will print the numbers one through five, each on their own line.

A trait for converting something into an iterator: `IntoIterator`
This trait has one method: `into_iter()`
which converts the thing implementing `IntoIterator` into an iterator

Compiler de-sugars a for loop into:
*/
let values = vec![1, 2, 3, 4, 5];
{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{}", x); };
        },
    };
    result
}
/**
First, we call `into_iter()` on the value.
Then, we match on the iterator that returns, calling next over and over until we
see a None. At that point, we break out of the loop, and we're done iterating.

There's one more subtle bit here:
the standard library contains an interesting implementation of IntoIterator:

impl<I: Iterator> IntoIterator for I

In other words, all Iterators implement IntoIterator, by just returning
themselves. This means two things:
- If you're writing an Iterator, you can use it with a for loop.
- If you're creating a collection, implementing IntoIterator for it will allow
  your collection to be used with the for loop.


|> Adapters
Functions which take an Iterator and return another Iterator are often
called 'iterator adapters', as they're a form of the 'adapter pattern'.
Common iterator adapters include: map, take, and filter.


|> Laziness
Iterators (and iterator adapters) are lazy.
This means that just creating an iterator doesn't do a whole lot.
Nothing really happens until you call next.
This is sometimes a source of confusion when creating an iterator solely for its
side effects.

For example, the map method calls a closure on each element it iterates over:
*/
let v = vec![1, 2, 3, 4, 5];
v.iter().map(|x| println!("{}", x));
/**
This will not print any values, as we only created an iterator, rather than using it.
The compiler will warn us about this kind of behavior:
  warning: unused result which must be used: iterator adaptors are lazy and
  do nothing unless consumed

The idiomatic way to write a map for its side effects is to use a for loop instead:
*/
let v = vec![1, 2, 3, 4, 5];

for x in &v {
    println!("{}", x);
}
/**
The 2 most common ways to evaluate an iterator:
1. use a for loop like this, or
2. use the collect method to produce a new collection


|> Infinity
Iterators do not have to be finite.

As an example, an open-ended range is an infinite iterator:
*/
let numbers = 0..;
/**
It is common to use the `take` iterator adapter
to turn an infinite iterator into a finite one:
*/
let numbers = 0..;
let five_numbers = numbers.take(5);
for number in five_numbers {
    println!("{}", number);
}
// This will print the numbers 0 through 4, each on their own line.
