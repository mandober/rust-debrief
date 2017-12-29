# Iter methods

## Index
```
all
any
by_ref
chain
cloned
collect
count
cycle
enumerate
filter
find
flat_map
fold
for_each
fuse
ge, gt, le, lt, ne, eq, cmp, partial_cmp
inspect
last
map
max_by_keys
max_by
max
min_by_keys
min_by
min
next
nth
partition
peekable
position
product
rev
rposition
scan
size_hint
skip_while
skip
step_by
sum
take_while
take
unzip
zip
```


## classic:
`for_each`
`map`
`flat_map`
`filter`
`fold`

## take 2 iterators:
`chain` - creates a new iterator over both in sequence.
`zip`

## ret -> bool:
`all` &mut self
`any`


## consumers:
`collect`
`count`

## comparisions:
`ge, gt, le, lt, ne, eq, cmp, partial_cmp`



```rust

// * &self
fn size_hint(&self) -> (usize, Option<usize>);


// * &mut self

fn by_ref(&mut self) -> &mut Self;

fn next(&mut self) -> Option<Self::Item>;

fn nth(&mut self, n: usize) -> Option<Self::Item>;

fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
   where P: FnMut(&Self::Item) -> bool;

fn all<F>(&mut self, f: F) -> bool
   where F: FnMut(Self::Item) -> bool;

fn any<F>(&mut self, f: F) -> bool
   where  F: FnMut(Self::Item) -> bool;

fn position<P>(&mut self, predicate: P) -> Option<usize>
   where P: FnMut(Self::Item) -> bool;

fn rposition<P>(&mut self, predicate: P) -> Option<usize>
   where P: FnMut(Self::Item) -> bool,
         Self: ExactSizeIterator + DoubleEndedIterator;


// * self

fn enumerate(self) -> Enumerate<Self>;

fn fuse(self) -> Fuse<Self>;

fn last(self) -> Option<Self::Item>;

fn peekable(self) -> Peekable<Self>;

fn skip(self, n: usize) -> Skip<Self>;

fn step_by(self, step: usize) -> StepBy<Self>; [LAB]

fn take(self, n: usize) -> Take<Self>;

fn cycle(self) -> Cycle<Self>
   where Self: Clone;


fn collect<B>(self) -> B
   where B: FromIterator<Self::Item>;

fn cloned<'a, T>(self) -> Cloned<Self>
   where Self: Iterator<Item = &'a T>, T: 'a + Clone;

fn filter<P>(self, predicate: P) -> Filter<Self, P>
   where P: FnMut(&Self::Item) -> bool;

fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
   where F: FnMut(Self::Item) -> U, U: IntoIterator;

fn fold<B, F>(self, init: B, f: F) -> B // iterator adaptor
   where F: FnMut(B, Self::Item) -> B;

fn for_each<F>(self, f: F)
   where F: FnMut(Self::Item) -> ();

fn inspect<F>(self, f: F) -> Inspect<Self, F>
   where F: FnMut(&Self::Item) -> ();

fn map<B, F>(self, f: F) -> Map<Self, F>
   where F: FnMut(Self::Item) -> B;

fn max_by_key<B, F>(self, f: F) -> Option<Self::Item>
   where B: Ord, F: FnMut(&Self::Item) -> B;

fn max_by<F>(self, compare: F) -> Option<Self::Item>
   where F: FnMut(&Self::Item, &Self::Item) -> Ordering;

fn max(self) -> Option<Self::Item>
   where Self::Item: Ord;

fn partition<B, F>(self, f: F) -> (B, B)
   where B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool;

fn product<P>(self) -> P
   where P: Product<Self::Item>;

fn rev(self) -> Rev<Self>
   where Self: DoubleEndedIterator;

fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
   where F: FnMut(&mut St, Self::Item) -> Option<B>;

fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
   where P: FnMut(&Self::Item) -> bool;

fn sum<S>(self) -> S
   where S: Sum<Self::Item>;

fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
   where P: FnMut(&Self::Item) -> bool;


fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>
  where U: IntoIterator<Item = Self::Item>;

fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
   where U: IntoIterator;

fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB) 
   where FromA: Default + Extend<A>, 
         FromB: Default + Extend<B>,
         Self: Iterator<Item = (A, B)>;
```
