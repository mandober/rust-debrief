# Initialization

- Initialization
- Re-initialization
- Deinitialization


## Initialization
A variable is initialized if it has been assigned a value and hasn't since been moved from. All other memory locations are assumed to be initialized. Only unsafe Rust can create such a memory without initializing it.

## Type Size
The size of a value has two definitions.
The first is that it is how much memory must be allocated to store that value.
The second is that it is the offset in bytes between successive elements in an array with that item type.
It is a multiple of the alignment, including zero. The size can change depending on compiler version (as new optimizations are made) and target platform (similar to how usize varies per-platform).
https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment

