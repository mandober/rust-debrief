# Bounds

Bounds Constraints


- `T: Debug` generic type T constrained to types that implement `Debug` trait.
- `T: 'a` type T must outlive `'a`, it cannot transitively contain any refs with lifetimes shorter than `'a`.
- `T: 'static` T contains no borrowed references other than `'static`
- `'b: 'a` generic lifetime `'b` must outlive lifetime `'a`.
- `T: ?Sized` allow generic type parameter to be a dynamically-sized type.
- `'a + trait, trait + trait` compound type constraint.




## Lifetimes as bounds

Just like generic types can be bounded so can lifetimes.  
The bound symbol, `:`, has a slightly different meaning. 

This reads as:
- `T: 'a` all references in `T` must outlive lifetime `'a`.
- `T: Trait + 'a` type `T` must implement trait `Trait` and all references in `T` must outlive lifetime `'a`.


This pattern is commonly found in types that represent some collection.

For example, a collection called `List`, which manages a sequence of nodes, `Node`, where each node carries a payload, `P`. So far we have: 

```rust
struct Node<P> {
  payload: P
}

struct List<P> {
  nodes: Node<P>
}
```

Both structs own their data, node owns the payload, and list owns the nodes; but implementing an `Iterator` trait to yield references to payloads will require lifetimes and lifetime bounds.

Implementing an iterator means creating a helper struct, usually called `Iter`, to hold each reference, which the `next` method yields; also a new `List` inherent method, conventianally named `iter`, will initialize iteration by preparing a first reference - by placing a reference to the first node of the list in the `Iter` struct; then the `next` method will take that reference and serve it (its job is also to prepare and place back a next reference)

it holds a reference to node which holds a payload

a new method, by convention called `iter()`, that yields payload references is needed;

also a new struct, called `Iter`, is created to hold each of these references (that the `next` method will produce).


```rust
impl<P> List<P> {
  fn iter(&self) -> Iter<'p, P> { 
    /*  ... */
  }
}

impl<P> Iterator for List<P> {
  type Item = Node<P>;
  fn next(&mut self) -> Option<&Self::Item> { /*  ... */ }
}

struct Iter<'p, P: 'p> {
  refnodes: Option<&'p Node<P>>
}
```









