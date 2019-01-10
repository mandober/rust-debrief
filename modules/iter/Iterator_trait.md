# Iterator trait

- trait `std::iter::Iterator`
- online [doc](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- An interface for dealing with iterators. This is the main iterator trait.



The heart and soul of this module is the `Iterator` trait.

```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}
```


## Iteration methods:

- `iter` iterates over `&T`.
- `iter_mut` iterates over `&mut T`.
- `into_iter`iterates over `T`.


## Iterator

- Associated Types:   
  `type Item` - The type of the elements being iterated over.
- Required Methods:   
  `fn next(&mut self) -> Option<Self::Item>;`
- Provided Methods:   

```rust
pub trait Iterator {
    fn size_hint(&self) -> (usize, Option<usize>) { ... }

    fn count(self) -> usize { ... }

    fn last(self) -> Option<Self::Item> { ... }

    fn nth(&mut self, n: usize) -> Option<Self::Item> { ... }

    fn step_by(self, step: usize) -> StepBy<Self> { ... }

    fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>
    where
        U: IntoIterator<Item = Self::Item>,
    { ... }

    fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
    where
        U: IntoIterator,
    { ... }

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
    { ... }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item) -> (),
    { ... }

    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
    { ... }

    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        F: FnMut(Self::Item) -> Option<B>,
    { ... }

    fn enumerate(self) -> Enumerate<Self> { ... }

    fn peekable(self) -> Peekable<Self> { ... }

    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
    { ... }

    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
    { ... }

    fn skip(self, n: usize) -> Skip<Self> { ... }

    fn take(self, n: usize) -> Take<Self> { ... }

    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        F: FnMut(&mut St, Self::Item) -> Option<B>,
    { ... }

    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    { ... }

    fn fuse(self) -> Fuse<Self> { ... }

    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        F: FnMut(&Self::Item) -> (),
    { ... }

    fn by_ref(&mut self) -> &mut Self { ... }

    fn collect<B>(self) -> B
    where
        B: FromIterator<Self::Item>,
    { ... }

    fn partition<B, F>(self, f: F) -> (B, B)
    where
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool,
    { ... }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    { ... }

    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    { ... }

    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    { ... }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    { ... }

    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    { ... }

    fn rposition<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
        Self: ExactSizeIterator + DoubleEndedIterator,
    { ... }

    fn max(self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    { ... }

    fn min(self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    { ... }

    fn max_by_key<B, F>(self, f: F) -> Option<Self::Item>
    where
        B: Ord,
        F: FnMut(&Self::Item) -> B,
    { ... }

    fn max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    { ... }

    fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>
    where
        B: Ord,
        F: FnMut(&Self::Item) -> B,
    { ... }

    fn min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    { ... }

    fn rev(self) -> Rev<Self>
    where
        Self: DoubleEndedIterator,
    { ... }

    fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        Self: Iterator<Item = (A, B)>,
    { ... }

    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Iterator<Item = &'a T>,
        T: 'a + Clone,
    { ... }

    fn cycle(self) -> Cycle<Self>
    where
        Self: Clone,
    { ... }

    fn sum<S>(self) -> S
    where
        S: Sum<Self::Item>,
    { ... }

    fn product<P>(self) -> P
    where
        P: Product<Self::Item>,
    { ... }



    fn cmp<I>(self, other: I) -> Ordering
    where
        I: IntoIterator<Item = Self::Item>,
        Self::Item: Ord,
    { ... }

    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>,
    { ... }

    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<<I as IntoIterator>::Item>,
    { ... }

    fn ne<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<<I as IntoIterator>::Item>,
    { ... }

    fn lt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>,
    { ... }

    fn le<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>,
    { ... }

    fn gt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>,
    { ... }

    fn ge<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<<I as IntoIterator>::Item>,
    { ... }
}
```
