# Function item types
https://doc.rust-lang.org/reference/types.html#function-item-types

When referred to, a function item (or the constructor of a tuple-like struct or enum variant), yields a zero-sized value of its *function item type*.

Function item type explicitly identifies the function: its name, its type arguments, and its early-bound lifetime arguments (but not its late-bound lifetime arguments, which are only assigned when the function is called) - so the *value doesn't need to contain an actual function pointer*, and no indirection is needed when the function is called.

There is currently no syntax that directly refers to a function item type, but the compiler will display the type as something like `fn() {foo::<u32>}` in error messages.


Because the function item type explicitly identifies the function, 
the *item types of different functions are different items*; the same item with different generics are distinct types, and mixing them will create a type error:

```rust
fn foo<T>() { }
fn bar<T>() { }

// function item type CANNOT be type annotated
let x = &mut foo::<i32>;
// although compiler will say: found type `&mut fn() {main::foo::<i32>}`

// but, dereferencing it, and binding it to the same fn item is ok:
*x = foo::<i32>;
// ERROR: mismatched types
*x = foo::<u32>;
// ERROR: expected fn item, found a different fn item
*x = bar::<i32>;
```


However, there is a *coercion from function items to function pointers* with the *same signature*, which is triggered not only when a function item is used when a function pointer is directly expected, but also when different function item types with the same signature meet in different arms of the same `if` or match:

```rust
// foo_ptr_1 has function pointer type `fn()` here
let foo_ptr_1: fn() = foo::<i32>;

// and so does foo_ptr_2. It type-checks.
let foo_ptr_2 = if want_i32 {
    foo::<i32>
} else {
    foo::<u32>
};
```
