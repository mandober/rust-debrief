# Type parameters

Items that can be parameterized by type:
- functions
- type aliases
- structs
- enums
- unions
- traits
- implementations

Type parameters are given as a comma-separated list of identifiers enclosed in angle brackets after the name of the item; except for implementations, where they come directly after `impl` and before its definition.

The type parameters of an item are considered "part of the name", not part of the type of the item.

A referencing path must (in principle) provide type arguments as a list of comma-separated types enclosed within angle brackets, in order to refer to the type-parameterized item. In practice, the type-inference system can usually infer such argument types from context.

There are no general _type-parametric types_, only _type-parametric items_. That is, Rust has no notion of type abstraction: there are no _higher-ranked types_ (no `forall`) abstracted over other types, though higher-ranked types do exist in connection with lifetimes.

Type parameters stand in for **input** types and associated types stand in for **output** types.


## Type parameters

Within the body of an item that has type parameter declarations, the names of its type parameters are types:

```rust
fn to_vec<A: Clone>(xs: &[A]) -> Vec<A> {
    if xs.is_empty() {
        return vec![];
    }
    let first: A = xs[0].clone();
    let mut rest: Vec<A> = to_vec(&xs[1..]);
    rest.insert(0, first);
    rest
}
```

Here, first has type A, referring to to_vec's A type parameter; and rest has type Vec<A>, a vector with element type A.


## Self types

The special type Self has a meaning within traits and impls: it refers to the implementing type. For example, in:

```rust
pub trait From<T> {
    fn from(T) -> Self;
}

impl From<i32> for String {
    fn from(x: i32) -> Self {
        x.to_string()
    }
}
```

The notation Self in the impl refers to the implementing type: String. In another example:

```rust
trait Printable {
    fn make_string(&self) -> String;
}

impl Printable for String {
    fn make_string(&self) -> String {
        (*self).clone()
    }
}
```

The notation &self is a shorthand for self: &Self.