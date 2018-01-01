# Advanced traits
https://doc.rust-lang.org/book/second-edition/ch19-03-advanced-traits.html



## Associated Types
Associated Types are a way of associating a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used in this type’s place for the particular implementation.

An example of a trait with an associated type is the `Iterator` trait. It has an associated type `Item` that stands in for the type of the values that are iterated over:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
Item is just a type alias: in order to avoid redefining the return type every
time this trait is implemented, we just set alias to some type (that is being iterated over):

```rust
impl Iterator for Counter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {}
```

Which might as well be:

```rust
impl Iterator for Counter {
    fn next(&mut self) -> Option<Self::u8> {}
```



## Associated Types Versus Generics

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