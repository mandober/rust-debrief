# Type systems

At the lowest level, **data** is just a stream of bits without inherent structure, open to arbitrary interpretation. For example, in an unconstrained access to data, a bit pattern representing an integer can be reinterpreted as it were representing a character.

**Data typing** is classification of data into data types, which define the ways the data is intended to be used. A type defines the meaning of the data and the operations that can be taken on such data. It may also define the way the data of that type should be stored.

A **data type** is a constraint placed upon the interpretation of data in a type system, describing representation, interpretation and structure of values stored in memory.

**Type system** is a set of rules that assigns a type to the value-carrying expressions of a language (variables, constants, objects, functions, etc.). The type system uses data type information to verify and enforce the constraints of data types through **type checking** process. Type errors caught at compile time will prevent program from compiling, while errors found at run-time will have unknown effects on the, already running, program.


## Static vs dynamic

Based on the time type checking occurs, type systems can be divided into static and dynamic: languages with static type system perform type checking at compile-time, as opposed to run-time checks of dynamic languages. Type systems can also be to an extent static or dynamic as type checking tasks are split between compile and run time.

In **statically typed** languages, _variables carry the types_. Once a variable acquires a type, it is type-locked. It cannot change its type and it accepts bindings only to the values of the matching type. Moreover, pure functional and logic languages, forbid a variable to be rebound to a different value, after its initial binding; other languages (e.g. Rust) permit rebinding, provided a variable is declared mutable on its initialization.

In **dynamically typed** languages, _values carry the type_, not variables. A variable is free to change its binding and type throughout program's execution. Values of different type can be (re)assigned to a variable, and interpreter will manage all the typing. This freedom often allows the introduction of subtle bugs, so these languages usually have external tools that allows them to opt-in into type safety (e.g. TypeScript, Flow, etc. for JavaScript).

**Type safety** is the extent to which a language discourages **type errors**, which result when operating on values of inappropriate type. When this happens, **strongly typed** language will immediately emit an error, while a **weakly (loosely) typed** language will first try dealing with this discrepancy, before issuing an error. For example, when an integer is treated as a boolean, a weakly typed language (e.g. JavaScript) will implicitly convert it into boolean, producing a "false" value if it were a zero, or "true" otherwise. Implicitness is one of the defining characteristics of dynamic, while static languages prefer explicitness in general, possibly with various levels of strictness; when a strongly typed language condones an implicit action, it is usually a well defined and well-known exception, put in place merely to provide more comfortable syntax.


## Inference vs annotation

In order to perform type checking, a (static) type system requires that all values (expressions) were designated with an appropriate type. The process of setting a type may be dynamic (performed by compiler or interpreter), or static (done manually by the programmer), or, to a different extent, somewhere in between. **Type annotation** is an explicit identification of the data type by placing the type identifiers directly on values in the source code. Type identifier directly refers to a named data type, with the same name as the identifier, but it can also refer to a type alias.

In contrast to manual type annotation, **type inference** is identification of data types entirely performed by the compiler, where the type of value is deduced from the context at compile-time. The ability of language (i.e. its compiler) to infer types makes programming easier, allowing the programmer to omit type annotations while still enjoying the benefits of type checking.

