# Iterators

- iterators are lazy: no effect until consumed.
- iterators implement `Iterator` trait, which requires defining `next` method and associated type `Item`, used as return type of the `next` method. `next` method returns one item at a time, wrapped in `Some`, unless the iteration is over, in which case it returns `None`.
- xxx
- an iterator is any type that implements `Iterator`.
- iterators automatically implement `IntoIterator`, with an `into_iter` method that simply returns the iterator.
- an iterable is any type that implements `IntoIterator`; call its `into_iter` method to get an iterator over it.
- an iterator produces items i.e. values.
- consumer is the code that receives the items an iterator produces.
- xxx
- `IntoIterator` and its `into_iter` method:
  - Given a _shared_ reference to the collection, `into_iter` returns an iterator that produces shared references to its items.
  - Given a _mutable_ reference to the collection, `into_iter` returns an iterator that produces mutable references to the items.
  - Given a collection by _value_, `into_iter` returns an iterator that takes ownership of the collection and returns items by value.



## Iteration methods:
- `iter`      produces iterator over references, `&T`.
- `iter_mut`  produces iterator over mut references, `&mut T`.
- `into_iter` produces iterator that takes ownership and returns owned `T`.


## Iterator trait:

```rust
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}
```

```rust
let mut iter = vec![1,2,3].iter();
println!("{:?}", iter.next());

let v = vec![1,2,3];
let mut iter = v.iter();
println!("{:?}", iter.next());
```


## Adaptors

- __Consuming adaptors__ are methods that consume iterator. Methods that call the `next` method are consuming adaptors, since calling them uses up the iterator.
- __Iterator adaptors__ are methods that produce other iterators. These methods transform iterators into different kind of iterators and allow for chaining of multiple calls to iterator adaptors. In the end, they require a call to one of 
consuming adaptor methods in order to collect results.


## Iterators are lazy

Producing an iterator doesn't do anything until it is consumed, either with a `for` loop or with a consumer method.






## Implementing iterator

<details>

<summary>Implementing iterator...</summary>

Creating an iterator for custom collection involves two steps:
1. Creating a struct to hold the iterator's state
2. Implementing `Iterator` for that struct.
  
This is why there are so many structs in `iter` module: there is one for each iterator and iterator adapter.

```rust
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
        if self.count < 6 { Some(self.count) } else { None }
    }
}

// now we can use it:
let mut counter = Counter::new();
let x = counter.next().unwrap();
println!("{}", x);
```

</details>




## IntoIterator

<details>

<summary>IntoIterator ...</summary>

There's a trait in the standard library for converting something into an iterator: `IntoIterator`. This trait has one method, `into_iter`, which converts the thing implementing `IntoIterator` into an iterator.

std contains this implementation of `IntoIterator`:
`impl<I: Iterator> IntoIterator for I`

In other words, all `Iterators` implement `IntoIterator`, by just returning themselves. This means 2 things:
1. If you're writing an `Iterator`, you can use it with a `for` loop.
2. If you're creating a collection, implementing `IntoIterator` for it will allow your collection to be used with the `for` loop.


When a type implements `IntoIterator`, you can call its `into_iter` method, just like `for` loop would. Collections provide several implementations of `IntoIterator`: for shared references, mutable references, and moves.

Given a _shared_ reference to the collection, `into_iter` returns an iterator that produces shared references to its items.   
  For example, `(&favorites).into_iter()` would return an iterator whose `Item` type is `&String`.

Given a _mutable_ reference to the collection, `into_iter` returns an iterator that produces mutable references to the items.   
  For example, if vec is `Vec<String>`, the call `(&mut vector).into_iter()` returns an iterator whose `Item` type is `&mut String`.

When passed the collection by _value_, `into_iter` returns an iterator that takes ownership of the collection and returns items by value; the item's ownership moves from the collection to the consumer, and the original collection is consumed in the process.  
  For example, the call `favorites.into_iter()` returns an iterator that produces each string by value; the consumer receives ownership of each string. When the iterator is dropped, any elements remaining are dropped too.


The `for` loop calls `IntoIterator::into_iter` on its operand (the given collection) resulting in these 3 iteration idioms: 
- iterating over shared references
- iterating over mutable references
- consuming the collection by iterating over its elements and taking their ownership.

```rust
for element in &collection { ... }
for element in &mut collection { ... }
for element in collection { ... }
```


</details>



## Iter

We can call `v.iter()` on something like a vector or slice. This creates an `Iter<'a, T>` type and it is this `Iter<'a, T>` type that implements the `Iterator` trait and allows us to call functions like map. 

It is important to note that this `Iter<'a, T>` type only has a reference to `T`. This means that calling `v.iter()` will create a struct that borrows from `v`. Use the `iter()` function if you want to iterate over the values by reference.
