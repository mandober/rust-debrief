# Programming Paradigms


# Main Memory
- The main memory is a temporary storage device that holds both a program and the data it manipulates while the processor is executing the program.
- Physically, main memory consists of a collection of dynamic random access memory(DRAM) chips.
- Logically, memory is organized as a linear array of bytes, each with its own unique address (array index) starting at zero.
- In general, each of the machine instructions that constitute a program can consist of a variable number of bytes. 
- The sizes of data items that correspond to C program variables vary according to type. For example, on an x86-64 machine running Linux, data of type short require 2 bytes, types int and float 4 bytes, and types long and double 8 bytes.


# CPU
- The CPU is the engine that interprets and executes instructions.
- At its core is a word-size register called the program counter (PC). 
- At any point in time, the PC contains the address of some machine-language instruction in main memory.


> Example: Sandy Bridge architecture, Intel Core i5-2500K, 3.70 GHz
> - Level 1 cache size:
>   - L1i: 4 x 32 KB 8-way set associative INSTRUCTION caches
>   - L1d: 4 x 32 KB 8-way set associative DATA caches
>   - latency: 4, @3.3GHz approx. 1.21 ns
>   - per core (4 cores), 32768 B each
> - Level 2 cache size:
>   - 4 x 256 KB 8-way set associative caches
>   - latency: 11
>   - per core (4 cores)
> - Level 3 cache size:
>   - 6 MB 12-way set associative SHARED CACHE
>   - latency: 25
> Accessing instructions stored in the L1 would take 4 cycles or, which, at 3.3GHz, is approx. 1.21 ns.




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

