# Ownership

Ingredients:
- variable (name, label)
- value (data)
- data types (move vs copy types)
- contracts: binding, ownership, borrowing, dropping
- immutable vs mutable



Ownership
- ownership
- shared read access
- exclusive read/write access

Giving access
- take ownership of self, consume self: `self`
- borrow self immutably, read self: `&self`
- borrow self mutably, write (to) self, mutate it: `&mut self`


A mutable reference represents temporary exclusive access to a value that you don't own. You're allowed to do absolutely anything you want to a value you have a mutable reference to as long as when your loan expires, wherever you loaned it from still sees a valid value. This means you can actually completely overwrite the value. A really useful special case of this is swapping a value out for another, which we'll be using a lot. The only thing you can't do with an &mut is move the value out with no replacement. &mut self is great for methods that want to mutate self.

A shared reference represents temporary shared access to a value that you don't own. Because you have shared access, you're generally not allowed to mutate anything. Think of & as putting the value out on display in a museum. & is great for methods that only want to observe self.




Ownership in Rust refers to the relationship of variables and values. 
Value is a sequence of bits together with its interpretation.
In Rust, a value has only one specific owner at a time.



It has only one owner at a time; it is owned by only one, specific variable.
That variable owns its value; it is the sole owner of the value it refers to.

Ownership of a value can change through binding:
assignment, passing and returning arguments to functions or methods.

A value is owned, but it can be borrowed.
A value is owned, but it can be either moved or copied.

# Borrowing
- A ref is guaranteed to always point to valid data.
- To enforece this, ref have lifetimes and lifetime annotations.


# Dropping
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

