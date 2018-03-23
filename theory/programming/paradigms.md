# Programming Paradigms

<!-- TOC -->

- [Polymorphism](#polymorphism)
- [Marker interfaces](#marker-interfaces)
- [Reflection](#reflection)
- [polymorphic type](#polymorphic-type)
- [Parametric polymorphism](#parametric-polymorphism)
- [Generic programming](#generic-programming)
- [invariant](#invariant)
- [type system](#type-system)
- [type class](#type-class)
- [standard library](#standard-library)
- [Undefined behavior](#undefined-behavior)
- [optimizing compiler](#optimizing-compiler)
- [Intermediate representation (IR)](#intermediate-representation-ir)
- [object code](#object-code)

<!-- /TOC -->

## Polymorphism
*Polymorphism* is the provision of a single interface to entities of different types. *Interface* is used to define an abstract type that defines behaviors as method signatures. 

## Marker interfaces
*Marker interfaces* contain no methods at all and serve to provide run-time information to generic processing using reflection. 

## Reflection
is the ability of a computer program to examine, introspect, and modify its own structure and behavior at runtime.

## polymorphic type
is one whose operations can also be applied to values of some other type, or types. 

## Parametric polymorphism
allows code to be written generically, so that it can handle values uniformly without depending on their type. When code is written without mention of any specific type and thus can be used transparently with any number of new types, it is said to be *generic*. Parametric polymorphism is a way to make a language more expressive while still maintaining full static type-safety. A function that can evaluate to or be applied to values of different types is known as a polymorphic or generic function.

Sometimes a limit on types which can be used in generics (for example, in generic function) is needed and this can be achived with *bounded parametric polymorphism*. It requires types, in order to be applicable for use with generics, to have something in common, like belonging to the same type class or to implement a common behaviour.


## Generic programming
is a style of computer programming in which algorithms are written in terms of types to-be-specified-later that are then instantiated when needed for specific types provided as parameters. This approach permits writing common functions that differ only in the set of types on which they operate when used, thus reducing duplication.



## invariant
an *invariant* is a condition that can be relied upon to be true during execution of a program, or during some portion of it. It is a logical assertion that is held to always be true during a certain phase of execution. For example, a loop invariant is a condition that is true at the beginning and end of every execution of a loop.



https://www.wikiwand.com/en/Late_binding

https://www.wikiwand.com/en/Virtual_method_table

https://www.wikiwand.com/en/Covariance_and_contravariance_(computer_science)

https://www.wikiwand.com/en/Type_system




All programming languages have some primitive building blocks for the description of data and the processes or transformations applied to them. These primitives are defined by syntactic and semantic rules which describe their structure and meaning respectively.

The **syntax** is the set of rules that defines the allowed combinations of symbols. **Semantics** refers to the meaning of languages, as opposed to their form.

An **expression** is a combination of one or more literals, constants, variables, operators, and functions that the programming language interprets and computes to produce another value. This process is called evaluation.

A **statement** is the smallest standalone element that expresses some action to be carried out. A statement may have internal components (e.g. expressions). Many languages make a distinction between statements and definitions, with a statement only containing executable code and a **definition** instantiating an identifier, while an expression evaluates to a value only. A distinction can also be made between simple and compound statements; the latter may contain statements as components.

> An expression is evaluated, a statement is executed, definition (declaration) instantiates an identifier.





Elements: 
- syntax
- semantics
  - Static semantics
  - Dynamic semantics
- Type system
  - Typed versus untyped languages
  - Static versus dynamic typing
  - Weak and strong typing
- Standard library and run-time system




## type system

A **type system** defines how a programming language classifies values and expressions into types, how it can manipulate those types and how they interact. The goal of a type system is to verify and usually enforce a certain level of correctness in programs written in that language by detecting certain incorrect operations. Any decidable type system involves a trade-off: while it rejects many incorrect programs, it can also prohibit some correct, albeit unusual programs. In order to bypass this downside, a number of languages have type loopholes, usually unchecked casts that may be used by the programmer to explicitly allow a normally disallowed operation between different types. In most typed languages, the type system is used only to type check programs, but a number of languages, usually functional ones, infer types, relieving the programmer from the need to write type annotations. The formal design and study of type systems is known as type theory.

## type class
a **type class** is a type system construct that supports ad hoc polymorphism. This is achieved by adding constraints to type variables in parametrically polymorphic types. Such a constraint typically involves a type class T and a type variable a, and means that a can only be instantiated to a type whose members support the overloaded operations associated with T.

## standard library
A **standard library** is the library made available across implementations of a programming language. It is often treated as part of the language by its users, although the designers may have treated it as a separate entity. The line between a language and its libraries differs between languages and it is not always clear. Some languages are designed so that the meanings of certain syntactic constructs cannot even be described without referring to the core library.


## Undefined behavior
**Undefined behavior** (UB) is the result of executing computer code whose behavior is not prescribed by the language specification.

## optimizing compiler
An **optimizing compiler** is a compiler that tries to minimize or maximize some attributes of an executable computer program. The most common requirement is to minimize the time taken to execute a program; a less common one is to minimize the amount of memory occupied.

## Intermediate representation (IR)
An **Intermediate representation (IR)** is the data structure or code used internally by a compiler or virtual machine to represent source code. An IR is designed to be conducive for further processing, such as optimization and translation. IR must be accurate - capable of representing the source code without loss of information, and independent of any particular source or target language. An IR may take one of several forms: an in-memory data structure, or a special tuple-based or stack-based code readable by the program. In the latter case it is also called an intermediate language.

## object code
In a general sense **object code** is a compiler produced sequence of statements or instructions in a computer language, usually a machine code language or an intermediate language such as register transfer language (RTL).





https://www.wikiwand.com/en/Backus%E2%80%93Naur_form
https://www.wikiwand.com/en/Side_effect_(computer_science)
https://www.wikiwand.com/en/Statement_(computer_science)
https://www.wikiwand.com/en/Primitive_data_type
https://www.wikiwand.com/en/Complex_data_type
https://www.wikiwand.com/en/Expression_(computer_science)
https://www.wikiwand.com/en/Syntax_(programming_languages)
https://www.wikiwand.com/en/Programming_language_theory
https://www.wikiwand.com/en/Programming_language
https://www.wikiwand.com/en/Parametric_polymorphism
https://www.wikiwand.com/en/Ad_hoc_polymorphism
https://www.wikiwand.com/en/Value_(computer_science)
https://www.wikiwand.com/en/Semantics_(computer_science)
https://www.wikiwand.com/en/Domain_theory
https://www.wikiwand.com/en/Partially_ordered_set
https://www.wikiwand.com/en/Order_theory
https://www.wikiwand.com/en/Exception_handling

https://www.wikiwand.com/en/Partial_equivalence_relation
https://www.wikiwand.com/en/Equivalence_relation
https://www.wikiwand.com/en/Partially_ordered_set
https://www.wikiwand.com/en/Equivalence_class

