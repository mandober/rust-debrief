# iter module

- `std::iter` module, since: 1.0.0
- doc: https://doc.rust-lang.org/stable/std/iter/
- Contents:
  - Traits: define what kind of iterators exist and what you can do with them.
  - Functions: helpful ways to create basic iterators.
  - Structs: return types of trait's methods.



## Traits
define what kind of iterators exist and what can be done with them.

- `Iterator` interface for dealing with iterators.
- `IntoIterator` conversion into an Iterator.
- `FromIterator` conversion from an Iterator.
- `DoubleEndedIterator` iterator able to yield elements from both ends.
- `ExactSizeIterator` iterator that knows its exact length.
- `Extend` Extend a collection with the contents of an iterator.
- `Sum` represents types that can be created by summing up an iterator.
- `Product` represent types created by multiplying elements of an iterator.
- `FusedIterator` iterator that keeps yielding None once exhausted. _LAB_
- `Step` objects that can be stepped over in both directions. _LAB_
- `TrustedLen` iterator that reports an accurate length using size_hint. _LAB_


## Functions
provide some helpful ways to create some basic iterators.

- `empty`  Creates an iterator that yields nothing.
- `once`   Creates an iterator that yields an element exactly once.
- `repeat` Creates a new iterator that endlessly repeats a single element.


## Structs
They are often the return type of various methods on this module's traits. Inspect the method that creates the struct, rather than the struct itself.

- `Filter` filters the elements of iterator with predicate.
- `Map` maps the values of iterator with f.
- `FilterMap` uses f to both filter and map elements from iterator.
- `FlatMap` maps each element and yields the elements of produced iterators.
- `Chain` strings two iterators together.
- `Cloned` clones the elements of an underlying iterator.
- `Cycle` repeats endlessly.
- `Empty` yields nothing.
- `Enumerate` yields the current count and the element during iteration.
- `Fuse` yields None forever after the underlying iterator yields None once.
- `Inspect` calls fn with a reference to each element before yielding it.
- `Once` yields an element exactly once.
- `Peekable` has peek method that returns an optional ref to next element.
- `Repeat` repeats an element endlessly.
- `Rev` double-ended iterator with the direction inverted.
- `Scan` maintains state while iterating another iterator.
- `Skip` skips over n elements of iter.
- `SkipWhile` rejects elements while predicate is true.
- `Take` only iterates over the first n iterations of iter.
- `TakeWhile` only accepts elements while predicate is true.
- `Zip` iterates two other iterators simultaneously.
- `StepBy` An adapter for stepping iterators by a custom amount. _LAB_

