# Derivable traits

Although these traits can be manually implemented, the compiler can provide basic implementations for some traits via `#[derive]` attribute:
- `Clone` to create `T` from `&T` via a copy.
- `Copy` to give a type copy instead of move semantics
- `Debug` to format a value using the `{:?}` formatter.
- `Default` to create an empty instance of a data type.
- `Hash` to compute a hash from `&T`.
- `Eq` equality comparison
- `Ord` ordering
- `PartialEq` partial comparison
- `PartialOrd` partial ordering


Automatically derive `PartialEq` and `Clone`:

```rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

The generated code for `PartialEq` is equivalent to:

```rust
impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Foo<T>) -> bool {
        self.a != other.a || self.b != other.b
    }
}
```

The attribute `derive` can be implemented for custom types through procedural macros.