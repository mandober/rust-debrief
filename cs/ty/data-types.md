# Data Types

* built-in types
* custom types
* primitive types
* scalar types
* compound types
* composite types 
* data structures
  * array
  * record
  * union
  * tagged union
  * enumeration
  * set
  * object
  * reference types
    * reference
    * pointer
    * smart pointer
    * handle
  * unit type
  * function type
* abstract data types



All data types that a language makes available are **built-in types**. Types can be classified into many categories: primitive, compound, composite, exotic types. Most languages also allow users to define additional types, usually by combining multiple elements of other types and defining the valid operations of the new data type, thereby creating user-defined **custom types**.

**Primitives** are the basic building blocks of data provided by a language. Usually these basic types are defined by the language, rather than as part of the standard library. The most basic primitives are **scalars** e.g. integers, floating-points, booleans and characters. They are atomic values that cannot be divided further and they do not depend on other types. Operations on scalar primitive types are the fastest language constructs there are. Integer addition, for example, can be performed as a single machine instruction.

More complicated types are constructed starting from basic types: **compound types** are made by homogeneous grouping of other types, **composite** or **aggregate** types are derived from heterogeneous grouping of other types. The ways other types are combined to form a combined type are called **data structures**. Some of these general types include: arrays, records, unions, enums, sets, objects.

An **array** stores homogeneous elements in a specific, contiguous order and it is indexed by number (an index). A **record** (also called **tuple** or **struct**) is a value that contains other values, typically in fixed number and sequence and typically indexed by names or by indices. The elements of records are usually called fields or members. A **union** type definition will specify which of a number of permitted types may be stored in its instances. A **tagged union** (also called a variant, variant record, discriminated union, disjoint union, or sum type) contains an additional field indicating its current type for enhanced type safety. A tagged union is a data structure used to hold a value that could take on several different, but fixed, types. Only one of the types can be in use at any one time, and a tag field explicitly indicates which one is in use. 

An **enumeration** (also called enumerated type, enum, or factor) is a data type consisting of a set of named values called elements, members, enumeral, or enumerators of the type. It has distinct values, which can be compared and assigned, but which do not necessarily have any particular concrete representation in the memory. A **set** is an abstract data structure that can store certain unique values, without any particular order. An **object** contains a number of data fields, like a record, but also a number of methods for accessing or modifying them.

Another major type group is called **reference types**, comprised of references, pointers, handles, smart references. A **reference** is a value that enables a program to indirectly access a particular data, such as a variable or a record in the memory. The reference is said to refer to the data, and accessing that data is called *dereferencing* the reference.

The main non-composite, derived type is the **pointer**, a data type whose value refers directly (points to) to another value, stored elsewhere in the memory, using its address. It is a primitive kind of reference. Pointers are often stored in a format similar to an integer. They are considered a separate type to the type of data they point to, even if the underlying representation is the same. Because pointers store a memory location's address, instead of a value directly, inappropriate use of pointers can lead to *undefined behavior* in a program.

A **smart pointer** is a reference type that acts like a pointer, but can only be accessed through particular methods, which provide additional features, such as automatic memory management or bounds checking. Such features are intended to reduce bugs caused by the misuse of pointers, while retaining efficiency. A **handle** is an abstract reference and may be represented in various ways. A common example is a *file handle*, used to abstract contents of a file, in order to simplify read and write operations.

**Unit type** is a type that allows only one value and thus can holds no information. It may also be regarded as the empty tuple. Functions can have their own **function type** derived from the type of their parameters and return value.

Any type that does not specify an implementation is an **abstract data type**. For instance, a stack can be implemented as an array or as a linked list. Other abstract types are queue, tree, graph, smart pointer, etc. Abstract types can be handled by code that does not know what underlying types are contained in them. Programming that is agnostic about concrete data types is called generic programming. Arrays and records can also contain underlying types, but are considered concrete because they specify how their contents or elements are laid out in memory.



## Primitive
A *data primitive* (or just primitive) is any datum that can be read from or written to computer memory using one memory access (for instance, both a byte and a word are primitives).

## Data Aggregate
An aggregate is a group of primitives that are logically contiguous in memory and that are viewed collectively as one datum.

For example, an aggregate could be 3 logically contiguous bytes, the values of which represent the 3 coordinates of a point in space.

When an aggregate is entirely composed of the same type of primitive, it is called an array.
