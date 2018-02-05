# Unit type

- Unit type
- empty type (empty tuple)
- type annotation: `()` the only type whose annotation is the same as the value
- value: `()`
- Unit is a type that allows only one value, so it cannot hold information.


## About 
- Unit is a structural type, it has no nominal representation, only a literal expression
- It may be regarded as the empty tuple, i.e. the product of no types.
- It may be omitted when it is the only or the last type parameter.



## Unit type
Unit type can be omitted when it is the only or the last type parameter.

For example when it specifies the return type of a function that returns nothing:

```rust
fn show() {}
// instead of annotating the unit:
fn show() -> () {}
```

The unit can be omited when it is the last type parameter, like with the Option enum, `Option<T>`, instead of `Option<T, ()>`.

However, when the unit is not the last type parameter it has to be written; for example, a Result enum with `Err` variant, but with nothing of interest in its `Ok` variant: `Result<(), E>`.




## Unit in type theory
Algebraic data type is a kind of composite type, formed by combining other types. Two common classes of algebraic types are product and sum types. These types get their name from algebra, where operations such as addition and multiplication operate on numbers, while in type theory they operate on types, giving sum and product types. Unit can be regarded as empty tuple i.e. the product of no types.



## Unit with sum types

```rust
enum BoolOption {
    Flag(bool),
    Empty
}
```

This is a set of all values which are either boolean or unit types:   
`bool + unit = 2 + 1 = 3`,   
since boolean has 2 values and unit has 1, the total number of values is 3:

```rust
let v1 = BoolOption::Flag(true);
let v2 = BoolOption::Flag(false);
let v3 = BoolOption::Empty;
```

The generic `Option` type can be regarded as a specialized version of the `Result` enum, which replaces the `Err(E)` variant with the unit type.

```rust
type Option<T> = Result<T, ()>;
```


## Unit with product types

```rust
struct Product {
    b: bool,
    u: ()
}

let v1 = Product { b: true, u: () };
let v2 = Product { b: false, u: () };
```

The set of all possible values is a Cartesian product, expressed as `Product = bool × unit`, so the number of all possible values of Product is the number of possible values of boolean (which is 2) times the number of possible values of unit (which is 1) i.e. two possible values.
