# Ownership

- Rust requires any references to freeze the referent and its owners.
- A reference cannot outlive its referent
- A mutable reference cannot be aliased
- The ownership is realized through binding.
- Ownership refers to the relationship of variables and values: 
  a value has only one specific owner at a time.
- ownership concept provides memory safety without a GC
- Ownership rules:
  1. Every value has an owner - a variable that owns it.
  2. There must be only one owner at a time.
  3. When the owner goes out of scope, the value is dropped
- Rust avoids _aliasing_ altogether by introducing the concept of ownership: each values has exactly one owner.
- That owner is a variable and it gets that status through binding.
- value has only one owner at a time
- it is owned by only one, specific variable.
- That variable owns its value
- it is the sole owner of the value it refers to.
- Ownership of a value can change through binding: assignment, passing and returning arguments to functions or methods.
- The concept of ownership is primarily used to achieve memory safety.
- Rust creates memory pointers optimistically, 
  checks memory pointers’ limited accesses at the compiler time 
  with the usage of references and borrowing. 
  It does automatic compile time memory management by checking the lifetimes.

## Borrowing
[see](borrowing.md)

- Accessing a value is done through its owner.
- Transfer ownership, give the owned value away
  - give ownership of value, let value be consumed
  - take ownership of self, consume self: `self`
- Borrow the owned value
  - borrow immutably
  - borrow self immutably, read self: `&self`
  - borrow immutably
  - borrow self mutably, write (to) self, mutate it: `&mut self`

## Dropping

- When a binding goes out of scope, the bound data is dropped automatically.
- When other value is reassigned to mut var, previous value is dropped
- Dropping a reference has no effect on the value it points to
- dropping a raw pointer has no effect on the lifecycle of any other value.



## Ownership relations

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

