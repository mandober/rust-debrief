# Algebraic Data Types

<!-- TOC -->

- [Product types](#product-types)
- [Sum types](#sum-types)
- [Exponential types](#exponential-types)

<!-- /TOC -->


Algebraic data type (ADT) is a composite type formed by heterogeneous grouping of other types. The name is associated with algebra, where operations like addition and multiplication operate on variables, whereas in type theory they operate on data types, producing a type class for each applicable algebraic operation: addition produces _sum types_, multiplication _product types_, exponentiation _exponential types_.

Values of algebraic types are analyzed with **pattern matching**, which identifies a value by its constructor or field names and extracts the data it contains. Pattern matching on algebraic data types matches on the structural properties of an object rather than on the character sequence of strings.

The set of all possible values a type can have is its **cardinality**.


## Product types
The product type (also known as tuple, or record) typically has several **fields** of heterogeneous type. The resulting value of the product type is constructed by assigning a value to each and every individual field. The cardinality of the product type is the product of cardinalities of its constituting field types.

```rust
struct Foo {
  sex: bool,
  age: u8
}
```

In this example, `Foo` has 2 fields; `sex` field has a type boolean, `age` field has a type 8 bit unsigned integer. All possible values of the boolean field are contained in the boolean set `{true, false}` i.e. the cardinality of boolean type is 2. The cardinality of the integer set is 256, as 8 bit unsigned integer can have 256 distinct values, from 0 to 255 inclusive. So, the cardinality of the `Foo` type is 512, which is the **product** of **multiplication** of its constituting sets, `2 * 256`.

Due to the fact that all fields must be used when creating a value, product type is also known as "AND" type.


## Sum types
A sum type (also called a tagged union, variant record, disjoint union, discriminated union) is a data structure that describes all the different types whose value it can have.

The structure of sum type looks similar to product type as it also contains fields, referred to as **variants**. Unlike the product type, a value of the sum type is constructed by assigning a value only to one variant.The value can be changed, but only one variant can be in use at any one time; a tag field, called **discriminant** explicitly indicates which one is in use.

Due to this fact the sum type is also known as "OR" type. 

The set of all possible values of a sum type is the set-theoretic sum i.e. the disjoint union, of the sets of all possible values of its variants.

A value of a variant type is usually created with a quasi-functional entity called a **constructor**. Each variant has its own constructor, which takes a specified number of arguments of defined type.

Enumerated types are a special case of sum types in which the constructors take no arguments, as exactly one value is defined for each constructor.

The cardinality of the sum type is the sum of cardinalities of its variants' types. For example, regarding the previous example, if `Foo` was a sum type, its cardinality would be the sum of cardinalities of its variants, i.e. `2 + 256`, 258.

In fact, since enum is a disjoint union, it is more precise to say that if the contributing sets overlap, the overlap is discriminated out i.e. it doesn't contribute to cardinality. For example, a sum type that has 2 variants of the same, boolean, type would have cardinality of 2 (not 4) because there would actually be only 2 distinct values.


## Exponential types
Functions are exponential types. Considering a unary function `fn(P) -> R`, where P is the input type and R is the return type, the cardinality is `R^P`.
