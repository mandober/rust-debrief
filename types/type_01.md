# Type descriptions

- [Unit](#unit)
- [Booleans](#booleans)
- [Integers](#integers)
- [Address Integers](#address-integers)
- [Floats](#floats)
- [Characters](#characters)
- [Tuples](#tuples)
- [Function pointers](#function-pointers)
- [References](#references)
- [Raw Pointers](#raw-pointers)
- [String slices](#string-slices)
- [Strings](#strings)
- [Arrays](#arrays)
- [Slices](#slices)
- [Vectors](#vectors)
- [Contiguous sequence](#contiguous-sequence)
- [Box](#box)


## Unit
  name: unit type  
  desc: empty tuple  
  type: `()`  
  cardinality: 1
  value: `()`  
  literal: ✔  
  std::module: ✗  
  sample: `fn empty(is: bool) -> ()`  

## Booleans
  name: boolean  
  type group: numeric  
  type: `bool`  
  cardinality: 2
  values: `true`, `false`
  literals: ✔  
  ref: `&bool` (&true)
  mut ref: `&mut bool` (&mut true)
  kind: primitive, scalar, concrete, fixed  
  sized: ✔    
  size: 1b  
  storage: stack (`Copy` type)  
  std::module: ✗  
  sample: `let b: bool = true`  
  traits: `Copy`, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign  
  Not, Debug, Display, Default, Hash, Eq, PartialEq, Ord, PartialOrd, FromStr  

## Integers
  name: integers  
  class: numeric  
  types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
  type variants: 10 (5 signed + 5 unsigned)
  val variants: i8(256), i16(65536), i32(131072), etc.
  literals: yes
  ref: `&i8`  
  mut ref: `&mut i8`  
  kind: primitive, scalar, concrete, fixed, numeric  
  sized: yes  
  size: 8b-64b  
  storage: stack  
  std: primitive, module  
  sample: `let n: i32 = 42`  
  traits: `Copy`  
  all traits: `Debug`, `Display`  

## Address Integers
  name: machine architecture-dependent integers  
  desc: address-wide (pointer sized) integers  
  class: numeric  
  type: `isize`, `usize`  
  ref: `&usize`  
  mut ref: `&mut usize`  
  kind: primitive, concrete, fixed, numeric  
  sized: yes  
  size: 64bits@x64 (32b@x86)  
  storage: stack  
  std: primitive types, module  
  sample: `let len: usize = 42`  
  traits: `Copy`  
  all traits: `Debug`, `Display`  

## Floats
  name: floats
  desc: IEEE 754-2008 binary32 and binary64  
  type group: numeric  
  class: numeric  
  type: `f32`, `f64`
  ref: `&f32`, `&f64`
  mut ref: `&mut f32`, `&mut f64`
  kind: primitive, concrete, fixed
  sized: yes
  size: 32b-64b
  storage: stack
  std: primitive, module
  sample: `let f: f32 = 3.142`
  traits: `Copy`
  all traits: `Debug`, `Display`

## Character
  name: character  
  desc: u32 word, range: 0x0000-0xD7FF, 0xE000-0x10FFFF
  type group: numeric  
  type: `char`
  ref: `&char`
  mut ref: `&mut char`
  kind: primitive, concrete, fixed
  sized: yes
  size: 4b
  storage: stack
  std: primitive, module
  sample: `let c: char = 'ß'`
  traits: `Copy`
  all traits: `Debug`, `Display`

## Tuples
  name: tuple
  type: `(T, U,...)`
  ref: `&(T, U,...)`
  ref mut: `&mut (T, U,...)`
  kind: primitive, generic, fixed, sequence
  sequence: heterogeneous
  sized: yes
  size: size of `T` * n
  storage: stack
  std: 
  sample: `let t: (u8, char) = (42, 'ß')`
  traits: `Copy`
  all traits:

## Function pointers
  name: Function pointer. 
  type: `fn(T) -> T`, more generally: `[extern] ["Language"] [unsafe] fn(T) -> T`
  Function pointers vary by signature, ABI, or safety.
  type eg: `fn(usize) -> bool`, `extern "C" fn(u8) -> u8`
  ref: 
  ref mut: 
  kind: primitive, generic, fixed
  sized: 
  size: 
  storage: 
  module: 
  sample: `let fp: fn(usize) -> usize = |x| x + 5`
  traits: `Copy`
  all traits:

## References
  name: reference
  type:
  ref: `&T`, `ref T`
  ref mut: `&mut T`, `ref mut T`
  kind: primitive, generic
  sized: 
  size: 
  storage: stack
  std: primitive, module
  sample: `let r: &u8 = &255`, `let ref r: u8 = 255`
  traits ref: `Copy` `Clone` (it will not defer to `T`'s `Clone` impl if it exists)
  `Deref`, `Borrow`, `Pointer`
  traits mut ref: `Deref`, `DerefMut`, `Borrow`, `BorrowMut`, `Pointer`

## Raw Pointers
  name: 
  type: 
  ref: 
  ref mut: 
  kind: primitive, generic, fixed, sequence
  sized: 
  size: 
  storage: 
  std: primitive, module
  sample: 
  traits: `Copy`
  all traits:
  immutable: `*const T`
  mutable: `*mut T`

## String slices
  name: string slices
  type: `&str`
  ref: (`&&str`)
  ref mut: 
  kind: primitive, generic, fixed, sequence
  sized: 
  size: 
  storage: 
  std: primitive, module
  sample: 
  traits: `Copy`
  all traits:
  `&str` string slices. `let ss: &str = &string[0..4]`

## Strings
  name: 
  type: 
  ref: 
  ref mut: 
  kind: primitive, generic, fixed, sequence
  group: literal string (fixed), String (growable), string slice (view into)
  elements: homogeneric
  sized: 
  size: 
  storage: 
  std: module
  sample: 
  traits: `Copy`
  all traits:

## Arrays
  name: array
  type: `[T; N]` N is size
  ref: `&[T; N]`
  mut ref: `&mut [T; N]`
  kind: primitive, generic, fixed, sequence
  elements: homogeneric
  sized: yes
  size: size of `T` * n
  storage: stack
  std: primitive
  sample: `let arr: [i32; 32] = [1; 32]`
  traits: `Copy`
  all traits: only for arrays of up to 32 elements, if element type allows it: 
  `Debug`, `Default`, `Clone` (only if `T: Copy`)
  `IntoIterator` (implemented for `&[T; N]` and `&mut [T; N]`)
  `PartialEq`, `PartialOrd`, `Eq`, `Ord`
  `Hash`, `AsRef`, `AsMut`, `Borrow`, `BorrowMut`

## Slices
  name: slice
  type: A view into a contiguous sequence `[T]`
  ref: `&[T]`
  ref mut: `&mut [T]`
  kind: primitive, generic, fixed, sequence, read-only
  sized: 
  size: 
  storage: 
  std: primitive, module 
  sample: `let slice = &vector[1..4]`
  traits: `Copy`
  all traits:

## Vectors
  name: vector
  type: `Vec<T>`
  ref: `&Vec<T>`
  ref mut: `&mut Vec<T>`
  kind: compound, generic, growable, sequence, null-pointer-optimized
  group: array (fixed), vector (growable), slice (view into seq)
  elements: homogeneric
  sized: no
  size: 
  storage: pointer, capacity, length triplet
  std: module, struct
  sample: `let v: Vec<bool> = [true, true, false]`
  traits: `Copy`, `Index`
  all traits:

## Contiguous sequence
  *Not a type per se. It represents a contiguous sequence like an array or vector.*
  name: contiguous sequence
  type: `[T]`
  ref: a slice `&[T]`
  ref mut: a mut slice `&mut [T]`
  kind: compound, generic, sequence
  group: array (fixed), vector (growable), slice (view into seq)

## Box
  name: box
  anot: `Box<T>`
  type: composite type, generic type, reference types group
  mod: `std::boxed`
  sized: yes
  store: ptr on stack to data on heap
  code: `let b = Box::new(5)`
