# Ownership

https://doc.rust-lang.org/nightly/nomicon/ownership.html


Ownership is Rust's most unique feature, enabling it to make memory safety guarantees without needing a garbage collector (GC). The primary motivator for introducing ownership is to avoid variable aliasing. Rust's compiler gathers information about variables and scopes, so it can catch a variable that tries to leave the scope in which the value, it refers to, is valid.

The concept of ownership is primarily used to achieve memory safety. Rust creates pointers optimistically, checks their limited accesses at compile-time with the usage of References and Borrowing. And it does automatic compile time memory management by checking the Lifetimes.

Ownership refers to the relation of a variable and a value: every value must have only one specific owner at a time. A value's owner is a particular variable that became its owner through the binding.


```rust
fn main() {
  // declare varables
  let x: String;
  let y: i32;

  // bind the values
  x = String::from("RAII");
  y = 42;

  // or, more commonly
  let x = String::from("RAII");
  let y = 42;

} // both vars went out of scope - drop their values
```

There are 2 kinds of values:
- values with copy semantics
- values with move semantics







Rules
1. Every value has an owner - a variable that owns it.
2. There must be only one owner at a time.
3. When the owner (owning variable) goes out of scope, the value is dropped




An assignment is a binding that sets the value stored at the memory address denoted by a variable name i.e. an assignment copies a value into the variable.

Ownership of a value can change through binding:
assignment, passing and returning arguments to functions or methods.







## Borrowing
A mutable reference represents temporary exclusive access to a value that you don't own. You're allowed to do absolutely anything you want to a value you have a mutable reference to as long as when your loan expires, wherever you loaned it from still sees a valid value. This means you can actually completely overwrite the value. A really useful special case of this is swapping a value out for another, which we'll be using a lot. The only thing you can't do with an &mut is move the value out with no replacement. &mut self is great for methods that want to mutate self.

A shared reference represents temporary shared access to a value that you don't own. Because you have shared access, you're generally not allowed to mutate anything. Think of & as putting the value out on display in a museum. & is great for methods that only want to observe self.


A value is owned, but it can be borrowed.
A value is owned, but it can be either moved or copied.

- A ref is guaranteed to always point to valid data.
- To enforce this, ref have lifetimes and lifetime annotations.


## Dropping
When a binding goes out of scope, the bound data is dropped automatically.
When other value is reassigned to mut var, previous value is dropped

Dropping a reference has no effect on the value it points to
dropping a raw pointer has no effect on the lifecycle of any other value.



- `T`, `&T`, `&mut T`
- `FnOnce`, `Fn`, `FnMut`
- `Copy` types
- `move` in closures
- `ref`, `ref mut` in matches


1. owns `T`, `*(&T)`, `*(&mut T)`
   `move`
   consumes/moves/takes ownership
   call-by-value `FnOnce`
   method takes self: `self`
2. borrows/reads/shared reference, `&T`
   `ref`
   call-by-reference `Fn`
   method borrows self: `&self`
3. consumes/mutates/writes/mutable reference, `&mut T`
   `ref mut`
   call-by-mutable-reference `FnMut`
   method borrows self mutably: `&mut self`



`T`,`*&T`,`*&mut T` | `&T`  | `&mut T`
--------------|-------------|-----------------
**owner**     | **refs**    | **refs**
owned type    | borrowed    | mutably borrowed
own           | borrow      | borrow mutably
consumes      | borrows     | mutates
owns          | reads       | writes
`move`        | `ref`       | `ref mut`
call-by-value | call-by-ref | call-by-mut-ref
`FnOnce`      | `Fn`        | `FnMut`
`self`        | `&self`     | `&mut self`
`into_iter`   | `iter`      | `iter_mut`


## Rules

1. There can be only one mut ref at the time.
   If a value has a mutable ref, there can be no other refs (mut or not).
   The value itself is frozen during the lifetime of that mut ref.

2. There can be more than one ref, but none of mut ref, at one time.


## Relationships

__method__  
In methods, dot operator does auto de/referencing: Rust automatically adds 
`&`, `&mut`, or `*` so object matches the signature of the method.
Rust can infer whether the method is:
- `&self` *reading*
- `&mut self` *mutating*
- `self` *consuming*

__Iterator__  
- `iter`      produces an iterator over *immutable references*.
- `iter_mut`  produces an iterator over *mutable references*.
- `into_iter` produces an iterator that takes *ownership* and returns *owned* values.

__Closure__  
- `Fn`      trait *borrows* values from the env.
- `FnMut`   can change the env since it *mutably borrows* values.
- `FnOnce`  *consumes* vars it captures from its enclosing scope.

