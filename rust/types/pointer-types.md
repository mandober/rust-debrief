# Pointer types

Rust has various types of pointers:
- Raw pointers
  - Constant raw pointer
  - Mutable raw pointer
  - NonNull
- References
  - Shared reference
  - Mutable reference
- Function pointer
- Slices
  - slice
  - string slice
- Smart pointers
  - Box: pointer for heap allocation.
  - Rc: reference-counted heap pointer
  - Arc: atomic reference-counted heap pointer
- Trait object



## Raw pointers
- Raw pointers are exactly like C pointers. 
- They can be only derefrenced in unsafe mode.
- NonNull
  - `*mut T`
  - must always be non-null, even if the pointer is never dereferenced
  - unlike `*mut T`, `NonNull<T>` is covariant over `T`
- Pinned reference, `PinMut`, is a lot like a mutable reference, except that it is not safe to move a value out of a pinned reference unless the type of that value implements the Unpin trait.


## Smart pointers

Box and Rc are pointers to objects on the heap. 
Box provides ownership, the Rc sharing via ref counting.

With Rc the ownership to the object living on the heap is shared. When the counter reaches 0, it drops the reference to the object, freeing the memory associated with it. That's why clone only hands-off a reference, there is no need to drop the object when it goes out of scope: we only drop it once all the references to it are gone. When calling clone, we don't need to know the size of the object we copy. The only thing we copy is the reference to the object living on the heap and we increment the counter by 1.

Box acts as the most simple form of heap allocation. When it goes out of scope, we drop the content. That's why we need to copy the whole content when calling clone, we effectively copy the content from method to method to keep it alive, transferring ownership from method to method as they go out of scope. To guarantee memory safety, we naturally need to know the size of the content we copy, thus the content must be Sized when we call the clone method.

In short: Box<T> copies values, Rc<T> clones references and keeps track of references in use.


### Box
- box is a smart pointer designed to allocate an object on the heap.
- box provides unique ownership for this allocation; box owns its contents.
- the contents of the box are dropped when the box goes out of scope.
- it is the most simple form of allocation on the heap
- To get a pointer to a boxed value, dereference the box. This does not take ownership of the original allocation and requires no resource management later, but you must not use the pointer after its lifetime.
- `into_raw` function consumes a box and returns the raw pointer.
  it doesn't destroy `T` or deallocate any memory.



### Rc
- Rc is a smart pointer i.e. a type of box with reference counting
- Rc is designed to provide shared ownership to its contents
- used when shared ownership over some content is needed
  e.g. a node in a doubly-linked list is often (in safe mode) implemented as Rc
- Rc counts the uses of the reference pointing to the same piece of data on the heap.
- This ensures that when the last reference is dropped, the data itself will be dropped and the memory properly freed.
- Arc is a thread-safe version of Rc


## Trait Object
Trait Object is a pointer to a type that implements a certain trait. It also has a virtual table to determine which type it actually referes to.
