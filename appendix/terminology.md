# Terminology


- [Abstract data types](#abstract-data-types)
- [Algebraic data type](#algebraic-data-type)
- [Application Binary Interface](#application-binary-interface)
- [Assembly](#assembly)
- [Big Endianness](#big-endianness)
- [Blanket implementations](#blanket-implementations)
- [Bounded parametric polymorphism](#bounded-parametric-polymorphism)
- [Compiler](#compiler)
- [Data structure](#data-structure)
- [Data type](#data-type)
- [Data typing](#data-typing)
- [Discriminant](#discriminant)
- [Dispatch](#dispatch)
- [Duck typing](#duck-typing)
- [Dynamically typed language](#dynamically-typed-language)
- [Dynamically Sized Type](#dynamically-sized-type)
- [Endianness](#endianness)
- [Fat pointer](#fat-pointer)
- [Foreign Function Interface (FFI)](#foreign-function-interface-ffi)
- [Generic programming](#generic-programming)
- [Inlining](#inlining)
- [Interface](#interface)
- [Interior mutability](#interior-mutability)
- [Invariant](#invariant)
- [Literal](#literal)
- [Little Endianness](#little-endianness)
- [Marker interfaces](#marker-interfaces)
- [Monomorphism](#monomorphism)
- [Opaque data type](#opaque-data-type)
- [Parametric polymorphism](#parametric-polymorphism)
- [Phantom data](#phantom-data)
- [Phantom types](#phantom-types)
- [Pointer](#pointer)
- [Polymorphic type](#polymorphic-type)
- [Polymorphism](#polymorphism)
- [Reentrant](#reentrant)
- [Reflection](#reflection)
- [Slice](#slice)
- [Statically typed language](#statically-typed-language)
- [Static dispatch](#static-dispatch)
- [Token](#token)
- [Trait object](#trait-object)
- [Transmute](#transmute)
- [Type annotation](#type-annotation)
- [Type checking](#type-checking)
- [Type erasure](#type-erasure)
- [Type identifier](#type-identifier)
- [Type inference](#type-inference)
- [Type safety](#type-safety)
- [Type system](#type-system)
- [Value](#value)
- [Variable](#variable)
- [Zero Sized Type](#zero-sized-type)

<!-- /TOC -->


## Abstract data types
Any type that does not specify an implementation is an abstract data type. For instance, a stack can be implemented as an array or as a linked list. Other abstract types are queue, tree, graph, smart pointer, etc. Programming that is agnostic about concrete data types is called generic programming.

## Algebraic data type
Algebraic data type is a kind of composite type, a type formed by combining other types. Two common classes of algebraic types are product types and sum types, also called tagged or disjoint unions or variant types.

## Application Binary Interface
ABI defines how to call a function at the assembly level. Languages define which ABI the external function uses to call its code.

## Assembly
An assembler language (asm), is a low-level programming language in which there is a very strong, but often not one-to-one, correspondence between the language and the architecture's machine code instructions. Assembly language may also be called symbolic machine code.

## Big Endianness
Big endian architectures order bytes in memory with the most significant byte (MSB) of a multi-byte value in the lowest-numbered memory location.

## Blanket implementations
Conditionally implenting a trait for a type that implements some other specific trait. Implementation of a trait on any type that satisfies the trait bounds are called blanket implementations. For example, the standard library implements `ToString` trait on any type that implements `Display` trait.

## Bounded parametric polymorphism
Sometimes a limit on types which can be used in generics is needed and this can be achived with bounded parametric polymorphism. It requires types, in order to be applicable for use with generics, to have something in common, like belonging to the same type class or to implement a common behaviour.

## Compiler
Compiler is primarily used to translate source code from a high-level programming language to a lower level language (e.g., assembly language, object code, or machine code), to create an executable program.

## Data structure
Data structure is a particular way of organizing and storing data in a computer so that it can be accessed and modified efficiently. More precisely, a data structure is a collection of data values, the relationships among them and the functions or operations that can be applied to the data.

## Data type
A data type, or just type, represents a constraint placed upon the interpretation of data in a type system, describing representation, interpretation and structure of values stored in memory. A type defines the ways how the data of that type is intended to be used.

## Data typing
Data typing or just typing is the process of classifying data into data types.

## Discriminant
Each `enum` instance has a discriminant (tag) which is an integer associated with it, used to determine which variant the instance holds.

## Dispatch
Dispatch is a mechanism to determine which specific version is to actually run when code involves polymorphism. It comes in 2 major forms: *static* and *dynamic* dispatch.

## Duck typing
Identifing a value by probing for methods it responds to, rather then checking its concrete type. The name comes from the phrase: "if it walks like a duck, and quacks like a duck, then it must be a duck".

## Dynamically typed language
In dynamic languages variables don't carry the type: a variable can change its binding and type freely, throughout program's execution, i.e. values of different types can be (re)assigned to a variable, and interpreter will manage all the typing. Dinamic languages rely heavily on type inference. Type checking is performed at run-time.

## Dynamically Sized Type
Dynamically Sized Type (DST) is a type without a statically known size or alignment. Becasue of this, these types can only exist behind some kind of fat pointer. A pointer to a DST is a fat pointer consisting of the pointer and the information that "completes" them. Two major DSTs exposed by the language are trait objects and slices.

## Endianness
Endianness only applies to processors that allow individual addressing of units of data (such as bytes) that are smaller than the basic addressable machine word. An architecture may use *big* or *little endianness*, or both, or be configurable to use either. The x86 architecture is little endian. Most RISC architectures (SPARC, Power, PowerPC, MIPS) were originally big endian (ARM was little endian), but many (including ARM) are now configurable.

## Fat pointer
A pointer with accompanying extra information. It comprises a pointer and one or more associated fields that "complete" the pointer. For example, a string is a fat pointer made up of pointer to some data on the heap, a length (number of characters it points to) and a capacity (additional space for characters reserved).

## Foreign Function Interface (FFI)
Interacting with functions written in a foreign language. In Rust, FFI is used to call functions written in another language (typically calling C or C++ functions from Rust) and when another language interfaces with Rust's functions (e.g. calling Rust's functions from JavaScript).

## Generic programming
Generic programming is a style of programming in which algorithms are written in terms of types to-be-specified-later that are then instantiated when needed for specific types provided as parameters.

## Inlining
When data structure is inlined it means it is on the stack. e.g. array is inlined contiguous sequence with a fixed size at compile time. To inline a function means the compiler should substitute the body of the function inline by performing inline expansion, i.e. by inserting the function code at the address of each function call, thereby saving the overhead of a function call.

## Interface
Interface is used to define an abstract type that defines behaviors as method signatures. It describes the contract which types that implement it need to obey. In Rust, interface is realized through traits.

## Interior mutability
Interior mutability is a pattern where an immutable type exposes an API for mutating an interior value, and the borrowing rules apply at runtime instead of compile time.

## Invariant
An invariant is a condition that can be relied upon to be true during execution of a program, or during some portion of it. It is a logical assertion that is held to always be true during a certain phase of execution. For example, a loop invariant is a condition that is true at the beginning and end of every execution of a loop.

## Literal
A literal is an expression consisting of a single token, rather than a sequence of tokens, that immediately and directly denotes the value it evaluates to, rather than referring to it by name or some other evaluation rule.

## Little Endianness
Little endian processors order bytes in memory with the least significant byte (LSB) of a multi-byte value in the lowest-numbered memory location.

## Marker interfaces
Marker interfaces contain no methods at all and serve to provide run-time information to generic processing using reflection. In Rust, marker interface is realized through marker traits: Copy, Sized, Send, Sync.

## Monomorphism
Monomorphization is the process of turning generic code into concrete code with the concrete types filled in, in the place of generic types, resulting in concretization of code. It is a form of *static dispatch*.

## Opaque data type
An opaque data type is a data type whose concrete data structure is not defined in an interface. This enforces information hiding, since its values can only be manipulated by calling subroutines. Typical examples of opaque data types include handles for miscellaneous resources.

## Parametric polymorphism
Using parametric polymorphism, a function or a data type can be written 
generically so that it can handle values identically without depending on their type. Such functions and data types are called generic functions and generic datatypes and form the basis of generic programming.

## Phantom data
Zero-sized type used to mark things that "act like" they own a `T`. Adding a `PhantomData<T>` field to your type tells the compiler that your type acts as though it stores a value of type `T`, even though it doesn't really. This information is used when computing certain safety properties. More about [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).

## Phantom types
Phantom data and phantom types are related, but not identical. A phantom type parameter is simply a type parameter which is never used. In Rust, this often causes the compiler to complain, and the solution is to add a dummy use by way of [phantom data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).

## Pointer
A pointer is a variable that contains the memory address of some value. To access the value it points to, the pointer is dereferenced. In Rust, these kind of pointers are called raw pointers; there is immutable raw pointer, `*const T` and mutable raw pointer, `*mut T`.

## Polymorphic type
A polymorphic type is one whose operations can also be applied to values of some other type, or types.

## Polymorphism
Polymorphism is the provision of a single interface to entities of different types.

## Reentrant
A function is reentrant if it can be interrupted in the middle of its execution, and then be safely called again ("re-entered") before its previous invocations complete execution.

## Reflection
Reflection is the ability of a program to examine, introspect, and modify its own structure and behavior at runtime.

## Slice
A slice is a view into some contiguous storage. Slice is a fat pointer; the information that completes a slice is the number of elements it points to.

## Statically typed language
In static languages, variables are classified into types: once a variable aquires a type, it is type-locked. It cannot change its type and it accepts bindings only to the values of the matching type. Moreover, in some languages, a variable cannot even be rebound to a different value, after its initial binding, although some languages permit this by declaring a variable mutable. Static languages perform type checking at compile-time (many also at run-time).

## Static dispatch
Static dispatch usually resolves polymorphic calls using *monomorphization*. It allows *inlining* of function calls.

## Token
Tokens are primitive productions in the grammar defined by regular, non-recursive, language.

## Trait object
Dynamically sized type that implements some trait. The original type is *erased* in favor of runtime reflection, with a *vtable* containing all the information necessary to use the type. The information that "completes" a trait object (trait object is a fat pointer) is a pointer to its vtable.

## Transmute
Reinterpret the bits of a value of one type as another type.

## Type annotation
Type annotation is an explicit identification of the data type by placing type identifiers directly on expressions in the source code.

## Type checking
The type system uses data type information to verify and enforce the constraints of data types through a process called type checking.

## Type erasure
Making the compiler forget about a determined type of a value. For example, trait object operations can be seen as erasing the compiler’s knowledge about the specific type of the reference, and hence trait objects are sometimes referred to as type erasure.

## Type identifier
Type identifier directly refers to a named data type, with the same name as the identifier, but it can also refer to a type alias.

## Type inference
Deduction of data types based on context rather than from explicit type annotations.

## Type safety
Type safety is the extent to which a language discourages type errors.

## Type system
A type system is a set of rules that assigns a type property to the various value-carrying constructs of a programming language.

## Value
A value is a sequence of bits together with its interpretation.

## Variable
A variable is a storage location paired with an associated symbolic name (an identifier) that contains some quantity of information referred to as a value.

## Zero Sized Type
Zero Sized Type (ZST) are types that occupy no space (empty tuple, empty array).

