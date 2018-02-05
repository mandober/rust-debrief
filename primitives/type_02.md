# Notes

concrete type: `i32`, `f64`, `bool`, etc.
generic type: `T`
reference: `&T`
dereference: `*&T`
var: `let var`
var mut: `let mut var`
array: `[T; N]`
array ref: `&[T; N]`
slice: `&[T]`, `&T[..]`
String slice: `&str`, `&str[..]`
vector: `Vec<T>`
fn object (fn type or fn pointer): `Fn(T)->T`
method: `.`; `T.method()`
associated fn: `::`; `T::af()`
trait object: `&TraitName` or `Box<TraitName>`
trait object can be get by coercion or by casting: `&x as &TraitName`
raw pointer: `*const T`
raw pointer mut: `*mut T`


```rust

// binding a value to a type-annotated varible
let v: u8 = 255;
// taking a reference to a value (borrowing)
let r: &u8 = &v;
// dereferencing a reference to a value
let v: u8 = *r;

// binding a value to a mutable type-annotated varible
let mut v: u8 = 255;
// taking a mutable reference to a mutable value
let mr: &mut u8 = &mut v;
// dereferencing a reference to a value
let d: u8 = *mr;


// function:
fn fn_name(p: u8) -> u8 { p }
// function pointer with type annotation
let fp: fn(u8)->u8 = fn_name;
// using function pointer to call fn:
let r: u8 = fp(5u8);


// array: [T; N], stack-allocated
let arr: [u8; 3] = [5, 5, 5];
// same as:
let arr: [u8; 3] = [5; 3];
// ref (slice) to array element
let el: &u8 = &arr[0];
// ref (slice) to whole array
let slice: &[u8] = &arr[..];
// or:
let slice: &[u8] = &arr;



// trait objects:
trait TraitName {
    fn method(&self) -> String;
}
// fn that takes trait object
fn func(x: &TraitName) {
    x.method();
}
let x = 5u8;
// call fn by casting
func(&x as &TraitName);
// call fn by coercing
func(&x);

```
