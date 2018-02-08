# Smart pointers

- smart pointer is a reference sort of type, but unlike primitive reference type it owns its data.
- smart pointer is implemented as struct; it is a fat pointer on the stack, that points to some data on the heap.
- all smart pointers impl `Deref` and `Drop` traits


The distinguishing characteristics of smart pointers from other structs, is that they implement `Deref` trait, which makes accessing the data behind the pointer very convenient; in fact, the rules regarding `Deref` and `DerefMut` were designed specifically to accommodate smart pointers.

They also implement `Drop` trait, so when they go out of scope, their value on the heap is cleaned up as well.


- `String`
- `Vec<T>`
- `Box<T>`
- `Rc<T>`
- `Arc<T>`
- `RefCell<T>`