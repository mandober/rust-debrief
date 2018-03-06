<!-- TOC -->

- [Structural type system](#structural-type-system)
- [Nominal type system](#nominal-type-system)
- [Nominal subtyping](#nominal-subtyping)
- [Option type](#option-type)
- [Nullable types](#nullable-types)
- [Function type](#function-type)

<!-- /TOC -->


## Structural type system
Structural (or property-based) type system is a major class of type system, in which type compatibility and equivalence are determined by the type's actual structure or definition, and not by other characteristics such as its name or place of declaration.

Structural systems are used to determine if types are equivalent and whether a type is a subtype of another. It contrasts with nominative systems, where comparisons are based on the names of the types or explicit declarations, and duck typing, in which only the part of the structure accessed at runtime is checked for compatibility.

## Nominal type system
Nominal or nominative (or name-based) type system is a major class of type system, in which compatibility and equivalence of data types is determined by explicit declarations and/or the name of the types. 

Nominal systems are used to determine if types are equivalent, as well as if a type is a subtype of another. It contrasts with structural systems, where comparisons are based on the structure of the types in question and do not require explicit declarations.

## Nominal subtyping
In a similar fashion, nominal subtyping means that one type is a subtype of another if and only if it is explicitly declared to be so in its definition. Nominally-typed languages typically enforce the requirement that declared subtypes be structurally compatible. However, subtypes which are structurally compatible "by accident", but not declared as subtypes, are not considered to be subtypes. Rust (as well as C++, C#, Java) primarily use both nominal typing and nominal subtyping.


## Option type
An option type or maybe type is a polymorphic type that represents encapsulation of an optional value; e.g., it is used as the return type of functions which may or may not return a meaningful value when they are applied.

It consists of a constructor which is either empty (named `None` or `Nothing`), or which encapsulates the original data type `T` (written `Just T` or `Some T`). Outside of functional programming, these are termed nullable types.

## Nullable types
Nullable types are a feature of the type system of some languages which allow the value to be set to the special value `NULL` instead of the usual possible values of that data type.

In statically-typed languages, a nullable type is an Option type, while in dynamically-typed languages (where values have types, but variables do not), equivalent behavior is provided by having a single null value.

Primitive types such as integers and booleans cannot generally be null, but the corresponding nullable types (nullable integer and nullable boolean) can also assume the NULL value. NULL is frequently used to represent a missing value or invalid value.

A boolean variable makes the effect more clear. Its values can be either "true" or "false", while a nullable boolean may also contain a representation for "undecided". However, the interpretation or treatment of a logical operation involving such a variable depends on the language.

In contrast, object pointers can be set to NULL by default in most common languages, meaning that the pointer or reference points to nowhere, that no object is assigned (the variable does not point to any object). 

Nullable references were invented by C.Hoare in 1965 as part of the Algol W language. Hoare later described their invention as a "billion-dollar mistake". This is because object pointers that can be NULL require the user to check the pointer before using it and require specific code to handle the case when the object pointer is NULL.

## Function type
Functions can have their own function type derived from the type of their parameters and return value.
