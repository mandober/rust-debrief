# Data Types

Every variable, item and value in a Rust program has a type.
The type of a value defines the interpretation of the memory holding it.

Built-in types and type-constructors are tightly integrated into the language, 
in nontrivial ways that are not possible to emulate in user-defined types. 
User-defined types have limited capabilities.


## Primitive types
- the boolean type, `bool`
- machine types: integer and floating-point.
- machine-dependent integer types: `isize`, `usize`
- Arrays
- Tuples
- Slices: slice and string slice
- Reference types: reference, function pointer, raw pointer


# Types


## Machine types
The machine types are the following:

The unsigned word types u8, u16, u32 and u64, with values drawn from the integer intervals [0, 2^8 - 1], [0, 2^16 - 1], [0, 2^32 - 1] and [0, 2^64 - 1] respectively.

The signed two's complement word types i8, i16, i32 and i64, with values drawn from the integer intervals [-(2^(7)), 2^7 - 1], [-(2^(15)), 2^15 - 1], [-(2^(31)), 2^31 - 1], [-(2^(63)), 2^63 - 1] respectively.

The IEEE 754-2008 binary32 and binary64 floating-point types: f32 and f64, respectively.

## Machine-dependent integer types
The usize type is an unsigned integer type with the same number of bits as the platform's pointer type. It can represent every memory address in the process.

The isize type is a signed integer type with the same number of bits as the platform's pointer type. The theoretical upper bound on object and array size is the maximum isize value. This ensures that isize can be used to calculate differences between pointers into an object or array and can address every byte within an object along with one byte past the end.


## Tuple types
A tuple type is a heterogeneous product of other types, called the elements of the tuple. It has no nominal name and is instead structurally typed.

Tuple types and values are denoted by listing the types or values of their elements, respectively, in a parenthesized, comma-separated list.

Because tuple elements don't have a name, they can only be accessed by pattern-matching or by using N directly as a field to access the Nth element.

For historical reasons and convenience, the tuple type with no elements (()) is often called ‘unit’ or ‘the unit type’.

## Array and slice types
Rust has two different types for a list of items:

[T; N], an 'array'
[T], a 'slice'
An array has a fixed size, and can be allocated on either the stack or the heap.

A slice is a dynamically sized type representing a 'view' into an array. To use a slice type it generally has to be used behind a pointer for example as

&[T], a 'shared slice', often just called a 'slice', it doesn't own the data it points to, it borrows it.
&mut [T], a 'mutable slice', mutably borrows the data it points to.
Box<[T]>, a 'boxed slice'
Examples:


// A stack-allocated array
let array: [i32; 3] = [1, 2, 3];

// A heap-allocated array, coerced to a slice
let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);

// A (shared) slice into an array
let slice: &[i32] = &boxed_array[..];
All elements of arrays and slices are always initialized, and access to an array or slice is always bounds-checked in safe methods and operators.

The Vec<T> standard library type provides a heap allocated resizable array type.


## Struct types

A struct type is a heterogeneous product of other types, called the fields of the type.1

New instances of a struct can be constructed with a struct expression.

The memory layout of a struct is undefined by default to allow for compiler optimizations like field reordering, but it can be fixed with the #[repr(...)] attribute. In either case, fields may be given in any order in a corresponding struct expression; the resulting struct value will always have the same memory layout.

The fields of a struct may be qualified by visibility modifiers, to allow access to data in a struct outside a module.

A tuple struct type is just like a struct type, except that the fields are anonymous.

A unit-like struct type is like a struct type, except that it has no fields. The one value constructed by the associated struct expression is the only value that inhabits such a type.

1
struct types are analogous to struct types in C, the record types of the ML family, or the struct types of the Lisp family.

## Enumerated types

An enumerated type is a nominal, heterogeneous disjoint union type, denoted by the name of an enum item. 2

An enum item declares both the type and a number of variants, each of which is independently named and has the syntax of a struct, tuple struct or unit-like struct.

New instances of an enum can be constructed in an enumeration variant expression.

Any enum value consumes as much memory as the largest variant for its corresponding enum type, as well as the size needed to store a discriminant.

Enum types cannot be denoted structurally as types, but must be denoted by named reference to an enum item.

2
The enum type is analogous to a data constructor declaration in ML, or a pick ADT in Limbo.

## Union types

A union type is a nominal, heterogeneous C-like union, denoted by the name of a union item.

A union contains the value of any one of its fields. Since the accessing the wrong field can cause unexpected or undefined behaviour, unsafe is required to read from a union field or to write to a field that doesn't implement Copy.

The memory layout of a union is undefined by default, but the #[repr(...)] attribute can be used to fix a layout.

## Recursive types

Nominal types — structs, enumerations and unions — may be recursive. That is, each enum variant or struct or union field may refer, directly or indirectly, to the enclosing enum or struct type itself. Such recursion has restrictions:

