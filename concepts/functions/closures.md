# Closure

- each closure has a unique anonymous type, known to the compiler only
- no two closures, even if identical, have the same type
- its type impl `Fn : FnMut : FnOnce`
- non-capturing closures can be coerced to function pointers



## Declaration
Closures are similar to functions in some aspects, but unlike functions, they don't demand explicit type annotations in their signature if the types can be inferred by the compiler.

```rust
// closure that takes no params
let noparams = || println!("pretty vacant");

// closure
let add = |x, y| x + y;
// calling this closure with i32 values fixes the types in its signature
let res = add(5, 9);
// now, the closure has fixed signature as if it were written as:
let add = |x: i32, y: i32| -> i32 { x + y };
```

If there's a return type annotation, the closure's body must be wrapped with curly braces.


## Anonymous type
Each closure expression produces a closure value that has _unique anonymous type_ that cannot be written out (cannot be annotated), but it is known to the compiler. No two closures, even if identical, have the same type.

This is similar to a function declarations: when referred to, a _function item_ (or the constructor of a tuple-like struct or enum variant), yields a zero-sized value of its _function item type_.

Closures that don't use anything from their environment can be *coerced to function pointers* (`fn`) with the matching signature.

```rust
// in IDEs, the type of `add` may appear as:
// `[closure@src\main.rs ...]`
let add = |x, y| x + y;
// this non-capturing closure can be coerced to a function pointer
// with the matching signature.
let f: fn(i32, i32) -> i32 = add;


// function declaration
fn sum(x: i32, y: i32) -> i32 {
  x + y
}
// when referred to, a function item yields a zero-sized value of its
// function item type:
let z = sum;
// the type of `z` may appear as: `fn(i32, i32) -> i32 {main::sum}`
// in compiler's error messages, but it cannot be used in annotations
```


## Save the environment
Closures may capture varables from their environment, which divides them into capturing and non-capturing closures.

```rust
let x = 5;
let y = 9;
// in IDEs, the type hint of `fx` may appear as:
// [closure@src\main.rs: ... x:&i32, y:&i32]
let fx = |z| x * y + z;
println!("{}", fx(6));


let mut x = 7;
let mut y = 9;
// [closure@src\main.rs:... x:&mut i32, y:&mut i32]
let mut res = |z| {
  x = 4;
  {
    let yy = &mut y;
    *yy = 22;
  }
  x + y + z
};
let r = res(3);
println!("{}", r);
```

When a closure uses a value from the environment, the compiler will try to infer how to provide it: as a reference, as a mutable reference, or the variable may be moved. Internally, the compiler represents a closure as a struct with impl block:

```rust
let x = 42;
// x will be borrowed, as shared ref
let closure = || x + x;

// internal representation:
struct __closure<'a> {
  x: &'a i32
}

impl __closure {
  fn __call(&self, x: i32) {
    x + x
  }
}
```



## Implemented traits
Depending on the requirements of the closure, its type implements one or more of the closure traits:
- `FnOnce` a closure can be called once; it can move out of its captured values.
- `FnMut` a closure can be called multiple times as mutable; it can mutate values from its environment. `FnMut` inherits from `FnOnce` i.e. anything implementing `FnMut` also implements `FnOnce`.
- `Fn` a closure can be called multiple times through a shared reference; it can neither move out from nor mutate captured variables, but read-only access to such values is allowed. Using `move` to capture variables by value is allowed so long as they aren't mutated or moved in the body of the closure. `Fn` inherits from `FnMut`, which itself inherits from `FnOnce`.

