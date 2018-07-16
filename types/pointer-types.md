# Pointer types

Rust has various types of pointers:
- Raw pointers
  - Constant raw pointer
  - Mutable raw pointer
  - NonNull
- Shared reference
- Mutable reference
- Function pointer
- Box: pointer for heap allocation.
- Rc: reference-counted heap pointer
- Arc: atomic reference-counted heap pointer


## Raw pointers
- Raw pointers are exactly like C pointers. 
- They can be only derefrenced in unsafe mode.
- NonNull
  - `*mut T` 
  - must always be non-null, even if the pointer is never dereferenced
  - unlike `*mut T`, `NonNull<T>` is covariant over `T`
- Pinned reference, `PinMut`, is a lot like a mutable reference, except that it is not safe to move a value out of a pinned reference unless the type of that value implements the Unpin trait.


## Box
- box is a smart pointer designed to allocate an object on the heap.
- box provides unique ownership for this allocation; box owns its contents.
- the contents of the box are dropped when the box goes out of scope.
- it is the most simple form of allocation on the heap
- To get a pointer to a boxed value, dereference the box. This does not take ownership of the original allocation and requires no resource management later, but you must not use the pointer after its lifetime.
- `into_raw` function consumes a box and returns the raw pointer.
  it doesn't destroy `T` or deallocate any memory.


## Rc
- Rc is a smart pointer i.e. a type of box with reference counting
- Rc is designed to provide shared ownership to its contents
- used when shared ownership over some content is needed
  e.g. a node in a doubly-linked list is often (in safe mode) implemented as Rc
- Rc counts the uses of the reference pointing to the same piece of data on the heap.
- This ensures that when the last reference is dropped, the data itself will be dropped and the memory properly freed.
- Arc is a thread-safe version of Rc
