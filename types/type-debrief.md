# Types debrief

<!-- TOC -->

- [Never type](#never-type)
- [Unit type](#unit-type)
- [Boolean](#boolean)
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

<!-- /TOC -->


## Never type
- desc: never type
- annotation: `!`
- value: no value
- module: no std module
- sample: `fn non_returning() -> !`

## Unit type
- desc: one value - no information, empty tuple
- annotation: `()`
- value: `()`
- module: no std module
- sample: `Result<(), E>`

## Boolean
- desc: boolean type for true and false boolean values
- annotation: `bool`   e.g. `let t: bool = true`
- values: true, false
- module: no std module
- ref: `&bool`         e.g. `let t: &bool = &true`
- mut ref: `&mut bool` e.g. `let t: &mut bool = &mut true`
- groups: primitive, scalar, numeric, sized, concrete
- size: sized, 1b
- traits: Copy, Not, Debug, Display, Default, Hash, Eq, PartialEq, Ord, FromStr, PartialOrd, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXorAssign, BitXor

## Integers
- annotation: `i8`, `u8`, `i64`, `u64`, etc. Or as suffix.
- type variants: 10 (5 signed + 5 unsigned)
- ref: `&u8`
- mut ref: `&mut i64`
- size: sized, 1-12 Bytes (8-128 bits)
- modules: `std::i8`, `std::i16`, etc.
- sample: `let n: i32 = 42`
- groups: primitive, scalar, numeric, sized, concrete
- character `_` has no meaning with numbers, useful for visual splitting
- traits: Copy, Debug, Display, etc.

## Address Integers
- annotation: `isize`, `usize`. Or as suffix.
- desc: machine architecture-dependent, pointer sized integers
- ref: `&usize`
- mut ref: `&mut usize`
- size: sized, 64bits@x64 (32b@x86)
- note: used for sizes, lengths, etc.
- groups: primitive, scalar, numeric, sized, concrete
- character `_` has no meaning with numbers, useful for visual splitting

## Floats
- annotation: `f32`, `f64`
- desc: IEEE 754-2008 binary32 and binary64
- ref: `&f32`, `&f64`
- mut ref: `&mut f32`, `&mut f64`
- size: sized, 4-8 Bytes (32-64 bits)
- groups: primitive, scalar, numeric, sized, concrete
- sample: `let f: f32 = 3.142`

## Characters
- annotation: `char`
- desc: u32 word, range: 0x0000-0xD7FF, 0xE000-0x10FFFF
- type group: numeric  
- ref: `&char`
- mut ref: `&mut char`
- size: 1 Byte (4 bits)
- groups: primitive, scalar, numeric, sized, concrete
- module: no
- sample: `let c: char = 'ß'`

## Tuples
- annotation: `(T, U,...)`
- desc: tuple, heterogeneous sequence
- ref: `&(T, U,...)`
- ref mut: `&mut (T, U,...)`
- groups: primitive, generic, fixed, sequence, compound
- size: size of `T` * n
- module: no
- sample: `let t: (u8, char) = (42, 'ß')`

## Function pointers
- annotation: `fn(T) -> T`
- more generally: `[extern] ["Language"] [unsafe] fn(T) -> T`
- function pointers vary by signature, ABI, or safety.
- e.g. `fn(usize) -> bool`, `extern "C" fn(u8) -> u8`
- sample: `let fp: fn(usize) -> usize = |x| x + 5`

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
  subtypes: `T × N`
  storage: stack
  std: primitive
  sample: `let arr: [i32; 32] = [1; 32]`
  traits: `Copy` (if elements are Copy)
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
