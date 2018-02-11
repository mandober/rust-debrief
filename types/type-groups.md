# Type groups

Rust's types can be grouped by multitude of criteria.

## Primitives

The most general partitioning is primitives and non-primitives.

Primitive types are basic building blocks of the language and they are implemented in the compiler. The compiler has intimate knowledge about each primitive type. Standard library implements methods on primitives.

Primitive types have several overlapping groupings:
- scalar types
- compound types
- dynamically-sized types
- pointer types

**Scalars** are the most basic types of numeric nature:
- booleans
- characters
- _Numbers_:
  - floats
  - integers:
    - independent integers
    - machine-dependent integers

**Compounds**:
- array: homogeneous
- tuple: heterogeneous

**Dynamically-sized** types:
- slice
- string slice

**Pointer** types:
- references
- slice
- string slice
- function pointers
- raw pointers


## Non-primitives

Non-primitive types are provided by standard library, along with their methods. They are grouped by module in the standard library.

- Compound: vector
- Pointer types:   
  Related to primitive pointer types, there are numerous types of this kind, collectively known as __smart pointers__.
- Algebraic types: sum (enum) and product (struct) types
  - Option
  - Result
- Collections:
  - Sequences: `Vec`, `VecDeque`, `LinkedList`
  - Maps: `HashMap`, `BTreeMap`
  - Sets: `HashSet`, `BTreeSet`
  - other: `BinaryHeap`


## All types

- Scalars: booleans, characters, numbers
- Compounds: array, tuple, vector, slice, linked list
- Algebraic: result, option
- Dynamically sized:
  - slice
  - string slice
  - trait object
- Pointer types:
  - primitive pointer types:
    - references
    - slice
    - string slice
    - function pointers
    - raw pointers
  - smart pointers:
   - String
   - Vec
   - Box
   - Rc, Weak
   - Cell, Ref, RefMut, RefCell
- Functions-related types
  - function item type
  - function pointer
  - closure type



## All types: by module

Boolean, array and tuple don't have a std module.

With eponymous module:
- `i8`        The 8-bit signed integer type.
- `u8`        The 8-bit unsigned integer type.
- `i16`       The 16-bit signed integer type.
- `u16`       The 16-bit unsigned integer type.
- `i32`       The 32-bit signed integer type.
- `u32`       The 32-bit unsigned integer type.
- `i64`       The 64-bit signed integer type.
- `u64`       The 64-bit unsigned integer type.
- `i128`      The 128-bit signed integer type. [LAB]
- `u128`      The 128-bit unsigned integer type. [LAB]
- `isize`     The pointer-sized signed integer type.
- `usize`     The pointer-sized unsigned integer type.
- `f32`       The 32-bit floating point type.
- `f64`       The 64-bit floating point type.
- `char`      A character type.
- `slice`     A dynamically-sized view into a contiguous sequence.
- `str`       String slices.
- `option` Option enum
- `result` Result enum
- `string` String
- `vec`    Vector

With similarly named module:
- `borrow`    references
- `ptr`       raw pointers
- `fn`        function pointers
- `boxed`     Box
- `cell`      `Cell<T>`, `Ref<T>`, `RefMut<T>`, `RefCell<T>`
- `rc`        `Rc<T>`, `Weak<T>`
