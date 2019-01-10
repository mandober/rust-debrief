# Module any

- Module `std::any`, 1.0.0
- This module implements the `Any` trait, which enables dynamic typing of any `'static` type through runtime reflection.
- Content:
  - Structs
    - `TypeId` represents a globally unique identifier for a type.
  - Traits
    - `Any` type to emulate dynamic typing.


`Any` itself can be used to get a `TypeId`, and has more features when used as a trait object.

As `&Any` (a borrowed trait object), it has the `is` and `downcast_ref` methods, to test if the contained value is of a given type, and to get a reference to the inner value as a type.

As `&mut Any`, there is also the `downcast_mut` method, for getting a mutable reference to the inner value. 

`Box<Any>` adds the `downcast` method, which attempts to convert to a `Box<T>`.

Note that `&Any` is limited to testing whether a value is of a specified concrete type, and cannot be used to test whether a type implements a trait.


## Example

Consider a situation where we want to log out a value passed to a function. We know the value we're working on implements Debug, but we don't know its concrete type. We want to give special treatment to certain types: in this case printing out the length of String values prior to their value. We don't know the concrete type of our value at compile time, so we need to use runtime reflection instead.


```rust
use std::fmt::Debug;
use std::any::Any;

// Logger function for any type that implements Debug.
fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &Any;

    // Try to convert our value to a String:
    // If successful, we want to output its len and value.
    // If not, it's a different type, just print it
    match value_any.downcast_ref::<String>() {
        Some(v) => {
            println!("String ({}): {}", v.len(), v);
        }
        None => {
            println!("{:?}", value);
        }
    }
}

// This function wants to log its parameter out prior to doing work with it.
fn do_work<T: Any + Debug>(value: &T) {
    log(value);
    // ...do some other work
}

let my_string = "Hello World".to_string();
do_work(&my_string);

let my_i8: i8 = 100;
do_work(&my_i8);
```
