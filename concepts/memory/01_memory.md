# Rust and Memory

- memory management: manual, gc
- memory sections: static, stack, heap
- memory objects: value, object
- memory agents: variables, pointers
- binding
- ownership of a value
- borrowing a value
- moving a value
- pointers: ref, mut ref, raw, box, rc

[The Periodic Table of Rust Types](http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/)

[Rust container cheat sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p)

Trait Object is a pointer to a type that implements a certain trait. It also has a virtual table to determine which type it actually referes to.

Box and Rc are pointers to objects on the heap. Box provides ownership, the Rc sharing via ref counting.

With Rc the ownership to the object living on the heap is shared. When the counter reaches 0, it drops the reference to the object, freeing the memory associated with it. That's why clone only hands-off a reference, there is no need to drop the object when it goes out of scope: we only drop it once all the references to it are gone. When calling clone, we don't need to know the size of the object we copy. The only thing we copy is the reference to the object living on the heap and we increment the counter by 1.

Box acts as the most simple form of heap allocation. When it goes out of scope, we drop the content. That's why we need to copy the whole content when calling clone, we effectively copy the content from method to method to keep it alive, transferring ownership from method to method as they go out of scope. To guarantee memory safety, we naturally need to know the size of the content we copy, thus the content must be Sized when we call the clone method.

In short: Box<T> copies values, Rc<T> clones references and keeps track of references in use.



## Managing Memory

Rust doesn't use garbage collector, prefering manual memory management assisted with RAII concept.

Manual memory management requires the programer to annotate the lifetimes.


The memory ocupied by a Rust program is split into two distinct areas: the heap and the sack. Simply put, the stack contains primitive variables, while the heap stores complex types; a heap can grow until the available memory is exhausted. The stack is faster, but may not grow without limits. Every binding in Rust is on the stack, but those bindings may refer to things in the heap, and elsewhere.

---

http://www.electronicdesign.com/blog/reflections-rust

In general, there is an owner of an object and references can be borrowed. An object cannot be released if there are borrowed references to it. All references have a lifetime, but they can often be inferred based on Rust language rules. There are also explicit lifetime parameters that can be used in various areas in the code, such as in function signatures.
