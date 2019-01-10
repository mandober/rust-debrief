# Closures

- each closure has a unique and anonymous type (non-annotatable)
- they impl: `Fn: FnMut: FnOnce`
- non-capturing closures can be coerced to function pointers
- capture: by immutable borrow, unique immutable borrow, mut borrow, by move



## Changes
Closures [now implement][49299] `Copy` and/or `Clone` if all captured variables implement either or both traits.

[49299]: https://github.com/rust-lang/rust/pull/49299



## Closure implementation
A closure type is approximately equivalent to a struct which contains the captured variables. For instance, the following closure:

```rust
fn fx<F : FnOnce() -> String> (g: F) {
  println!("{}", g());
}

let mut s = String::from("foo");
let t = String::from("bar");

fx(|| { s += &*t; s });
// Prints "foobar"
```

generates a closure type roughly like the following:

```rust
struct Closure<'a> {
  s : String,
  t : &'a String,
}

impl<'a> (FnOnce() -> String) for Closure<'a> {
  fn call_once(self) -> String {
    self.s += &*self.t;
    self.s
  }
}
```

so that the call works as if it were:

```rust
fx(Closure {s: s, t: &t} );
```

The compiler's preference when capturing a closed-over variable:
- by immutable borrow
- unique immutable borrow
- by mutable borrow
- by move

It will pick the first choice of these that allows the closure to compile. The choice is made only with regards to the contents of the closure expression; the compiler does not take into account surrounding code, such as the lifetimes of involved variables.

If the `move` keyword is used, then all captures are by move or, for `Copy` types, by copy, regardless of whether a borrow would work. The `move` keyword is usually used to allow the closure to outlive the captured values, such as if the closure is being returned or used to spawn a new thread.

__Composite types such as structs, tuples, and enums are always captured entirely__, not by individual fields. It may be necessary to borrow into a local variable in order to capture a single field:

```rust
struct SetVec {
  set: HashSet<u32>,
  vec: Vec<u32>
}

impl SetVec {
  fn populate(&mut self) {
    // borrow vec into a local variable
    let vec = &mut self.vec;
    self.set.iter().for_each(|&n| {
      // don't use vec directly as self.vec.set()
      vec.push(n);
    })
  }
}
```

If, instead, the closure were to use `self.vec` directly, then it would attempt to capture self by mutable reference. But since `self.set` is already borrowed to iterate over, the code would not compile.

**Unique immutable borrows in captures**   
Captures can occur by a special kind of borrow called a *unique immutable borrow*, which cannot be used anywhere else in the language and cannot be written out explicitly. It occurs when modifying the referent of a mutable reference, as in the following example:

```rust
let mut b = false;
let x = &mut b;
{
    let mut c = || { *x = true; };
    // The following line is an error:
    //let y = &x; // error
    c();
}
let z = &x;
```

In this case, borrowing x mutably is not possible, because x is not mut. 
At the same time, borrowing x immutably would make the assignment illegal, because a `& &mut` reference may not be unique, so it cannot safely be used to modify a value. So a unique immutable borrow is used: it borrows x immutably, but like a mutable borrow, it must be unique.

In the above example, uncommenting the declaration of y will produce an error because it would violate the uniqueness of the closure's borrow of x; the declaration of z is valid because the closure's lifetime has expired at the end of the block, releasing the borrow.



## Declaration
Closures are similar to functions in some aspects, but unlike functions, they don't demand explicit type annotations in their signature if the types can be inferred by the compiler. If there's a return type annotation, the closure's body must be wrapped with curly braces.

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


## Anonymous type
Each closure expression produces a closure value that has _unique anonymous type_ that cannot be annotated.

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


## Call traits and coercions
Closure types all implement `FnOnce`, indicating that they can be called once by consuming ownership of the closure.

Additionally, some closures implement more specific call traits:
- A closure which does not move out of any captured variables implements `FnMut`, indicating that it can be called by mutable reference.
- A closure which does not mutate or move out of any captured variables implements `Fn`, indicating that it can be called by shared reference.

Move closures may still implement `Fn` or `FnMut`, even though they capture variables by move. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them.

Non-capturing closures are closures that don't capture anything from their environment. They can be coerced to function pointers (`fn`) with the matching signature.


## Other traits
All closure types implement `Sized`. Additionally, closure types implement the following traits if allowed to do so by the types of the captures it stores: `Clone`, `Copy`, `Sync`, `Send`.

The rules for `Send` and `Sync` match those for normal struct types, while `Clone` and `Copy` behave as if derived. For `Clone`, the order of cloning of the captured variables is left unspecified.

Because captures are often by ref, the following general rules arise:
- A closure is `Sync` if all captured variables are Sync.
- A closure is `Send` if all vars captured by non-unique immutable reference are Sync, and all values captured by unique immutable or mutable reference, copy, or move are Send.
- A closure is `Clone` or `Copy` if it does not capture any values by unique immutable or mutable reference, and if all values it captures by copy or move are Clone or Copy, respectively.

