# Traits

- A trait defines a certain behavior or capability.
- Blank is a trait without method definitions (type capability marker).
- Behavior Enforcing Traits (BET) define required methods (fn signature only).
- BET with default methods (full fn, signature and body)
- Types that impl BETs must provide required methods with the same signatures.
- Default methods can be overridden if needed.
- Traits can also have associated types, assoc. constants, associated fns.


<!-- TOC -->

- [Traits](#traits)
  - [Empty trait](#empty-trait)
  - [Behavior enforcing trait](#behavior-enforcing-trait)
  - [Default method](#default-method)
- [Trait bounds](#trait-bounds)
- [Orphan Rule](#orphan-rule)

<!-- /TOC -->

## Traits
Traits are similar to interfaces from other languages; they define an abiding  contract that the parties ascribing to it must respect. A trait may define a certain behavior or capability.

### Empty trait
Unlike behavior enforcing traits, empty (blank) traits do not define any methods; they are used as markers, grouping types by some capability. The standard library has several of such traits in `std::marker` module: `Copy`, `Sync`, `Send`, `Sized` and `Unsize`.

```rust
// Example of blank trait:
trait Integer {}
// implementing blank trait
impl Integer for u8 {}
```


### Behavior enforcing trait
This trait enforces a certain behavior by defining a method signature only (no fn body). The type implementing this trait must use that exact same method signature and provide the method's body. A trait can define more than one required method signatures; their proper implementation is always enforced by the compiler.

Example of behavior enforcing trait:

```rust
// decalring a trait and its required method
trait Documentable {
  fn document(&self) -> String;
}
// the type implementing it
impl Documentable for Article {
  fn document(&self) -> String {
    // ...must provide the body of the method
    // compatible with the method's signature
  }
}
```

### Default method
The traits of this kind are the same as previous, except they have full methods declared (fn signatures along with the bodies) providing a default behavior. This default implementation is usually provided in situations when the types implementing the trait would've probably defined the same implementation themselves.

Example of behavior enforcing trait with default method:

```rust
// declaring a trait with default method
trait Summarizable {
  fn summary(&self) -> &str {
    "No summary. Check later."
  }
}

// implementing type can use the default:
impl Summarizable for Article {}

// or override it:
impl Summarizable for Article {
  fn summary(&self) -> &str {
    &self.0[0..100]
  }
}
```

A very convenient pattern is a trait that demands definition of just one method, but provides default implementation for many other methods that depend on the required one. A prime example of this pattern is the `Iterator` trait, whose only requirement of the implementing type is to provide a single method in order to score numerous default methods for free. Namely, by providing impl of the `next` method, a type implementing `Iterator` receives instant access to a lot of default useful methods that depend on the `next`.


```rust
pub trait Iterator {
  // assoc. type
  type Item;

  // required method
  fn next(&mut self) -> Option<Self::Item>;

  // multitude of attached methods based on the `next`
  // ...
}

// impl one, get multitude for free:
impl Iterator for Counter {
  type u8;
  fn next(&mut self) -> Option<Self::Item>;
}
```


## Trait bounds
Traits can be combined with generic types in order to constrain a generic type to those types that have a particular behavior, rather than any type at all. In situations where we use generic type parameters, we can use trait bounds to specify, at compile time, that the generic type may be any type that implements a certain trait and therefore has the expected behavior.

The behavior of a type consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together in order to define a set of necessary behaviors.


## Orphan Rule
A restriction to note with trait implementations: we may implement a trait on a type as long as either the trait or the type are local to our crate. In other words, we aren’t allowed to implement external traits on external types. This restriction is part of what’s called the orphan rule; it's called the orphan rule because the parent type is not present.
