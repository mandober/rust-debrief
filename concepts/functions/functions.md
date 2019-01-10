# Functions

- Function declaration consists of the function's header and optional function body surrounded by the code block.
- A function that ends in with a panic call or an infinite loop is a **diverging function**. Its return type is `!`, the **never type**; it can substitute any other (expected) type, so it can be consider as the **top type** with regards to type theory.
- Function-type modifiers:`unsafe`prefix for **unsafe functions** and `extern` prefix for **external functions** (ABI).
  - Non-external functions have an implicit ABI of `"Rust"`
  - `extern` functions without an explicit ABI have a default ABI of `"C"`.
- __Function's type__ depends on its signature, safety and ABI.



<hr>
<!-- TOC -->

- [Function definition](#function-definition)
- [Function item](#function-item)
- [Diverging functions](#diverging-functions)
- [Generic functions](#generic-functions)
- [Function pointer type](#function-pointer-type)
- [Extern functions](#extern-functions)

<!-- /TOC -->



## Function definition

Functions are declared with the keyword `fn`, followed by the function's name (identifier), a set of parenthesis sorrounding potential parameters, an `->` symbol, followed by the function's return type, and a block containg the function's body.

```rust
// the simplest fn:
fn nothing() { }
// signature:
fn nothing();
// fn item: `fn() {nothing}`

// return type is unit and it is implicit,
// but it can be explicitlly annotated:
fn nothing() -> () { }
```

Type annotations in function's signatures are mandatory: functions may declare one or more parameters and they must be type-annotated; if a function returns some value, the type of that value must be annotated. In case when the function is only used for its side-effects and actually returns nothing of interest, its return type is unit and it can be ommited.

Function's signature consists of `fn` keyword, identifier, parameter types (if any) in parenthesis, and return type (preceded by the `->` symbol).



```rust
// the simplest fn:
fn nothing() { }
// signature:
fn nothing();
// fn item: `fn() {nothing}`





// the simplest fn
fn nothing() { }
// return type (unit type) is implicit,
// but it can be explicitlly annotated:
fn nada() -> () { }

// with params:
fn sq(a: i32, b: i32) -> i32 { a * b }
// its header:
fn sq(a: i32, b: i32) -> i32;
// its signature:
fn sq(i32, i32) -> i32;

// its function item type as shown in compiler:
// fn(f64, f64) -> f64 {sq}


// with return value
fn returning() -> String { "return this".to_string() }

// with param and return value
fn param_return(a: u8) -> u8 { a * a }

// never to return
fn gone() -> ! { panic!(); }

// non-external functions have an implicit ABI of "Rust"
extern "Rust" fn() -> () {} // is the same as:
fn() -> () {}

// extern functions default to "C" ABI
extern fn() -> () {} // is the same as:
extern "C" fn() -> () {}

```


## Function item

When referred to, a **function item** (also the constructor of a tuple-like struct or enum variant) yields a *zero-sized* value of its **function item type**.

That type explicitly identifies the function:
- its name,
- its type arguments and
- its early-bound lifetime arguments (but not its late-bound lifetime arguments, which are only assigned when the function is called)
so the value does not need to contain an actual function pointer, and no indirection is needed when the function is called.

Type annotation for a function item type currently doesn't exist, but the compiler will display it, for example, as `fn(i32) -> i32 {fn_name}`.

Since the function item type explicitly identifies the function, the item types of different functions are different items; also, the same item with different generics are distinct. Mixing them will result in a type error.


```rust
// function definition
fn dv(a: f64, b: f64) -> f64 { a / b }
// when the function item is refered to, it yields
// a zero-sized value of its function item type
let var1 = dv;
// the type of `var1` is the function item type,
// but it cannot be annotated (no such syntax)

// when `var1` is deliberatly annotated with a wrong type:
let var1: u8 = dv;
// the compiler will show an error that will
// reveal this type as `fn(f64, f64) -> f64 {dv}`
// which cannot be used, as it is only for internal reference

// but if `var1` is annotated with the fn type (function pointer):
let var1: fn(f64, f64) -> f64 = dv;
// the fn item will coerce into fn type
```



## Diverging functions
- Diverging functions never return a value to the caller.
- They have a `!` for return type.
- diverging are the functions that always end in `panic!()`, or end with an infinite loop, or a function that calls another diverging function.
- main() is a diverging function, although its return type is left out.
- the typechecker checks that every control path in a function ends either with a return or diverging expression.
- Because the never type, `!`, can be substituted for any other (expected) type it can be consider as top type with regards to type theory.


```rust
// gone
fn die() -> ! { panic!(); }

// would not not compile without the `!` annotation on die fn above
fn two_paths(i: i32) -> i32 {
  if i == 42 {
    42     // return expression: typechecks with i32
  } else {
    die()  // diverging expression: typechecks with i32 or any other type
  }
}
```



## Generic functions
A generic function allows one or more parameterized types to appear in its signature. Each type parameter must be explicitly declared in an angle-bracket-enclosed and comma-separated list, following the function name.

```rust
// foo is generic over A and B
fn foo<A, B>(x: A, y: B) {
```

Inside the function signature and body, the name of the type parameter can be used as a type name. Trait bounds can be specified for type parameters to allow methods with that trait to be called on values of that type. This is specified using the where syntax:

```rust
fn foo<T>(x: T) where T: Debug {
```

When a generic function is referenced, *its type is instantiated* based on the context of the reference. For example, calling the foo function here:

```rust
fn foo<T>(x: &[T]) where T: std::fmt::Debug { /** details elided */ }

foo(&[1, 2]);
```
will instantiate type parameter `T` with `i32`.


The type parameters can also be explicitly supplied in a *trailing path component* after the function name.

This might be necessary if there is not sufficient context to determine the type parameters. For example:

```rust
mem::size_of::<u32>() == 4
```


## Function pointer type
Function pointer is annotated as `fn(P1,...)->R`, where parameters' types and return type is optional. For example: `fn(usize) -> bool`

In addition, function pointers of any signature, ABI, or safety are `Copy`,
and all safe function pointers implement `Fn`, `FnMut`, and `FnOnce`.


Function pointer types, written using the `fn` keyword, refer to a function whose identity is not necessarily known at compile-time. Function pointers can be created via a *coercion* from both, *function items* and *non-capturing closures*. A function pointer type consists of a possibly-empty set of function-type modifiers (such as unsafe or extern), a sequence of input types and an output type.

Example: function pointer type and its annotation:

```rust
let adder: fn(u8, u8) -> u8 = |x, y| x + y;
let y = adder(4, 5);


fn add(x: i32, y: i32) -> i32 {
    x + y
}

let mut x = add(5,7);

// type alias
type Binop = fn(i32, i32) -> i32;
// annotating with type alias
let bo: Binop = add;
// instead of:
let bo: fn(i32, i32) -> i32 = add;

x = bo(5,7);
```


## Extern functions
Extern functions are part of Rust's foreign function interface, providing the opposite functionality to external blocks. Whereas external blocks allow Rust code to call foreign code, `extern` functions with bodies defined in Rust code can be called by foreign code. They are defined in the same way as any other Rust function, except that they have the `extern` modifier.

```rust
// Declares an extern fn, the ABI defaults to "C"
extern fn new_i32() -> i32 { 0 }

// Declares an extern fn with "stdcall" ABI
extern "stdcall" fn new_i32_stdcall() -> i32 { 0 }
```

Unlike normal functions, extern fns have `extern "ABI" fn()` type. This is the same type as the functions declared in an extern block.

```rust
type ExtFn = extern "C" fn() -> i32
let fptr: ExtFn = new_i32;
```
