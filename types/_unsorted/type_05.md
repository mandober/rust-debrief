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

