# Box

- module: `std::boxed`, since 1.0.0
- module [doc](https://doc.rust-lang.org/std/boxed "external link:std docs")
- the main feature of this module is [Box struct](box.md).
- `boxed` module contains
    - Structs:
    - `Box`
    - `ExchangeHeapSingleton`[LAB]
    - `IntermediateBox`[LAB]
    - Constants:
    - `HEAP` [LAB]
    - Traits:
    - `FnBox` [LAB]


### Structs
- `Box` a pointer type for heap allocation.
- `ExchangeHeapSingleton` [LAB] the singleton type used for `boxed::HEAP`.
- `IntermediateBox` - [LAB] uninitialized backing storage for Box.

### Constants
- `HEAP` [LAB] a value that represents the heap. This is the default place that the box keyword allocates into when no place is supplied.

### Traits
- `FnBox` [LAB] a version of the `FnOnce` intended for use with boxed closure objects. `Box<FnBox()>` is to be used instead of storing `Box<FnOnce()>` in a data structure. The two traits behave essentially the same, except that a `FnBox` closure can only be called if it is boxed. Note that `FnBox` may be deprecated in the future if `Box<FnOnce()>` closures become directly usable.


The main feature of this module is [Box struct](box.md).