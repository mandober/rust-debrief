# Associated Types


<!-- TOC -->

- [Associated Types](#associated-types)
- [Associated Types, another example](#associated-types-another-example)

<!-- /TOC -->



Generics have both input and output types: type parameters stand in for **input** types and associated types stand in for **output** types. Input types are specified with generic type parameters and output types are specified using associated types.


## Associated Types
Associated Types are a way of associating a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used in this type's place for the particular implementation.

An example of a trait with an associated type is the `Iterator` trait. It has an associated type `Item` that stands in for the type of the values that are iterated over:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```


When we impl the `Iterator` trait on the `Counter` struct, we then specify that the `Item` type is `u32`:

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
```

This feels similar to generics.

```rust
// hypothetical definition of the Iterator using generics:
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```
The difference is that with the generic definition above, we could also implement `Iterator<String>` for `Counter`, or any other type as well, so that we’d have multiple implementations of Iterator for Counter. 
In other words, *when a trait has a generic parameter, we can implement that trait for a type multiple times*, changing the generic type parameters with concrete types each time.
Then when we use the next method on Counter, we’d have to provide type annotations to indicate which implementation of Iterator we wanted to use.

With associated types, *we can’t implement a trait on a type multiple times*. Using the actual definition of Iterator, we can only choose once what the type of Item will be, since there can only be one impl Iterator for Counter. We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.

The benefit of not having to specify generic type parameters when a trait uses associated types shows up in another way as well. Consider these two traits:

```rust
trait GGraph<Node, Edge> {
    // methods
}

trait AGraph {
    type Node;
    type Edge;
    // methods
}
```
If we were to impl a fn that computes the distance between two nodes, constrained to only the types that implement Graph trait, defined using generics, the fn signature would have to be:

```rust
fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { }
```

This fn would need to specify the GTPs: N, E, and G, where G is bound by the trait GGraph that has type N as its Node type and type E as its Edge type. Even though distance doesn’t need to know the types of the edges, we’re forced to declare an E parameter, because we need to use the GGraph trait and that requires specifying the type for Edge.

Contrast with the definition of distance in Listing 19-24 that uses the AGraph trait from Listing 19-22 with associated types:

```rust
fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 { }
```


## Associated Types, another example
https://rustbyexample.com/generics/assoc_items/the_problem.html

A trait that is generic over its container type has type specification requirements - users of the trait must specify all of its generic types.

In the example below, the `Contains` trait allows the use of the generic types `A` and `B`. The trait is then implemented for the `Container` type, specifying `i32` for `A` and `B` so that it can be used with `fn difference()`.

Because `Contains` is generic, we are forced to explicitly state all of the generic types for `fn difference()`.

Practically we want a way to express that A and B are determined by the input C.


```rust
struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

// `C` contains `A` and `B`.
// In light of that, having to express `A` and `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 
where C: Contains<A, B> {
    container.last() - container.first()
}


fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
```



https://rustbyexample.com/generics/assoc_items/types.html

The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types.


```rust
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains {
    // Define generic types here whose methods we'll be able to utilize.
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
```