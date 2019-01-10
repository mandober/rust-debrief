fn inspect<F>(self, f: F) -> Inspect<Self, F>
   where F: FnMut(&Self::Item) -> ();
/**
Do something with each element of an iterator, passing the value on.

When using iterators, you'll often chain several of them together.
While working on such code, you might want to check out
what's happening at various parts in the pipeline.

To do that, insert a call to inspect().

It's much more common for inspect() to be used as a debugging
tool than to exist in your final code, but never say never.
*/
let a = [1, 4, 2, 3];

// this iterator sequence is complex.
let sum = a.iter()
           .cloned()
           .filter(|&x| x % 2 == 0)
           .fold(0, |sum, i| sum + i);

println!("{}", sum);

// let's add some inspect() calls to investigate what's happening
let sum = a.iter()
           .cloned()
           .inspect(|x| println!("about to filter: {}", x))
           .filter(|&x| x % 2 == 0)
           .inspect(|x| println!("made it through filter: {}", x))
           .fold(0, |sum, i| sum + i);

println!("{}", sum);

/**
This will print:

about to filter: 1
about to filter: 4
made it through filter: 4
about to filter: 2
made it through filter: 2
about to filter: 3
6
*/