Recursive types must include a nominal type in the recursion (not mere type definitions, or other structural types such as arrays or tuples). So type Rec = &'static [Rec] is not allowed.
The size of a recursive type must be finite; in other words the recursive fields of the type must be pointer types.
Recursive type definitions can cross module boundaries, but not module visibility boundaries, or crate boundaries (in order to simplify the module system and type checker).
An example of a recursive type and its use:


enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

let a: List<i32> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));
Pointer types

All pointers in Rust are explicit first-class values. They can be moved or copied, stored into data structs, and returned from functions.

## Shared references (&)

These point to memory owned by some other value. When a shared reference to a value is created it prevents direct mutation of the value. Interior mutability provides an exception for this in certain circumstances. As the name suggests, any number of shared references to a value may exit. A shared reference type is written &type, or &'a type when you need to specify an explicit lifetime. Copying a reference is a "shallow" operation: it involves only copying the pointer itself, that is, pointers are Copy. Releasing a reference has no effect on the value it points to, but referencing of a temporary value will keep it alive during the scope of the reference itself.

## Mutable references (&mut)

These also point to memory owned by some other value. A mutable reference type is written &mut type or &'a mut type. A mutable reference (that hasn't been borrowed) is the only way to access the value it points to, so is not Copy.


### Textual types
the types `char` and `str` hold textual data.

A value of type `char` is a Unicode scalar value
(i.e. a code point that is not a surrogate),
represented as a 32-bit unsigned word in the
`0x0000 to 0xD7FF` or `0xE000 to 0x10FFFF` range.
An array of chars is effectively an `UCS-4/UTF-32` string.

A value of type `str` is a Unicode string, represented as an array of 8-bit 
unsigned bytes holding a sequence of UTF-8 code points.
Since `str` is of unknown size, it is *not a first-class type*, but can only be 
instantiated through a reference type, such as `&str`


## Tuple types: tuple and unit type

## Tuple
A tuple type is a heterogeneous product of other types, called the elements of the tuple. 
It has no nominal name and is instead structurally typed.

Tuple types and values are denoted by listing the types or values of their elements, 
respectively, in a parenthesized, comma-separated list.

Because tuple elements don't have a name, they can only be accessed by pattern-matching 
or by using `N` directly as a field to access the `N`th element.

An example of a tuple type and its use:
```rust
type Pair<'a> = (i32, &'a str);
let p: Pair<'static> = (10, "ten");
let (a, b) = p;

assert_eq!(a, 10);
assert_eq!(b, "ten");
assert_eq!(p.0, 10);
assert_eq!(p.1, "ten");
```

## Unit type
For historical reasons and convenience, the tuple type with no elements,
`()`, is often called unit or the unit type`.



### 8.1.4 Array and Slice types

Rust has two different types for a list of items:
- `[T; N]` an `array`
  `array` has a fixed size, and can be allocated on either the stack or the heap.
  
- `&[T]` a `slice`
  `slice` is a *view* into an array. It doesn't own the data it points to, it borrows it.


Examples:  
```rust
// A stack-allocated array
let array: [i32; 3] = [1, 2, 3];

// A heap-allocated array
let vector: Vec<i32> = vec![1, 2, 3];

// A slice into an array
let slice: &[i32] = &vector[..];
```

As you can see, the `vec!` macro allows you to create a `Vec<T>` easily. The vec! macro 
is also part of the standard library, rather than the language.

All in-bounds elements of arrays and slices are always initialized, and access to an 
array or slice is always bounds-checked.


# TYPES

```
i8          u8      Copy    i: -(2**n-1) to (2**n-1)-1 inclusive
i16         u16     Copy    u: 0 to (2**n)-1
i64         u64     Copy    where n is 8, 16, 32 or 64
i128        u128    Copy    *experimental
i32         u32     Copy    = default integer type (fastest, even on x64)
isize       usize   Copy    = (OS_arch: x64 or x32) pointer
```


## SCALARS
4 primary scalar types:
- integers
- floats
- booleans
- characters


## INTEGERS
let d = 98_222;
let h = 0xff;
let o = 0o77;
let binary = 0b1111_0000;
let byte_u8_only = b'A';

- all number literals except the byte literal allow a type suffix,
  such as 57u8, 3.14f32 and _ as a visual separator,
  such as 1_000 or 3.14_f64
- Defaults: i32 for integers and f64 for floats


## FLOATS
- Floating-point numbers are represented according to the IEEE-754 standard.
- The f32 type is a single-precision float, and f64 has double precision.


## TYPE ALIAS
alias type with the `type` keyword
type MagicPower = u16;
let run: MagicPower = 7800;
// run is of type u16 aliased as MagicPower

## COMPOUND TYPES

- Compound types can group multiple values of other types into one type.
- Rust has 2 primitive compound types: tuples and arrays.


## CUSTOM TYPES

Rust custom data types are formed mainly through the two keywords:
- struct: define a structure
- enum: define an enumeration
Constants can also be created via the const and static keywords.

