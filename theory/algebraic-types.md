# Algebraic Data Types (ADT)


Algebraic data type is a kind of composite type, a type formed by heterogeneous grouping of other types.

The name is associated with algebra, where operations like addition and multiplication operate on variables, whereas in type theory they operate on data types producing a type class for each applicable algebraic operation: addition produces **sum** types, multiplication **product** types, exponentiation **exponential** types.

Product types are tuples and records, and **sum types** are called tagged or disjoint unions or variant types.

Values of algebraic types are analyzed with **pattern matching**, which identifies a value by its constructor or field names and extracts the data it contains. Pattern matching on algebraic data types matches on the structural properties of an object rather than on the character sequence of strings.


## Product types
The values of a product type typically contain several values, called fields. All values of that type have the same combination of field types. The set of all possible values of a product type is the set-theoretic product i.e. the Cartesian product of the sets of all possible values of its field types.


## Sum types
A sum type (also called a tagged union, variant record, disjoint union, discriminated union) is a data structure used to hold a value that could take on several different, but fixed, types. Only one of the types, called **variant**, can be in use at any one time, and a **tag** field, called **discriminant** explicitly indicates which one is in use.

A value of a variant type is usually created with a quasi-functional entity called a **constructor**. Each variant has its own constructor, which takes a specified number of arguments with specified types. 

The set of all possible values of a sum type is the set-theoretic sum i.e. the disjoint union, of the sets of all possible values of its variants.

**Enumerated types** are a special case of sum types in which the constructors take no arguments, as exactly one value is defined for each constructor.


In Rust, sum types are represented by `enum`:

```rust
enum Sum {
    Booleans(bool),
    Naturals(u8)
}

enum Discr {
    B1(bool),
    B2(bool),
}
```

Here, `Sum` is a set of all values which are _either_ booleans _or_ unsigned integers. The number of possible values of this enum is a sum of sets, `Sum = bool + u8`, which is 2 possible `bool` values, plus 256 possible `u8` values, results in 258 distinct values. So the **cardinality** of this sum type is 258.

In fact, since enum is a disjoint (discriminated) union, it is more precise to note that if the contributing sets overlap, the overlap is discriminated out, it doesn't contribute to cardinality.

This would be the case of second example with two boolean variants. The cardinality of `Discr` type would be 4, but since overlaps are discriminated out, it is 2 and all possible distinct values are:

```
Discr::B1 = true;
```








`struct`, `tuple`
A record, also called tuple or struct is a value that contains other values, typically in fixed number and sequence and typically indexed by names or by indices. The elements of records are usually called fields or members.
They are `AND` types.

```rust
struct Product {
    a: bool,
    b: u8
}

let tuple: (bool, u8) = (a, b);
```
The set of all possible values of `Product` is `{(a,b): a ∈ bool, b ∈ u8 }`  
This is Cartesian product, expressed as `Product = bool × u8`  
The number of all possible values of Product is the number of possible values of bool (which is 2) times the number of possible values of u8 (which is 256) i.e. 2*256=512. The same holds for tuple.



## Nullability
Sum types are typically used to express nullability:
Many languages have a `null` type that represents absence of a value.
In languages with strong type system this concept is represented with sum types: either there is some value or there is nothing.

```rust
// an Option is either "something", containing a type, or "nothing"
enum Option<T> {
    Some(T),
    None
}
```




## Exponential types

Exponential types are encountered in function types, `fn(P) -> R`, where P is the input type and T is the return type. The total number of values is R^P



```rust



struct Rgb {
  r: u8,
  g: u8,
  b: u8
}

enum Colors {
  Rgb(u8, u8, u8),
  Hls(u8, u8, u8),
  Nom(String)
}



let red: (u8, u8, u8) = (0xff, 0x00, 0x00);


fn exp<T>(p: T) -> () { p }

fn exp<T>(p: T) -> bool { p }

fn exp(p: u8) -> bool { p }

```
bool^u8 = 2^256

