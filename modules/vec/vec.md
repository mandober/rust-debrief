# Vector

- contiguous growable sequance with heap-allocated contents
- std module: [`std::vec`](https://doc.rust-lang.org/std/vec/)
- actual module: [`alloc::vec`](https://doc.rust-lang.org/alloc/vec/)
- representation: fat pointer (ptr, len, cap) on stack with data on the heap
- deref: `Deref<Target = [T]>`
- annotation: `Vec<T>`
- type: generic, compound, library, move, owned, nominal
- O(1) indexing, amortized O(1) push (to the end) and O(1) pop (from the end)
- indexing through `Index` and `IndexMut` traits
- structs `std::vec`
  - `Drain` draining iterator for Vec<T>
  - `IntoIter` iterator that moves out of Vec<T>
  - `Splice` splicing iterator for Vec<T>
  - `Vec` contiguous growable array type, Vec<T>
  - `DrainFilter` [LAB] iterator produced by `drain_filter` on Vec<T>
  - `PlaceBack` [LAB] place for insertion at the back of Vec<T>

