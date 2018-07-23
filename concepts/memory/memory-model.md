# Memory model


A Rust program's memory consists of a static set of items and a heap.

Immutable portions of the heap may be safely shared between threads, mutable portions may not be safely shared, but several mechanisms for effectively-safe sharing of mutable values, built on unsafe code but enforcing a safe locking discipline, exist in the standard library.

Allocations in the stack consist of _variables_, and allocations in the heap consist of _boxes_.


## Memory allocation and lifetime

The items of a program are those functions, modules and types that have their value calculated at compile-time and stored uniquely in the memory image of the rust process.

Items are neither dynamically allocated nor freed.

The heap is a general term that describes boxes.

The lifetime of an allocation in the heap depends on the lifetime of the box values pointing to it.

Since box values (pointers to boxes) may themselves be passed in and out of frames, or stored in the heap, heap allocations may outlive the frame they are allocated within.

An allocation in the heap is guaranteed to reside at a single location in the heap for the whole lifetime of the allocation. It will never be relocated as a result of moving a box value - it will have a _stable address_.


## Memory ownership

When a stack frame is exited, its local allocations are all released, and its references to boxes are dropped.


## Variables

A variable is a component of a stack frame, either a _named function parameter_, an _anonymous temporary_, or a _named local variable_.

A local variable (or stack-local allocation) holds a value directly, allocated within the stack's memory. The value is a part of the stack frame.

Local variables are immutable unless explicitly declared as mutable. 
Function parameters are immutable unless explicitly declared as mutable.
The `mut` keyword applies only to the following adjecent parameter.

For example: |mut x, y| and fn f(mut x: Box<i32>, y: Box<i32>) declare one mutable variable x and one immutable variable y.

Methods that take either `self` or `Box<Self>` can optionally place them in a mutable variable by prefixing them with `mut` (similar to regular arguments). For example:

```rust
trait Changer: Sized {
    fn change(mut self) {}
    fn modify(mut self: Box<Self>) {}
}
```


Local variables are not initialized when allocated. Instead, the entire frame worth of local variables are allocated, on frame-entry, in an uninitialized state. Subsequent statements within a function may or may not initialize the local variables. Local variables can be used only after they have been initialized; this is enforced by the compiler.


