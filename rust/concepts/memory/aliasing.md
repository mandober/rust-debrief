# Aliasing

Rust has scope-based aliasing analysis.
The essence of the aliasing analysis in Rust is based on lexical scopes (barring threads).

The beginner level explanation that you probably know is:
- if you have a &T, then there is no &mut T to the same instance,
- if you have a &mut T, then there is no &T or &mut T to the same instance.

As suited to a beginner, it is a slightly abbreviated version. For example:

```rust
fn main() {
    let mut i: i32 = 32;
    let mut_1: &mut 32  = &mut i;
    let mut_2: &mut i32 = mut_1;

    println!("{}", mut_2);
}
```

is perfectly fine, even though both a `mut_ref: &mut i32` and a `x: &i32` point to the same instance.

However, If you try to access `mut_1` after forming `mut_2`, the truth is unveiled:

```rust
fn main() {
    let mut i = 32;
    let mut_ref = &mut i;
    let x: &i32 = mut_ref;
    *mut_ref = 2;
    println!("{}", x);
}
```

```
error[E0506]: cannot assign to `*mut_ref` because it is borrowed
  |
4 |         let x: &i32 = mut_ref;
  |                       ------- borrow of `*mut_ref` occurs here
5 |         *mut_ref = 2;
  |         ^^^^^^^^^^^^ assignment to borrowed `*mut_ref` occurs here
```


So, it is fine to have both &mut T and &T pointing to the same memory location at the same time; however mutating through the &mut T will be disabled for as long as the &T exists.

In a sense, the &mut T is temporarily downgraded to a &T.


