# Type Systems

At the lowest level, **data** is just collection of bits without inherent structure, leaving it open to arbitrary interpretation; For instance, in an unconstrained access to data, a bit pattern representing an integer can be reinterpreted \(transmuted\) as a floating point number.

**Data typing** is classification of data into data types, which define the ways the data is intended to be used. A type defines the meaning of the data and the operations that can be taken on such data; it may also prescribe the way the data of that type should be stored. A **data type** represents a constraint placed upon the interpretation of data in a type system, describing representation, interpretation and structure of values stored in memory.

**Type system** is a set of rules that assigns a type to the value-carrying expressions of a language (variables, constants, objects, functions, etc.). The type system uses data type information to verify and enforce the constraints of data types through **type checking** process. Based on the time this process occurs, type systems can be divided into static and dynamic. **Statically typed** languages perform type checking at *compile-time*, as opposed to *run-time* checks that **dynamically typed** languages implement. 

In static languages, variables are classified into types: once a variable aquires a type, it is type-locked. It cannot change its type and it accepts bindings only to the values of the matching type. Moreover, in some languages, a variable cannot even be rebound to a different value, after its initial binding, although some languages permit this by declaring a variable mutable. In dynamic languages variables don't carry the type: a variable can change its binding and type freely, throughout program's execution, i.e. values of different types can be (re)assigned to a variable, and interpreter will manage all the typing. Type systems can also sit anywhere between the static and dynamic extremes; e.g. they can perform different type safety checks at both compile and run time.

**Type safety** is the extent to which a language discourages **type errors**, which result when operating on values that are not of the appropriate type. For example, if an integer is treated as a boolean, **weakly** (loosely) typed languages will try to deal with this discrepancy (possibly by performing implicit type conversion), while **strongly** typed languages will decisively emit a error and quit. Type errors caught at compile-time will prevent program from compiling, while those found at run-time will have unforeseen effects on the, already running, program.

In order to perform type checking, a type system requires that all values (expressions) have been designated with appropriate type. The process of setting a type, i.e. **typing**, may be dynamic (performed by compiler or interpreter), or static (done manually by the programmer), or it can be, to a various degree, split between the two.

**Type annotation** is an explicit identification of the data type by placing type identifiers directly on expressions in the source code. **Type identifier** directly refers to a named data type, with the same name as the identifier, but it can also refer to a type alias.

In contrast to type annotation, **type inference** is identification of data types, entirely performed by the compiler, where the type of expression is deduced from the context, at compile-time. The ability to infer types automatically makes many programming tasks easier, leaving the programmer free to omit type annotations while still permitting type checking.


Rust is a compiled, statically and strongly typed language with type inference that relieves programmer from annotating a fair amount of code.
