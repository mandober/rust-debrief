# About types


Primitives: scalar types
- integers, machine dependent integers
- floats
- booleans
- characters

Primitives: compound types
- array: `[T; N]`
- tuple: `(T, U,...)`
- slices: `&[T]`
- string slices: `&str`

Primitives: reference types
- function pointer: `fn`
- reference `&T`, `ref T`; `&mut T`, `ref mut T`
- pointers: `*const T`, `*mut T`

Agragate types
`Option<T>`
`Result<T, E>`

`Box<T>`
`Rc`
`Arc`
`Cell`
`RefCell`


Primitives are implemented by the compiler.
std implements methods directly on the primitive types.
implicit methods on primitive types are documented in std.
std exports many modules with the same name as primitive types.


# Types in Rust

- primitive vs compound (aggregate)
- concrete vs generic
- value vs object; Copy vs Move (Clone)
- first-class type, core types
- storage: stack, heap, binary, handle
- grouping: numeric, text, sequence, refs, fns
- relation: owned, ref, mref
- fixed vs growable pairs
- sized vs unsized

### core types
- string: `String`
- vector: `Vec<T>`
- hash map: `HashMap`

- trait object: `&TraitName`
- structure: `struct`
- enumeration: `enum`

## Grouping
numberic: int, floats i/usize, (bool)
text: String, string slice, char
sequence: array, vec, slice
refs: references, raw pointers
fn: fn, closure

# Types
[s] = storage: [s] on stack, [h] stack ptr to heap data, [b] in final binary
[p] = is primitive? y|n
[m] = has module? y|n


name         | type          | sample | size    |s|p| trait|m| spec
-------------|---------------|--------|---------|-|-|------|-|-------
boolean      | `bool`        | true   |      1b |s|y| Copy |n|
character    | `char`        | 'ß'    |      4b |s|y| Copy |y|
float32      | `f32`         | 1.2525 |     32b |s|y| Copy |y|
float64      | `f64`         | 3.1425 |     64b |s|y| Copy |y|
mach int     | `isize`       | -100   | 32b/64b |s|y| Copy |y|
mach uint    | `usize`       | 100    | 32b/64b |s|y| Copy |y|
int          | `i8-64`       | -42    |   8-64b |s|y| Copy |y|
uint         | `u8-64`       | 42     |   8-64b |s|y| Copy |y|
             |               |        |         | | |      | |
string slice | `&str`        | "lit"  |         |s|y| Copy |n| valid UTF8
string       | `String`      | "own"  |         |h|n| Move |n| valid UTF8
array        | `[T; n]`      | [0,1,2]|T size*n | | |      | |
slice        | `&str`        | "lit"  |         |s|y| Copy |n| valid UTF8




# Heap values
are referenced by a variable on the stack, which
contains the memory address of the object on the heap

## slice
Slices are a view into a block of memory represented as a pointer and a length.
A view into a contiguous sequence `[T]`

## string slice
Consists of 2 parts: a pointer to some bytes and a length.
A view into a string.





---

## Type annotations
Types that appear in explicit type annotations

## Concrete

owned     | ref         | mut ref
----------|-------------|------------
`int`     | `&int`      | `&mut int`
5         | &5          | &mut 5
`bool`    | `&bool`     | `&mut bool`
`char`    | `&char`     | `&mut char`
          | `&str`      | `&mut str`



`i8`   `u8`     `&i8`   `&u8`     `&mut i8`   `&mut u8`
`i16`  `u16`    `&i16`  `&u16`    `&mut i16`  `&mut u16`
`i32`  `u32`    `&i32`  `&u32`    `&mut i32`  `&mut u32`
`i64`  `u64`    `&i64`  `&u64`    `&mut i64`  `&mut u64`

`isize` `usize`  `&isize` `&usize`  `&mut isize` `&mut usize`
`f32`            `&f32`              `&f32`

`f32`
`f64`

`bool` boolean
`char` character

`&str` string slice


## Generic

`*const T`      pointer
`*mut T`        pointer mutable

`&T`            reference
`&mut T`        reference mutable

`[T; n]`        array
`&[T; n]`       array reference
`&mut [T; n]`   array reference mutable

`[T]`           contiguous sequence. `[i32]`
`&[T]`          slice (ref)
`&mut [T]`      mut slice (mut ref)

`(T, U, ...)`   tuple. can contain mix of any other types
`&(T,U, ...)`   tuple reference

`fn`

`String`
`&String`

`std::vec::Vec<T>`  vector     `std::vec::Vec<i32>`
`&std::vec::Vec<T>` vector ref `&std::vec::Vec<i32>`





## Example code:

```rust
#![allow(unused_variables, dead_code, unused_assignments)]

fn main() {
    
    // BOOLEANS
    let lie = &mut true;
    let b: &mut bool = &mut true; // although truth cannot be manipulated!?
    println!("boolean: {}", b);
    // alt: b is here `bool`, but right after it is `&mut bool`
    let ref mut b: bool = true;
    println!("{}b", std::mem::size_of_val(&b));


    // INTEGERS
    let x: &mut i32 = &mut 5; // although 5 cannot be mutated
    println!("int: {}", x);


    // TEXT
    // String (growable text)
    let s: String = "literal".to_string();
    // string slice (view into text: view into String or literal string)
    let ss: &str = &s[0..2];
    println!("string slice ss: {}", ss);
    println!("{}b", std::mem::size_of_val(&ss));
    // String binding must be mut to take a mut string slice
    let mut z: String = "literal".to_string();
    // mut string slice
    let zz: &mut str = &mut z[0..2];
    println!("mut string slice zz: {}", zz);
    // literal string (fixed text)
    let l = "literal";
    let ss = &l[0..2];
    println!("{}", ss);


    // SEQUENCES

    // array (fixed seq)
    let mut arr: [i32; 6] = [2; 6];

    // vec (growable seq)
    let mut v: Vec<i32> = vec![0,1,2,3,4];

    // slice (view into seq)
    let as: &[i32] = &arr[1..3];
    let mas: &mut [i32] = &mut arr[1..3];
    let vs: &[i32] = &v[1..3];
    let mvs: &mut [i32] = &mut v[1..3];
    
    // alt: rev is here a sequence: `[i32]`, not a slice: `&[i32]`
    let ref mut rev = v[1..3];
    let ref mut rev: [i32] = v[1..3]; // explicitly annotated
    
    // vec without true owner: e will be mut ref after this line
    let ref mut e = vec![0,1,2];
    e.push(22);
    println!("vec el: {:?}", (*e)[3]);  // same as below due to auto-deref
    println!("vec el: {:?}", e[3]);
}
```
