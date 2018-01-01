# Traits

* A trait defines a certain behaviour or capability by defining one or more
  methods that a type implementing it must provide:
  * *Required methods* are obligatory and enforced by the compiler.
  * *Default methods* are already provided by the trait, so the type 
    implementing it, can keep the defaults or override them.



- Orphan Rule
  A restriction to note with trait implementations:
  we may implement a trait on a type as long as either the trait or the type are local to our crate. In other words, we aren’t allowed to implement external traits on external types. This restriction is part of what’s called the orphan rule, from type theory. Briefly, it’s called the orphan rule because the parent type is not present.

- Traits are Rust’s take on interfaces or abstract base classes.
- Traits are a way to define behavior in a generic way.
- Traits let us abstract over behavior that types can have in common.

- Traits can be combined with generic types in order to constrain a generic type
  to those types that have a particular behavior, rather than any type at all.

- A trait tells the Rust compiler about a functionality a particular type has,
  and might share with other types. In situations where we use generic type
  parameters, we can use trait bounds to specify, at compile time, that the
  generic type may be any type that implements a trait and therefore has the
  behavior we want to use in that situation.

- The behavior of a type consists of the methods we can call on that type.
  Different types share the same behavior if we can call the same methods on all
  of those types. Trait definitions are a way to group method signatures together in order to define a set of behaviors necessary to accomplish some purpose.

- Kinds of traits: marker, copy vs move, size (Sized, ?Sized),...




Declaring a trait
- empty trait, marker trait
- trait (with requred methods)
- trait (with default methods)
- trait with requred and default methods
- extension traits

Implementing a trait
- by defining req methods
- by overriding default methods with own
- by overriding default methods with empty
- for (primitive) type `impl IsEmoji for char`




## Declaring a trait


Huge number of traits is defined in the Standard Library and users can define their own. Here is `Write` trait from std, without provided methods:

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    //...
}
```
Methods write and flush are required.





Declaration of `Summarizable` trait that defines behavior each type that implements it needs to provide: to have a method`summary` with this exact signature.

```rust
trait Summarizable {
    fn summary(&self) -> String;
}
```

## Implementing a trait
Implementing a trait on a type is similar to implementing other methods.


impl trait for type

(instead of: impl type)




## Marker traits
- `Sized` Marker for types with a fixed size known at compile time.
- `Clone` Types that support cloning of values.
- `Copy` Marker for types that can be copied.
- `Sync`
- `Send`

## Conversion traits
- `AsRef` and `AsMut` Conversion for borrowing references.
- `Borrow` and `BorrowMut` Conversion with hashing, ordering, and equality.
- `From` and `Into` Conversion traits for transforming one type to another.
- `ToOwned` Conversion trait for converting a reference to an owned value.
- `Deref` and `DerefMut`Traits for smart pointer types.

## Utility traits
- `Drop` Destructors. Cleanup code called when a value is dropped.
- `Default` Types that have a sensible "default value".

