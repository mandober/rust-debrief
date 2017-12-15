# Macro std::vec

Creates a Vec containing the arguments.

```rust
macro_rules! vec {
    ( $ elem : expr ; $ n : expr ) => { ... };
    ( $ ( $ x : expr ) , * ) => { ... };
    ( $ ( $ x : expr , ) * ) => { ... };
}
```

vec! allows Vecs to be defined with the same syntax as array expressions. There are 2 forms of this macro:


1. Create a Vec containing a given list of elements:

```rust
let v = vec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
```

2. Create a Vec from a given element and size:

```rust
let v = vec![1; 3];
assert_eq!(v, [1, 1, 1]);
```

Note that unlike array expressions this syntax supports all elements which implement `Clone` and the number of elements doesn't have to be a constant.

This will use `clone()` to duplicate an expression, so one should be careful using this with types having a nonstandard `Clone` implementation.

For example, `vec![Rc::new(1); 5]` will create a vector of 5 references to the same boxed integer value, not 5 references pointing to independently boxed integers.
