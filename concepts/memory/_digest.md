


## Move semantics
types that don't implement the `Copy` trait have move semantics.

```rust
let owner = String::from("literal");
let new_owner = owner; // move: owner is dropped
```
If copy of the value is needed, there is explicit clone method.


## Copy semantics
types that implement the `Copy` trait have copy semantics.

```rust
let owner1 = 42_u8;
let owner2 = owner1; // copy: owner1 is still valid
// owner1 and owner2 are each owners of their own value, which was copied
```
Every assignment of the value results in a copy of the value.


## immutable borrowing
immutable (shared) borrowing of value freezes the value ❄

```rust

```


the original object is only frozen: you can still take more non-mutable references, but you cannot move or take mutable references of it.


## mutable borrowing
mutable (exclusive) borrowing of value locks the value 🔒

```rust

```

the original object is effectively locked for the duration of the borrow, rendering it unusable.





Even a simple statement packs a lot of rules:

```rust
let n = 5;
```
First, declaring variables like n, cannot happend on its own: either we 
explicitly type annotate the variable or we immediately make an assignment, so 
type can be inferred.

```rust
let n;          // error: "cannot infer type for _"
let n: i32;     // ok: explicit type annotation
let n = 5;      // ok: type infered as i32 (default for integers)
let n: i32 = 5; // ok: reduntant explicit type annotation
let n: i16 = 5; // ok: explicit type annotation
```


