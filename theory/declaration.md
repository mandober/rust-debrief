# Declaration

A declaration is a language construct that specifies properties of an identifier: it declares what a identifier "means".

Declarations are most commonly used for functions, variables, constants, and classes, but can also be used for other entities such as enumerations and type definitions.

Beyond the name (the identifier itself) and the kind of entity (function, variable, etc.), declarations typically specify the data type (for variables and constants), or the type signature (for functions); types may also include dimensions, such as for arrays. 

A declaration is used to announce the existence of the entity to the compiler; this is important in those strongly typed languages that require functions, variables, and constants, and their types to be specified with a declaration before use, and is used in forward declaration.

Declaration is frequently contrasted with _definition_, but meaning and usage varies significantly between languages. A basic dichotomy is whether a declaration contains a definition or not. For example, when a declaration of a variable also specifies its value it is a definition of variable; whether a function declaration also specifies its body (function definition) or only its type signature (function declaration).

Not all languages make this distinction: in many languages declarations always include a definition, and may be referred to as either "declarations" or "definitions". However, this distinction is clear in languages that require declaration before use (forward declaration), and in languages where interface and implementation are separated: the interface contains declarations, the implementation contains definitions.
