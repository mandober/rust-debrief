# Container types

Box
Rc, Arc
Cell
RefCell
Mutex
RwLock

Box is a container type designed to allocate and "hold" an object on the heap. It's the most simple form of allocation on the heap and the content is dropped when it goes out of scope.

Rc (short for Reference Counting) is used when we want multiple methods using a read only reference thus providing with shared ownership over some content. It counts the uses of the reference pointing to the same piece of data on the heap. This ensures that when the last reference is dropped, the data itself will be dropped and the memory properly freed.
Arc is a thread-safe Rc.
