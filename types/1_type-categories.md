# Type categories

<!-- TOC -->

- [Nominal vs structural](#nominal-vs-structural)
- [Primitive types](#primitive-types)
- [Scalar types](#scalar-types)
- [Compound types](#compound-types)
- [Generic vs concrete](#generic-vs-concrete)
- [Copy vs Move types](#copy-vs-move-types)
- [sized vs unsized](#sized-vs-unsized)
- [Dependent](#dependent)
- [Dynamically-sized types](#dynamically-sized-types)
- [Pointer types](#pointer-types)
- [Algebraic types](#algebraic-types)
- [Functions-related types](#functions-related-types)
- [Data structures](#data-structures)

<!-- /TOC -->

Rust's types can be classified in many categories by different criteria.

## Nominal vs structural
Types are either nominal, meaning they have both, a proper type name and a  colloquial name, with latter being used in type annotations (e.g. `bool`, `u8`, `char`), or they are structural, without a proper type name, whose type is determined from structural elements. Naturally, structural types have colloquial names, so we can talk about array, tuple, references, raw pointers, slice, unit, and never type. All structural types are primitives.

## Primitive types
The basic type category in any language is primitives, the types that make the basic building blocks of the language. In Rust, they are implemented by the compiler and they are stored on the stack. They have distinguishing naming style in comparison to other types: a single lowercased word as opposed to CamelCase naming style of non-primitives. The Rust's Core library, `libcore` in `core` crate, implements methods on primitives, and they are also re-exported (and accessible) through `std`. Rust's primitives: numbers, boolean, character, reference, raw pointer, function pointer, slice, string slice, array, tuple, unit, and never type.

## Scalar types
Scalars are the most basic primitives, they cannot be divided further and they don't depend on other types. They are atomic units: numbers, booleans and characters. Operations on scalar primitive types are the fastest language constructs there are. Integer addition, for example, can be performed as a single machine instruction.

## Compound types
Compound (or aggregate) types are build from heterogeneous or homogeneous grouping of other types. Array is a fixed contiguous sequence of values of the same type, the same as vector, except vector is growable and therefore stored on the heap. Tuples are used to group heterogeneous types. They have a fixed size, and like arrays, their size is a part of their type (although with tuples this is implicit).

## Generic vs concrete
Unlike concrete types (e.g. boolean, character), generic types have a type parameter, which will be provided when needed (when used). For example, a vector, `Vec<T>`, has a type parameter, `T`, that is a placeholder for any concrete type, which will be specified when a vector is constructed. Among the primitives, array, tuple, pointer types are generic; Majority of non-primitives are generic. Generic types are like type constructors: they take some inputs and produce a new type. For example, `&` operator takes a value of some type and a lifetime and produces a sharable reference.

## Copy vs Move types
Types that implement Copy trait have copy semantics, other types have move semantics. Copy types are scalars, sharable reference, 


## sized vs unsized

Smart pointers
pointer types


## Dependent
- vector
- array
- pointer types (reference, raw pointer, function pointer, slice, string slice)




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
