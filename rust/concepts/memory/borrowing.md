# Borrowing

- Rust requires any references to freeze the referent and its owners.
- A reference cannot outlive its referent
- A mutable reference cannot be aliased
- Simple values like scalars are entirely stored on the stack
- complex values are represented with a stack-stored pointer that points to some data allocated on the heap. 
- Contrary to the heap, manipulating the stack is very cheap and efficient.
- Copy types are values stored exclusively on the stack, which can be duplicated simply by copying bits within the stack.
- By default, variable bindings have move semantics.
- However, if a type implements Copy, it instead has copy semantics
- Copy types:
  - scalars, never, unit
  - array and tuple (provided all elements are copy)
  - immutable pointer types: 
    - shared refrence
    - function pointer
    - constant raw pointer
- Borrowing Rules
  1. There can be only one mut ref at the time.
     If a value has a mutable ref, there can be no other refs (mut or not).
     The value itself is frozen during the lifetime of that mut ref.
  2. There can be more than one ref, but none of mut ref, at one time.

- _mutable reference_ represents temporary exclusive access to a value that you don't own.
- allowed to do anything to a value as long as when your loan expires, 
  wherever you loaned it from still sees a valid value.
- This means you can actually completely overwrite the value.
- A really useful special case of this is swapping a value out for another
- The only thing you can't do with an &mut is move the value out with no
  replacement.
- &mut self is great for methods that want to mutate self.

- _shared reference_ represents temporary shared access to a value that you don't own.
- Because you have shared access, you're generally not allowed to mutate anything.
- & is great for methods that only want to observe self.

- A ref is guaranteed to always point to valid data.
- To enforce this, ref have lifetimes and lifetime annotations.

- A value is owned, but it can be borrowed.
- A value is owned, but it can be either moved or copied.



---

https://doc.rust-lang.org/nightly/nomicon/ownership.html
https://doc.rust-lang.org/nightly/book/2018-edition/ch04-01-what-is-ownership.html
