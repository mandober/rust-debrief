# Methods and associated functions

*Methods* are different from functions in that they’re defined within the context 
of a data structure (struct, enum, trait object). Methods are accessed with dot
opearator (`.`).

*self* - their first parameter is always `self`, which represents the instance 
of the data structure the method is being called on. Methods can take ownership 
of self or borrow self mutably or immutably - depending on a level of 
ownership required, self can be: `self` (consuming, taking ownership), 
`&self` (reading, borrowing), `&mut self` (mutating, borrowing mutably).

*Associated functions* are also defined within the context of a data structure, 
but they don't refer `self`. Common example is the `new()` assoc. function.
Associated functions are accessed with double colon opearator(`::`).

*Self* - methods and associated functions can refer to `Self` as a shorthand for 
the type i.e. data structure they are defined in the context of.
For example, when `struct Circle` implements a method, within `impl Circle` block
methods and associated functions can refer to `Circle` type or they can just  
refer to `Self`, which is the same thing. Also, depending on the level of 
ownership refered to, Self can be used as:
`Self` or `Circle` for consumation,
`&Self` or `&Circle` for borrowing,
`&mut Self` or `&mut Circle` for mutating.

*dot operator* (`.`)
Dot operator automatically (de)references objects during method call.
It will perform as many referencing, by adding `&` or `&mut`, or dereferencing, 
by adding `*`, as needed to its left operand, in order to match the signature of
the method call.

*double colon opearator* (`::`)
is used to call associated functions.




## Self types
The special type `Self` has a meaning within `trait` and `impl` 
where it refers to the implementing type.

The notation `self`      is a shorthand for `self: Self`
The notation `&self`     is a shorthand for `self: &Self`
The notation `&mut self` is a shorthand for `self: &mut Self`


For example, in:

```rust
pub trait From<T> {
    fn from(T) -> Self;
}

impl From<i32> for String {
    fn from(x: i32) -> Self {
        x.to_string()
    }
}
```

The notation `Self` in the `impl` *refers to the implementing type*: `String`. 

In another example:

```rust
trait Printable {
    fn make_string(&self) -> String;
}

impl Printable for String {
    fn make_string(&self) -> String {
        (*self).clone()
    }
}
```

The notation `&self` is a *shorthand* for `self: &Self`.