# Declaration

<!-- TOC -->

- [Keywords](#keywords)
- [Name](#name)
- [Name binding](#name-binding)
- [Declaration](#declaration)
- [Definition](#definition)

<!-- /TOC -->


- identifier
- name
- declaration
- definition
- forward declaration
- explicit declaration
- implicit declaration


Declaration is a language construct that introduces a new identifier.

**Identifiers** are lexical tokens that give symbolic names to entities. This makes refering to an entity easier, by merely using its symbolic name.


Identifiers are tokens which name language entities.

Kinds of entites an identifier may denote include:
- variables
- functions
- types
- constants
- classes
- interfaces
- traits
- labels
- memory address
- resources (memory, file system, databases, networks, etc)
- keywords



## Keywords

In most languages, some character sequences have the lexical form of an identifier but are known as keywords.

For example, `if` is frequently a keyword for an `if` clause, but lexically is of the same form as `foo` i.e. just a sequence of letters.

This overlap can be handled in various ways: these may be forbidden from being identifiers – which simplifies tokenization and parsing – in which case they are reserved words; they may both be allowed but distinguished in other ways, such as via stropping; or keyword sequences may be allowed as identifiers and which sense is determined from context, which requires a context-sensitive lexer.

Non-keywords may also be reserved words (forbidden as identifiers), particularly for forward compatibility, in case a word may become a keyword in future.

The scope, or accessibility within a program of an identifier can be either local or global. A global identifier is declared outside of functions and is available throughout the program. A local identifier is declared within a specific function and only available within that function.

For implementations of programming languages that are using a compiler, identifiers are often only compile time entities. That is, at runtime the compiled program contains references to memory addresses and offsets rather than the textual identifier tokens (these memory addresses, or offsets, having been assigned by the compiler to each identifier).

In languages that support reflection, such as interactive evaluation of source code (using an interpreter or an incremental compiler), identifiers are also runtime entities, sometimes even as first-class objects that can be freely manipulated and evaluated. In Lisp, these are called symbols.

Compilers and interpreters do not usually assign any semantic meaning to an identifier based on the actual character sequence used. However, there are exceptions. For example:
- In Perl a variable is indicated using a prefix called a sigil, which specifies aspects of how the variable is interpreted in expressions.
- In Ruby a variable is automatically considered immutable if its identifier starts with a capital letter.
- In Fortran, the first letter in a variable's name indicates whether by default it is created as an integer or floating point variable.
- In Go, the capitalization of the first letter of a variable's name determines its visibility (uppercase for public, lowercase for private).
- In some languages such as Go, identifiers uniqueness is based on their spelling and their visibility.
- In HTML an identifier (`id`) is one of the possible attributes of an HTML element. It should be unique within the document.



## Name
A name, usually an identifier, is a character string used to represent something else. It allows for referring to entities in a program by a symbol instead of an address. It provides a level of abstraction in a program: classes for data abstraction, functions for control abstraction. A name give us a better focus on some aspects of a program by reducing the conceptual complexity of the code.


## Name binding 
In programming languages, name binding is the association of entities (data and/or code) with identifiers.[1] An identifier bound to an object is said to reference that object. Machine languages have no built-in notion of identifiers, but name-object bindings as a service and notation for the programmer is implemented by programming languages. Binding is intimately connected with scoping, as scope determines which names bind to which objects – at which locations in the program code (lexically) and in which one of the possible execution paths (temporally).

Use of an identifier id in a context that establishes a binding for id is called a binding (or defining) occurrence. In all other occurrences (e.g., in expressions, assignments, and subprogram calls), an identifier stands for what it is bound to; such occurrences are called applied occurrences.


## Declaration

Declaration is a language construct that introduces a new identifier.

specifies the kind of entity being declared (e.g. variable, function) and the properties of an identifier (type, visibility).


A declaration introduces a new name into the environment (current scope). Declarations are mandatory in Rust and every entity must be eventually declared, meaning that __forward declarations__ are not necessary, e.g. declaring a struct before referring to it.

Declarations are __explicit__ (`let`, `const`, `static`, `fn`, etc.), although they can be __implicit__ in a few well-defined places such as function's parameters and a `for` loop iterator.

The kind of an identifier (declarable entities):
- module (`mod`)
- function (`fn`)
- variable (`let`)
- constant (`const`)
- static (`static`)
- enumeration (`enum`)
- structure (`struct`)
- union (`union`)
- trait (`trait`)
- type alias (`type`)
- name imports (`crate`)
- name imports (`use`)


Importing names with `crate` and `use` makes entities (that were defined elsewhere) available in the current namespace; this doesn't really create new names on the crate level, but it does introduce new names in the current environment (module or function), so these two might as well be considered a declaration, especially the `use` statement which can also create name aliases by importing an entity and making it available under a different name.



The properties of an identifier include
- type (variables, constants, statics), type signature (functions)
- lifetime (e.g. `'static`)
- visibility/privacy modifier (`pub`)
- mutability (`mut`)
- generic type parameters
- ABI
- extern modifier
- safety modifier
- etc.

Not all entities support all the properties. Apart from variables, all the other entities support visibility modifier.

---

A declaration is a language construct that specifies properties of an identifier: it declares what a identifier "means".

Declarations are most commonly used for functions, variables, constants, and classes, but can also be used for other entities such as enumerations and type definitions.

Beyond the name (the identifier itself) and the kind of entity (function, variable, etc.), declarations typically specify the data type (for variables and constants), or the type signature (for functions); types may also include dimensions, such as for arrays. 

A declaration is used to announce the existence of the entity to the compiler; this is important in those strongly typed languages that require functions, variables, and constants, and their types to be specified with a declaration before use, and is used in forward declaration.

Declaration is frequently contrasted with _definition_, but meaning and usage varies significantly between languages. A basic dichotomy is whether a declaration contains a definition or not. For example, when a declaration of a variable also specifies its value it is a definition of variable; whether a function declaration also specifies its body (function definition) or only its type signature (function declaration).

Not all languages make this distinction: in many languages declarations always include a definition, and may be referred to as either "declarations" or "definitions". However, this distinction is clear in languages that require declaration before use (forward declaration), and in languages where interface and implementation are separated: the interface contains declarations, the implementation contains definitions.


## Definition

Declaration alone doesn't allocate any resources, but a declaration followed by initialization may (e.g. defining an empty vector won't cause any allocation). Such a construct is called definition. For example, when a declaration of a variable also specifies its value it is a variable definition; a function declaration only specifies its signature (input types, output type, visibility modifier, ABI, extern and safety modifiers), but a function definition additionally specifies its body.

Not all languages make distinction between declaration and definition: in many languages declarations always include a definition, and may be referred to as either "declarations" or "definitions". However, this distinction is clear in languages where interface and implementation are separated: the interface contains declarations, the implementation contains definitions.

In fact, in Rust, only variables and functions have both, declaration and definition; all the other entities have only definition; moreover, only a function used inside a trait definition can avoid specifying its body.
