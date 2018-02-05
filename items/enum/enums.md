# Enumeration


- Enum allows for defining a new type by enumerating all its possible values.
- Each enumeration is called a variant and has an associated tag (integer), called discriminant, which identifies it.
- Enum is, what is called in type theory, an algebraic data type, more precisely, an algebraic sum type, meaning that the number of different values it can have is the sum of the different values each of its variants can hold. Sum types are "OR" types because only one of their variants can be active (assigned to) at a time. More accurately, Enum is a disjoint (discriminated) union: if the sets have overlap, the overlap is discriminated out.


An example of enum:

```rust
enum Choice {
    A,
    B(bool),
    C(char)
}
```

## Discriminants
An enum stores a tag to indicate which of its variants the enum represents at the given instance. Tags are associated with sequence of integers, starting at 0 with a step of 1.

```rust
enum Tag {
    A,
    B(bool),
    C(char)
}
```
Above, `A` variant has a tag 0, `B` is 1 and `C` is 2.


- Since any variant of an enum can become any other variant of that enum, the memory requirement of its variants is equal to the memory requirement of its largest variant.
- Every element needs as much space as the largest element because it has to be ready to become that element anytime.
- Enum is a sum type because it is the sum of the two sets:

```rust
enum Sum {
    a(bool),
    b(u8)
}
```

This is a set of all values which are booleans or integers. This is a sum of sets, `Sum = bool + u8` i.e. Sum=2+256=258. More accurately, it is a disjoint (discriminated) union: if the sets have overlap, the overlap is discriminated out.

