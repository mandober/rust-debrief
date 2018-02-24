# Tuple

- tuple is a heterogeneous product of other types, its elements
- it has no nominal name and is instead structurally typed, `(bool, u8)`

Tuples are types but they are dependent on their length as well as the types of their components, so there is theoretically an infinite number of tuple types

Tuple types and values are denoted by listing the types or values of their
elements, respectively, in a parenthesized, comma-separated list.

Because tuple elements don't have a name, they can only be accessed by
pattern-matching or by using `N` directly as a field to access the `N`th
element.

An example of a tuple type and its use:

```rust
type Pair<'a> = (i32, &'a str);
let p: Pair<'static> = (10, "ten");
let (a, b) = p;

assert_eq!(a, 10);
assert_eq!(b, "ten");
assert_eq!(p.0, 10);
assert_eq!(p.1, "ten");
```

For historical reasons and convenience, the tuple type with no elements (`()`)
is often called ‘unit’ or ‘the unit type’.
