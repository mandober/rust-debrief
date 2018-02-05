# Vector

- name: vector
- contiguous growable array type with heap-allocated contents
- annotation: `Vec<T>`
- generic, compound
- O(1) indexing, amortized O(1) push (to the end) and O(1) pop (from the end)
- std module: [`std::vec`](https://doc.rust-lang.org/std/vec/)
- actual module: [`alloc::vec`](https://doc.rust-lang.org/alloc/vec/)
- store: fat pointer on stack to data on heap
- deref: `Deref<Target = [T]>`
- indexing through `Index` and `IndexMut` traits
- structs `std::vec`
  - `Drain` draining iterator for Vec<T>
  - `IntoIter` iterator that moves out of Vec<T>
  - `Splice` splicing iterator for Vec<T>
  - `Vec` contiguous growable array type, Vec<T>
  - `DrainFilter` [LAB] iterator produced by `drain_filter` on Vec<T>
  - `PlaceBack` [LAB] place for insertion at the back of Vec<T>

