fn collect<B>(self) -> B
   where B: FromIterator<Self::Item>;
/**
Transforms an iterator into a collection.

collect() can take anything iterable, and turn it into a relevant collection.

This is one of the more powerful methods in the
standard library, used in a variety of contexts.

The most basic pattern in which collect() is
used is to turn one collection into another.
You take a collection, call `iter` on it, do a bunch
of transformations, and then collect() at the end.

One of the keys to collect's power is that many things
you might not think of as 'collections' actually are.
For example, a String is a collection of chars.
And a collection of Result<T, E> can be
thought of as single Result<Collection<T>, E>.

Because collect() is so general, it can cause problems with type inference.
As such, collect() is one of the few times you'll see the syntax affectionately
known as the 'turbofish': ::<>. This helps the inference algorithm understand
specifically which collection you're trying to collect into.

EXAMPLES:
*/
let a = [1, 2, 3];
let doubled: Vec<i32> = a.iter()
                         .map(|&x| x * 2)
                         .collect();
assert_eq!(vec![2, 4, 6], doubled);
/**
Note that we needed the Vec<i32> on the left-hand side.
This is because we could collect into, for example, a VecDeque<T> instead:
*/
use std::collections::VecDeque;
let a = [1, 2, 3];
let doubled: VecDeque<i32> = a.iter()
                              .map(|&x| x * 2)
                              .collect();
assert_eq!(2, doubled[0]);
assert_eq!(4, doubled[1]);
assert_eq!(6, doubled[2]);

// Using the 'turbofish' instead of annotating doubled:
let a = [1, 2, 3];
let doubled = a.iter()
               .map(|&x| x * 2)
               .collect::<Vec<i32>>();
assert_eq!(vec![2, 4, 6], doubled);
/**
Because collect() only cares about what you're collecting into,
you can still use a partial type hint, _, with the turbofish:
*/
let a = [1, 2, 3];
let doubled = a.iter()
               .map(|&x| x * 2)
               .collect::<Vec<_>>();
assert_eq!(vec![2, 4, 6], doubled);


// Using collect() to make a String:
let chars = ['g', 'd', 'k', 'k', 'n'];
let hello: String = chars.iter()
                         .map(|&x| x as u8)
                         .map(|x| (x + 1) as char)
                         .collect();
assert_eq!("hello", hello);

/**
If you have a list of Result<T, E>s, you can
use collect() to see if any of them failed:
*/
let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
// gives us the first error
assert_eq!(Err("nope"), result);


let results = [Ok(1), Ok(3)];
let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
// gives us the list of answers
assert_eq!(Ok(vec![1, 3]), result);
