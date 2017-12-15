# Module `boxed`
https://doc.rust-lang.org/std/boxed/

`std::boxed` 1.0.0

A pointer type for heap allocation.

`Box<T>`, casually referred to as a *box*, provides the simplest form of *heap* 
allocation in Rust. Boxes provide *ownership* for this allocation, and *drop*
their contents when they go out of scope.


## Creating a box:

```rust
let x = Box::new(5);
```


## Creating a recursive data structure:

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
// This will print Cons(1, Cons(2, Nil)).
```
Recursive structures must be boxed, because if the 
definition of Cons looked like this:
```rust
Cons(T, List<T>),
```
It wouldn't work. This is because the size of a List depends on how many elements 
are in the list, and so we don't know how much memory to allocate for a Cons. 
By introducing a Box, which has a defined size, we know how big Cons needs to be.


## Structs
`Box`
A pointer type for heap allocation.

`ExchangeHeapSingleton`
[LAB] This the singleton type used solely for `boxed::HEAP`.

`IntermediateBox`
[LAB] IntermediateBox represents uninitialized backing storage for Box.


## Constants
`HEAP`
[LAB] A value that represents the heap. This is the default place
that the box keyword allocates into when no place is supplied.

## Traits
`FnBox`
[LAB] FnBox is a version of the `FnOnce` intended for use with boxed closure 
objects. The idea is that where one would normally store a `Box<FnOnce()>` in a 
data structure, you should use `Box<FnBox()>`. The two traits behave essentially 
the same, except that a `FnBox` closure can only be called if it is boxed. 
Note that `FnBox` may be deprecated in the future if `Box<FnOnce()>` closures 
become directly usable.
