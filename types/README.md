# Rust Data Types


## Primitive types
* Scalar types
  - Booleans
  - Numbers
    * Integers: signed and unsigned
    * Machine-architecture dependent integers
    * Floating-point numbers
  - Characters
  * Compound types
    - Tuples
    - Arrays
  * Unit type (empty tuple)
  * Dynamically-sized types
    - Slices
    - String slices
  * Pointer types
    - References
    - Raw pointers
    - Function pointers

## Composite types
- struct (product type)
  * named-fields struct
  * tuple struct
  * unit-like struct
- enum
- union
- smart pointers
  * Box
  * Rc
  * Arc
- function item types
- closure types
- trait objects (dynamically sized type)
- Collections
- Vector
- String
- HashMap
- Set


**Move vs Copy types**
Most primitive values are Copy types.
Primitive data types kept on the stack (that implement the `Copy` trait)
integers, floats, characters, booleans, references.
- heap vs stack values. stored in final binary. allocated.
- boxing values onto the heap
