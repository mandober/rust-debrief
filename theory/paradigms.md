# Programming Paradigms

Polymorphism
Parametric polymorphism
Interfaces
generics
monomorphism




*Polymorphism* is the provision of a single interface to entities of different types. *Interface* is used to define an abstract type that defines behaviors as method signatures. *Marker interfaces* contain no methods at all and serve to provide run-time information to generic processing using reflection. *Reflection* is the ability of a computer program to examine, introspect, and modify its own structure and behavior at runtime.

A *polymorphic type* is one whose operations can also be applied to values of some other type, or types. 
*Parametric polymorphism* allows code to be written generically, so that it can handle values uniformly without depending on their type. When code is written without mention of any specific type and thus can be used transparently with any number of new types, it is said to be *generic*. Parametric polymorphism is a way to make a language more expressive while still maintaining full static type-safety. A function that can evaluate to or be applied to values of different types is known as a polymorphic or generic function.
Sometimes a limit on types which can be used in generics (for example, in generic function) is needed and this can be achived with *bounded parametric polymorphism*. It requires types, in order to be applicable for use with generics, to have something in common, like belonging to the same type class or to implement a common behaviour.

Generic programming is a style of computer programming in which algorithms are written in terms of types to-be-specified-later that are then instantiated when needed for specific types provided as parameters. This approach permits writing common functions that differ only in the set of types on which they operate when used, thus reducing duplication.

algebraic data type is a kind of composite type, i.e. a type formed by combining other types. Two common classes of algebraic types are product types (tuples, records) and sum types, also called tagged or disjoint unions or variant types.
The values of a product type typically contain several values, called fields. All values of that type have the same combination of field types. The set of all possible values of a product type is the set-theoretic product, i.e., the Cartesian product, of the sets of all possible values of its field types. The values of a sum type are typically grouped into several classes, called variants. A value of a variant type is usually created with a quasi-functional entity called a constructor. Each variant has its own constructor, which takes a specified number of arguments with specified types. The set of all possible values of a sum type is the set-theoretic sum, i.e., the disjoint union, of the sets of all possible values of its variants. Enumerated types are a special case of sum types in which the constructors take no arguments, as exactly one value is defined for each constructor. Values of algebraic types are analyzed with pattern matching, which identifies a value by its constructor or field names and extracts the data it contains. Pattern matching on algebraic data types matches on the structural properties of an object rather than on the character sequence of strings.

an *invariant* is a condition that can be relied upon to be true during execution of a program, or during some portion of it. It is a logical assertion that is held to always be true during a certain phase of execution. For example, a loop invariant is a condition that is true at the beginning and end of every execution of a loop.
