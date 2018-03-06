# Classification of types

<!-- TOC -->

- [Nominal vs structural](#nominal-vs-structural)
- [Primitives vs library types](#primitives-vs-library-types)
- [Scalar types](#scalar-types)
- [Compound types](#compound-types)
- [Generic vs concrete](#generic-vs-concrete)
- [Copy vs Move types](#copy-vs-move-types)
- [Sized vs Unsized](#sized-vs-unsized)
- [Dependent](#dependent)
- [Dynamically-sized types](#dynamically-sized-types)
- [Pointer types](#pointer-types)
- [Algebraic types](#algebraic-types)
- [Functions-related types](#functions-related-types)
- [Data structures](#data-structures)

<!-- /TOC -->

## Nominal vs structural
A type is nominal when it has a formal type name that is used in type annotations (e.g. `bool`, `u8`, `char`). Nominal types often also have additional, colloquial, names (e.g. boolean, character, string slice).

A type is structural if it doesn't have a formal type name, but is identifiable by its structure. Naturally, structural types always have colloquial names, so we can talk about array, tuple, references, raw pointers, function pointer, slice, unit, and never type.


## Primitives vs library types
Fundamental type category in any language are the primitives. Primitive types are the basic building blocks of a language. In Rust, they are implemented by the compiler, while the Standard Library (std) implements method on them. Some primitives have an own module in std with additional resources.

For example, the type `f64` itself is implemented by the compiler, methods on `f64` are implemented by std (`is_nan`, `is_infinite`, etc.) and documented in the std's "primitives" section; but std also contains a related module, `std::f64`, with items such as constants `MAX_EXP`, `EPSILON`, `INFINITY` and math constants like `PI`, `E`, `SQRT_2`, etc.

Nominal primitives also have distinguishing naming style in comparison to other types - a single lowercased word, as opposed to CamelCase naming style of non-primitives. Rust's primitives are numbers, boolean, character, reference, raw pointer, function pointer, slice, string slice, array, tuple, unit, and never type.

All primitive (or built-in) types are tightly integrated into the language, in nontrivial ways that are not possible to emulate in user-defined types. Unlike them, non-primitive (or library) types are coded after the fact. They are no different than any other user-defined type, except they are standardized and made available as the Rust Standard Library (but Rust can be used without it).


## Scalar types
Scalars are the most basic primitives, they cannot be divided further and they don't depend on other types. They are atomic units, numeric in nature: integers and floats, but also boolean and character, which are merely a different interpretation of integers. Operations on scalar primitive types are the fastest language constructs there are. Integer addition, for example, can be performed as a single machine instruction.


## Compound types
Compound (or aggregate) types are build from heterogeneous or homogeneous grouping of other types. Array is a fixed contiguous sequence of values of the same type, the same as vector, except vector is growable and therefore stored on the heap. Tuples can group heterogeneous types. They have a fixed size like arrays, and their size is a part of their type; although, unlike arrays', their size is implicit.


## Generic vs concrete
Concrete types have a known, determined set of available values; e.g. the cardinality of boolean set is two, because a boolean can only have 2 distinct values.

On the other hand, generic types behave like type constructors: they take some input type and produce a new type. They do this by declaring a type parameter that acts as a placeholder for a concrete type, which is to be specified later. This allows them to have a single definition, but to work with different concrete types. For example, a vector, `Vec<T>`, has one type parameter, `T`, but when constructing a new vector, `let vec: Vec<u8> = vec(1, 2, 3);`, the concrete type `u8` is used in its place.

Rust employs monomorphism, which means the compiler will produce, for this example, just a single, concrete vector that works exclusively with `u8` values. Had there been another vector with elements of `char` type, there would've been two concrete vectors produced after compilation. Generic type is like a template used during coding-time that will produce concrete type(s) during compile-time.

Among the primitives, pointer types (refs, fn pointers, raw pointers, slices), array and tuple are generic. Non-primitives are almost exclusively generic (except `String`).


## Copy vs Move types
Pointer values are split between the stack and the heap: the pointer itself is on the stack, while the data it refers to is located on the heap (resource). Such a type must manage its own resources; when it itself goes out of scope, it must free its resources, which is done by implementing the `Drop` trait. 

They are solely responsible for their resources: when these types are instantiated, only a single variable can bind to a value of this, resource-managing, type at the same time - aliasing is disallowed. If such value is assigned to a different variable, the "move" semantics is employed: the value changes its owner, but the ex owner (original variable) cannot be used anymore.

Types whose values can be duplicated simply by copying bits have copy semantics. They are marked as such by implementing the `Copy` trait. Other types implementing the `Drop` trait have move semantics.

Array and tuple are Copy types only if their elements are Copy types. Shared reference, raw pointer and function pointer are also Copy, but their mutable variants are not (shared and mutable cannot coexist), and neither are non-primitive types.



## Sized vs Unsized

Smart pointers
pointer types

Getting size:

```rust
let s = String::from("love: ❤️");
assert_eq!(12, s.len() * std::mem::size_of::<u8>());
let v: Vec<char> = s.chars().collect();
assert_eq!(32, v.len() * std::mem::size_of::<char>());
```



## Dependent
Functions, closures

## Dynamically-sized types
- slice
- string slice
- trait object

## Pointer types
- references
- slice
- string slice
- function pointers
- raw pointers
- smart pointers

## Algebraic types
sum (enum) and product (struct) types

## Functions-related types
- function item type
- function pointer
- closure type

## Data structures
- Sequences: `Vec`, `VecDeque`, `LinkedList`
- Maps: `HashMap`, `BTreeMap`
- Sets: `HashSet`, `BTreeSet`
- other: `BinaryHeap`

# Smart pointers
- `String`
- `Vec<T>`
- `Box<T>`
- `Rc<T>`
- `Arc<T>`
- `RefCell<T>`
- smart pointer is a reference sort of type, but unlike primitive reference type it owns its data.
- smart pointer is implemented as struct; it is a fat pointer on the stack, that points to some data on the heap.
- all smart pointers impl `Deref` and `Drop` traits

The distinguishing characteristics of smart pointers from other structs, is that they implement `Deref` trait, which makes accessing the data behind the pointer very convenient; in fact, the rules regarding `Deref` and `DerefMut` were designed specifically to accommodate smart pointers.

They also implement `Drop` trait, so when they go out of scope, their value on the heap is cleaned up as well.
