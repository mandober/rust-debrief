# iter module

- module `std::iter`, since: 1.0.0
- online [doc](https://doc.rust-lang.org/stable/std/iter/)
- Module contents:
  - Traits: define what kind of iterators exist and what you can do with them.
  - Functions: helpful ways to create basic iterators.
  - Structs: return types of trait's methods.



## Traits
Define what kind of iterators exist and what can be done with them.

- `Iterator` main trait
- `IntoIterator` conversion into an Iterator
- `FromIterator` conversion from an Iterator
- `DoubleEndedIterator` yields elements from both ends
- `ExactSizeIterator` knows its exact length
- `Extend` extend collection with iterator
- `Sum` sum elements of iterator
- `Product` multiplying elements of iterator
- `FusedIterator` yields None once exhausted. _LAB_
- `Step` stepped over in both directions. _LAB_
- `TrustedLen` reports accurate length using `size_hint`. _LAB_


## Functions
Provide some helpful ways to create some basic iterators.

- `empty`  iterator that yields nothing.
- `once`   iterator that yields an element once.
- `repeat` iterator that endlessly repeats a single element.


## Structs
Return types of various methods of traits.

- `Filter` filters the elements with predicate.
- `Map` maps the values with fn.
- `FilterMap` filter and map elements with fn.
- `FlatMap` maps each element and yields the elements of produced iterators.
- `Chain` strings two iterators together.
- `Cloned` clones the elements of an underlying iterator.
- `Cycle` repeats endlessly.
- `Empty` yields nothing.
- `Enumerate` current count along with element.
- `Fuse` once None is encountered, it yields None forever.
- `Inspect` calls fn with a ref to each element before yielding it.
- `Once` yields an element exactly once.
- `Peekable` returns an optional ref to next element.
- `Repeat` repeats an element endlessly.
- `Rev` double-ended with the direction inverted.
- `Scan` maintains state while iterating another iterator.
- `Skip` skips over elements.
- `SkipWhile` rejects elements while predicate is true.
- `Take` only iterates over the first n iterations of iter.
- `TakeWhile` only accepts elements while predicate is true.
- `Zip` iterates over 2 iterators simultaneously.
- `StepBy` adapter for stepping by a custom amount. _LAB_
