# Structs
https://doc.rust-lang.org/book/second-edition/ch05-00-structs.html

A struct, is a custom data type that lets us name and package together multiple 
related values that make up a meaningful group.

The struct and enum are the building blocks for creating new types.

struct is heterogenous type group: fields of struct can be of different types.

Kinds of structs:
- C-like struct
- Tuple-like struct
- Unit-like structs



```rust
struct F();           // tuple struct
let _f1: F = F();     // type: F
let _f2: fn()->F = F; // type: fn() -> F
                      //       fn() -> F { F::{{constructor}} }

struct G;      // unit struct
let _g1 = G;
let _g2 = G(); // error[E0618]: expected function, found `G`

struct H(G);   // tuple struct


// tuple-like structs:
struct Rgb(u8, u8, u8);
let _black = Rgb(0, 0, 0);

struct Byte(u8);  // or
struct Byte (u8); // or
struct Byte (
  u8
);


// C-like struct
struct User {
    id: i32,
    name: (String, String)
}
```


```rust
struct P();
fn main() {
    let b = P;
    println!("{:?}", b);
    // error[E0277]: the trait bound 
    //  `fn() -> P {P::{{constructor}}}: std::fmt::Debug`
    //  is not satisfied
}

struct P();
fn main() {
    let b: fn()->P = P;
    println!("{:?}", b); // 0x56096605df70
}
```



To use a struct after we’ve defined it, we create an instance of
the struct by specifying concrete values for each of the fields.

*field init shorthand*: if vars have the same names as struct fields (like ES6)

`..` - struct update syntax specifies that the remaining fields should have
the same value as the fields in the given instance. (like spread in ES6)

print out struct debug info with println! macro, using `{:?}` placeholder and 
  deriving `Debug` trait with `#[derive(Debug)]` (put just before the sruct).

In methods, dot operator does auto de/referencing: Rust automatically adds 
  `&`, `&mut`, or `*` so object matches the signature of the method.
  Rust can figure out whether the method is
  - reading: `&self`
  - mutating: `&mut self`
  - consuming: `self`

In methods, fns within `impl` blocks that don’t take `self` as a parameter
  are *associated functions* called with `::` syntax. They are functions, not 
  methods, because they don’t have an instance of the struct to work with.
  In other langs known as *static methods*.

## Kinds of structs:
- C-like struct, named structs
- Tuple-like structs
- Unit-like structs


### C-like struct
structs with named fields. Each field has a name and associated type.

```rust
struct User {
    username: String,
    sign_in_count: u64,
    active: bool,
}

let mut joe = User {
    username: String::from("joe28"),
    active: true,
    sign_in_count: 1,
};
```

### Tuple-like struct
Structs that look similar to tuples, that have the added meaning the struct name provides, but don't have names associated with their fields, just field types.

The definition of a tuple struct still starts with the `struct` keyword and the struct name, which are followed by the types in the tuple.

Each struct we define is its own type, even though the fields within the struct 
have the same types. Otherwise, tuple struct instances behave like tuples:
you can destructure them into their individual pieces,
you can use a . followed by the index to access an individual value, etc.


```rust
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
```



### Unit-like structs
Structs that don't have any fields.

These are called unit-like structs since they behave similarly to the unit type. 

Unit-like structs can be useful in situations such as when you need to
impl a trait on some type, but you don't have any data that you want to
store in the type itself.

