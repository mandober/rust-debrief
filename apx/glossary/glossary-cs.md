# CS glossary

<!-- TOC -->

- [Abstract Data Types](#abstract-data-types)
- [Algebraic Data Type](#algebraic-data-type)
- [Abstract Syntax Tree](#abstract-syntax-tree)
- [Aliasing](#aliasing)
- [Alignment](#alignment)
- [Application Binary Interface](#application-binary-interface)
- [Arity](#arity)
- [Assembly](#assembly)
- [Big Endianness](#big-endianness)
- [Bounded parametric polymorphism](#bounded-parametric-polymorphism)
- [Compiler](#compiler)
- [Composition over inheritance](#composition-over-inheritance)
- [Data structure](#data-structure)
- [Data type](#data-type)
- [Data typing](#data-typing)
- [Discriminant](#discriminant)
- [Dispatch](#dispatch)
- [Duck typing](#duck-typing)
- [Dynamically typed language](#dynamically-typed-language)
- [Endianness](#endianness)
- [Embarrassingly parallel](#embarrassingly-parallel)
- [Foreign Function Interface](#foreign-function-interface)
- [Generic programming](#generic-programming)
- [Hungarian notation](#hungarian-notation)
- [Initialization](#initialization)
- [Inlining](#inlining)
- [Interface](#interface)
- [Intermediate Representation](#intermediate-representation)
- [Invariant](#invariant)
- [Link-time optimization](#link-time-optimization)
- [Literal](#literal)
- [Little Endianness](#little-endianness)
- [Monomorphization](#monomorphization)
- [Nominal Types](#nominal-types)
- [Opaque data type](#opaque-data-type)
- [Parametric polymorphism](#parametric-polymorphism)
- [Pointer](#pointer)
- [Polymorphism](#polymorphism)
- [Polymorphic type](#polymorphic-type)
- [Prelude](#prelude)
- [Reentrant](#reentrant)
- [Reflection](#reflection)
- [Reference](#reference)
- [Sigil](#sigil)
- [Smart pointer](#smart-pointer)
- [Standard Library](#standard-library)
- [Statically Typed Language](#statically-typed-language)
- [Static dispatch](#static-dispatch)
- [Static Single Assignment](#static-single-assignment)
- [Token](#token)
- [Transmute](#transmute)
- [Type annotation](#type-annotation)
- [Type checking](#type-checking)
- [Type erasure](#type-erasure)
- [Type identifier](#type-identifier)
- [Type inference](#type-inference)
- [Type safety](#type-safety)
- [Typestate](#typestate)
- [Type system](#type-system)
- [Undefined behavior](#undefined-behavior)
- [Value](#value)
- [Variable](#variable)

<!-- /TOC -->



## Abstract Data Types
Any type that does not specify an implementation is an abstract data type. For instance, a stack can be implemented as an array or as a linked list. Other abstract types are queue, tree, graph, smart pointer, etc. Programming that is agnostic about concrete data types is called generic programming.

## Algebraic Data Type
Algebraic data type (ADT) is a kind of composite type, a type formed by combining other types. Two common classes of algebraic types are product types and sum types, also called tagged or disjoint unions or variant types.

## Abstract Syntax Tree
AST is abstract syntactic structure of the source code, representing the structure of the program. This tree data structure is an intermediate product of compilation, where each node of the tree denotes a construct occurring in the source code. As a ballpark example, an expression `x = 6` would have a subroot node, named "=", with two children, "x" and "6".

## Aliasing
Aliasing is situation in which a data location in memory can be accessed through different symbolic names in the program; modifying the data through one name implicitly modifies the value associated with all aliased names, which may not be expected. As a result, aliasing makes it particularly difficult to understand, analyze and optimize programs. Rust allows read-only aliasing.

## Alignment
Alignment is the way a data structure (value, object) is laid-out and accessed in memory. A memory access is aligned when the address of data is a multiple of its size. It specifies what addresses must be used to store a particular value. It is always a power of two.

## Application Binary Interface
ABI defines how to call a function at the assembly level. Languages define which ABI the external function uses to call its code.

## Arity
Arity refers to the number of arguments a function or operator takes. Languages that support automatic partial application, only have unary functions. Such functions are unary (arity is 1); functions that take two arguments are binary; generally all functions are n-ary.

## Assembly
An assembler language (ASM), is a low-level programming language in which there is a very strong, but often not one-to-one, correspondence between the language and the architecture's machine code instructions. Assembly language may also be called symbolic machine code.

## Big Endianness
Big endian architectures order bytes in memory with the most significant byte (MSB) of a multi-byte value in the lowest-numbered memory location.

## Bounded parametric polymorphism
Sometimes a limit on types which can be used in generics is needed and this can be achieved with bounded parametric polymorphism. It requires types, in order to be applicable for use with generics, to have something in common, like belonging to the same type class or to implement a common behaviour.

## Compiler
Compiler is primarily used to translate source code from a high-level programming language to a lower level language (e.g., assembly language, object code, or machine code), to create an executable program.

## Composition over inheritance
or composite reuse principle in object-oriented programming (OOP) is the principle that classes should achieve polymorphic behavior and code reuse by their composition (by containing instances of other classes that implement the desired functionality) rather than inheritance from a base or parent class.

## Data structure
Data structure is a particular way of organizing and storing data in a computer so that it can be accessed and modified efficiently. More precisely, a data structure is a collection of data values, the relationships among them and the functions or operations that can be applied to the data.

## Data type
A data type, or just type, represents a constraint placed upon the interpretation of data in a type system, describing representation, interpretation and structure of values stored in memory. A type defines the ways how the data of that type is intended to be used.

## Data typing
Data typing or just typing is the process of classifying data into data types.

## Discriminant
Each `enum` instance has a discriminant (tag) which is an integer associated with it, used to determine which variant the instance holds.

## Dispatch
Dispatch is a mechanism to determine which specific version is to actually run when code involves polymorphism. It comes in 2 major forms: _static_ and _dynamic_ dispatch.

## Duck typing
Identifying a value by probing for methods it responds to, rather then checking its concrete type. The name comes from the phrase: "if it walks like a duck, and quacks like a duck, then it must be a duck".

## Dynamically typed language
In dynamic languages variables don't carry the type: a variable can change its binding and type freely, throughout program's execution, i.e. values of different types can be (re)assigned to a variable, and interpreter will manage all the typing. Dynamic languages rely heavily on type inference. Type checking is performed at run-time.

## Endianness
Endianness only applies to processors that allow individual addressing of units of data (such as bytes) that are smaller than the basic addressable machine word. An architecture may use _big_ or _little endianness_, or both, or be configurable to use either. The x86 architecture is little endian. Most RISC architectures (SPARC, Power, PowerPC, MIPS) were originally big endian (ARM was little endian), but many (including ARM) are now configurable.

## Embarrassingly parallel
In parallel computing, a pleasingly (embarrassingly, perfectly) parallel problem is one where little effort is needed to separate the problem into a number of parallel tasks. 

## Foreign Function Interface
Interacting with functions written in a foreign language. In Rust, FFI is used to call functions written in another language (typically calling C or C++ functions from Rust) and when another language interfaces with Rust's functions (e.g. calling Rust's functions from JavaScript).

## Generic programming
Generic programming is a style of programming in which algorithms are written in terms of types to-be-specified-later that are then instantiated when needed for specific types provided as parameters.

## Hungarian notation
It is a variable-naming convention that reminds the user what type the variable has, by prefixing its name with a mnemonic.

## Initialization
A variable is initialized if it has been assigned a value and hasn't since been moved from. All other memory locations are assumed to be initialized. Only unsafe Rust can create such a memory without initializing it.

## Inlining
1. Function:
  inlining is removal of a function call by including the function body directly into the callsite, enabling optimizations (such as avoiding the overhead of a function call). It is controlled with the `inline` attribute:
  - `#[inline(never)]`
  - `#[inline]` hint to the compiler that inlining is desirable. It is required for any cross-crate inlining.
  - `#[inline(always)]` hint to the compiler that inlining is required.
2. Data structure: 
  when a data structure is inlined it means it is on the stack. e.g. array is inlined, contiguous sequence with a fixed size at compile time.

## Interface
Interface is used to define an abstract type that defines behaviors as method signatures. It describes the contract which types that implement it need to obey. In Rust, interface is realized through traits.


## Intermediate Representation
Intermediate Representation (IR)
It is the output LLVM IR Code, which can be shown in text form by passing `--emit=ir` to rustc.

## Invariant
An invariant is a condition that can be relied upon to be true during execution of a program, or during some portion of it. It is a logical assertion that is held to always be true during a certain phase of execution. For example, a loop invariant is a condition that is true at the beginning and end of every execution of a loop.

## Link-time optimization
Link-time optimization (LTO)
A type of optimization performed by a compiler at link time. In Rust, link-time optimization can only be performed on executables.

## Literal
A literal is an expression consisting of a single token, rather than a sequence of tokens, that immediately and directly denotes the value it evaluates to, rather than referring to it by name or some other evaluation rule.

## Little Endianness
Little endian processors order bytes in memory with the least significant byte (LSB) of a multi-byte value in the lowest-numbered memory location.

## Marshalling
Marshalling is the process of transforming the memory representation of an object to a data format suitable for storage or transmission.

Marshaling and serialization are loosely synonymous in the context of remote procedure call, but semantically different as a matter of intent. In particular, marshaling is about getting parameters from here to there, while serialization is about copying structured data to or from a primitive form such as a byte stream. In this sense, serialization is one means to perform marshaling, usually implementing pass-by-value semantics. It is also possible for an object to be marshaled by reference, in which case the data "on the wire" is simply location information for the original object. However, such an object may still be amenable to value serialization.


## Monomorphization
Monomorphization, a form of _static dispatch_, is a process of turning generic into concrete code. When generic code is used, concrete types replace generic types (concretization of code). This results in the code that would have been written in the first place if only concrete types were available.

Monomorphization increases code size, which, in some cases (that are avoidable by employing certain strategies) is referred to as the _code (size) bloat_, but the generated machine code is highly efficient (because concrete types are used) and very fast (which is never referred to as the _speed bloat_). The alternative with inversed trade-offs is _dynamic dispatch_.

## Nominal Types
Types that can be referred using a path directly. Specifically enum, struct, union and trait object.

## Opaque data type
An opaque data type is a data type whose concrete data structure is not defined in an interface. This enforces information hiding, since its values can only be manipulated by calling subroutines. Typical examples of opaque data types include handles for miscellaneous resources.

## Parametric polymorphism
Using parametric polymorphism, a function or a data type can be written 
generically so that it can handle values identically without depending on their type. Such functions and data types are called generic functions and generic datatypes and form the basis of generic programming.

## Pointer
A pointer is a variable that contains the memory address of some value. To access the value it points to, the pointer is dereferenced.

## Polymorphism
Polymorphism is the provision of a single interface to entities of different types.

## Polymorphic type
A polymorphic type is one whose operations can also be applied to values of some other type, or types.

## Prelude
Prelude defines a set of commonly and frequently used language items that is implicitly imported.

## Reentrant
A function is reentrant if it can be interrupted in the middle of its execution, and then be safely called again ("re-entered") before its previous invocations complete execution.

## Reflection
Reflection is the ability of a program to examine, introspect, and modify its own structure and behavior at runtime.

## Reference
A reference is a value that enables an indirect access a particular data; it refers to some data and accessing that data is called dereferencing the reference. A reference is distinct from the data itself. A reference may be implemented as the physical address of where the data is stored in memory; due to this, it is often confused with a pointer, but a reference may also be implemented in other ways, such as the offset between the data's address and some fixed "base" address, as an index into an array, or more abstractly as a handle; more broadly, in networking, a reference may be a network address such as URL.

## Sigil
A sigil is a symbol attached to a variable name, presenting some attribute of variable (datatype, scope, etc.), usually in prefix position; e.g. `$foo` where `$` is the sigil.

## Smart pointer
A smart pointer is an abstract data type that simulates a pointer while providing added features, such as automatic memory management or bounds checking. Such features are intended to reduce bugs caused by the misuse of pointers, while retaining efficiency. Smart pointers typically keep track of the memory they point to, and may also be used to manage other resources, such as network connections and file handles. Smart pointers prevent most situations of memory leaks, caused by pointer mishandling (double free, dangling pointers, etc.), by making the memory deallocation automatic.

## Standard Library
A standard library is the library made available across implementations of a programming language. It is often treated as part of the language by its users, although the designers may have treated it as a separate entity. The line between a language and its libraries differs between languages and it is not always clear. Some languages are designed so that the meanings of certain syntactic constructs cannot even be described without referring to the core library.

## Statically Typed Language
In static languages, variables are classified into types: once a variable acquires a type, it is type-locked. It cannot change its type and it accepts bindings only to the values of the matching type. Moreover, in some languages, a variable cannot even be rebound to a different value, after its initial binding, although some languages permit this by declaring a variable mutable. Static languages perform type checking at compile-time (many also at run-time).

## Static dispatch
Static dispatch usually resolves polymorphic calls using _monomorphization_. It allows _inlining_ of function calls.

## Static Single Assignment
Static Single Assignment (SSA) is a property of an intermediate representation (IR), which requires that each variable is assigned exactly once, and every variable is defined before it is used. Existing variables in the original IR are split into versions, new variables typically indicated by the original name with a subscript in textbooks, so that every definition gets its own version. In SSA form, use-def chains are explicit and each contains a single element.

## Token
Tokens are primitive productions in the grammar defined by regular, non-recursive, language.

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

## Typestate
Typestate is the dependence of the type and its behavior on the contained value: typestates define valid sequences of operations that can be performed upon an instance of a given type. Typestates, associate state information with variables of that type. This state information is used to determine, at compile-time, which operations are valid on an instance of that type. Operations performed on an object, that would usually only be executed at run-time, are performed upon the type state information, which is modified to be compatible with the new state of the object.

## Type system
A type system is a set of rules that assigns a type property to the various value-carrying constructs of a programming language.

## Undefined behavior
Undefined behavior (UB) is the result of executing computer code whose behavior is not prescribed by the language specification.

## Value
A value is a sequence of bits together with its interpretation.

## Variable
A variable is a storage location paired with an associated symbolic name (an identifier) that contains some quantity of information referred to as a value.

## Software development kit
Software development kit (SDK) containing docs, headers, libraries, samples, tools, etc. required for developing software for/around some product (for which the SDK has been released). For example, Windows 10 SDK, released by Microsoft, provides the latest headers, libraries, metadata, and tools for building Windows 10 apps.



