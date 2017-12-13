# Types Classes

A **type** represents a constraint placed upon the interpretation of data in a type system, describing representation, interpretation and structure of values stored in memory. A **variable** is a storage location paired with an associated symbolic name (an identifier) that contains some quantity of information referred to as a value. A **value** is a sequence of bits together with its interpretation.

All data types that a language makes available are **built-in types**. Types can be classified in various ways; a common division is into primitives and non-primitives i.e. compound, composite or exotic types. Most languages also allow users to define additional types, usually by combining multiple elements of other types and defining the valid operations of the new data type, thereby creating user-defined custom types.


## Primitive types
Primitive types (or just primitives) are the basic building blocks of data provided by a language. Usually these basic types are defined by the language, rather than as part of the standard library. The most basic primitives are **scalars** i.e. numeric values like integer, floating-point, boolean and character. 

Depending on the language and its implementation, primitive data types may or may not have a 1:1 correspondence with objects in the computer's memory. However, one usually expects operations on basic primitive data types to be the fastest language constructs there are. Integer addition, for example, can be performed as a single machine instruction, and some processors offer specific instructions to process sequences of characters with a single instruction. 


## Compound and composite types
More complicated types are constructed starting from basic types. Compound types are made by homogeneous grouping of other types; Composite (aggregate) types are derived from heterogeneous grouping of other types. 

The ways other types are combined to form a combined type are called **data structures**. Some of these types include: arrays, records, unions, enums, sets, objects.

An *array* stores homogeneous elements in a specific, contiguous order and it is indexed by number (an index).

A *record* (also called tuple or struct) is a value that contains other values, typically in fixed number and sequence and typically indexed by names or by indices. The elements of records are usually called fields or members.

A *union* type definition will specify which of a number of permitted types may be stored in its instances, e.g. float OR integer. Contrast with a record, which could be defined to contain a float AND an integer; whereas, in a union, there is only one type allowed at a time.

A *tagged union* (also called a variant, variant record, discriminated union, disjoint union, or sum type) contains an additional field indicating its current type for enhanced type safety. A tagged union is a data structure used to hold a value that could take on several different, but fixed, types. Only one of the types can be in use at any one time, and a tag field explicitly indicates which one is in use.

An *enumeration* (also called enumerated type, enum, or factor) is a data type consisting of a set of named values called elements, members, enumeral, or enumerators of the type. It has distinct values, which can be compared and assigned, but which do not necessarily have any particular concrete representation in the memory. For example, the four suits in a deck of playing cards may be four enumerators belonging to an enumerated type named `Suit`. If a variable is declared to have a `Suit` as its data type, one can assign any of those four values to it.

A *set* is an abstract data structure that can store certain unique values, without any particular order.

An *object* contains a number of data fields, like a record, but also a number of subroutines (methods) for accessing or modifying them.


## Reference types
A reference is a value that enables a program to indirectly access a particular data, such as a variable or a record in the memory. The reference is said to *refer* to the data, and accessing that data is called *dereferencing* the reference.

The main non-composite, derived type is the **pointer**, a data type whose value refers directly (points to) to another value, stored elsewhere in the memory, using its address. It is a primitive kind of reference. Pointers are often stored in a format similar to an integer. They are considered a separate type to the type of data they point to, even if the underlying representation is the same. Because pointers store a memory location's address, instead of a value directly, inappropriate use of pointers can lead to *undefined behavior* in a program. 

A **smart pointer** is a reference type that acts like a pointer, but can only be accessed through particular methods, which provide additional features, such as automatic memory management or bounds checking. Such features are intended to reduce bugs caused by the misuse of pointers, while retaining efficiency. A **handle** is an abstract reference, and may be represented in various ways. A common example is a file handle, used to abstract contents of a file, in order to simplify read and write operations.


## Other types
**Unit type** is a type that allows only one value and thus can holds no information. It may also be regarded as the empty tuple, i.e. the product of no types. Several computer programming languages provide a unit type to specify the return type of a function that returns nothing, and the argument type of a function that does not require arguments.
https://www.wikiwand.com/en/Unit_type

Unit type to type theory is what number one is in algebra regarding mutiplication: any number multipled by one results in itself. 



Functions can have their own **function type** derived from the type of their parameters and return value.


## Abstract types
Any type that does not specify an implementation is an abstract data type. For instance, a stack can be implemented as an array or as a linked list. Other abstract types are queue, tree, graph, smart pointer, etc. Abstract types can be handled by code that does not know what underlying types are contained in them. Programming that is agnostic about concrete data types is called generic programming. Arrays and records can also contain underlying types, but are considered concrete because they specify how their contents or elements are laid out in memory.
