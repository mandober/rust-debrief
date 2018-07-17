# Programming Paradigms


## Polymorphism
*Polymorphism* is the provision of a single interface to entities of different types. *Interface* is used to define an abstract type that defines behaviors as method signatures. 

## Polymorphic type
is one whose operations can also be applied to values of some other type, or types. 

## Parametric polymorphism
allows code to be written generically, so that it can handle values uniformly without depending on their type. When code is written without mention of any specific type and thus can be used transparently with any number of new types, it is said to be *generic*. Parametric polymorphism is a way to make a language more expressive while still maintaining full static type-safety. A function that can evaluate to or be applied to values of different types is known as a polymorphic or generic function.

Sometimes a limit on types which can be used in generics (for example, in generic function) is needed and this can be achived with *bounded parametric polymorphism*. It requires types, in order to be applicable for use with generics, to have something in common, like belonging to the same type class or to implement a common behaviour.

## Generic programming
is a style of computer programming in which algorithms are written in terms of types to-be-specified-later that are then instantiated when needed for specific types provided as parameters. This approach permits writing common functions that differ only in the set of types on which they operate when used, thus reducing duplication.


## Type classes

https://www.wikiwand.com/en/Type_class
http://okmij.org/ftp/Computation/typeclass.html

Type classes are a powerful tool used in functional programming to enable ad-hoc polymorphism, more commonly known as overloading. Where many object-oriented languages leverage subtyping for polymorphic code, functional programming tends towards a combination of parametric polymorphism (think type parameters, like Java generics) and ad-hoc polymorphism.

Rust supports traits, which are a limited form of type classes with coherence.

parametric overloading, also known as bounded polymorphism, or just `type classes'
