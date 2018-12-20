# Value

A value is a sequence of bits together with its interpretation.



The term **value type** is used to refer to one of 2 kinds of Data types
- types of values:
- types of objects with deep copy semantics



Since the values underpinning value types are not stored, value types also do not include a notion of mutation.

A type that does determine constraints for storage in RAM is often called an _object_ type.


Programming languages that distinguish between value types and reference types typically offer a mechanism, called *boxing*, to wrap some or all of their value types in reference types. This permits the use of value types in contexts expecting reference types. The converse process (to unwrap the value type) is known as *unboxing*.


A *reference type* is a data type that refers to an object in memory. A *pointer type*, on the other hand, refers to a memory address. Reference types can be thought of as pointers that are implicitly dereferenced. The objects being referred to are dynamically allocated on the heap whereas value types are allocated automatically on the stack.



## Value type
Since a value is a sequence of bits together with its interpretation, a value type is a correspondence between a set of data and a set of abstract or concrete entities sharing characteristic attributes. The set of entities is sometimes called a _species_.

For example, a value type `i16` can establish the correspondence between a sequence of 16 bits and integer values from −32,768 to 32,767 through a _two's complement_ representation.

Value types do not include constraints on how their values are stored; e.g, the type `i16` does not determine byte order, alignment, or even the number of 8-bit bytes used to store the 16 bits of the value type's representation.

Since the values, underpinning value types are not stored, value types also do not include a notion of mutation.


## Value object
Object type is a common name for a type that does determine constraints for its storage in RAM.

Languages that distinguish between value types and reference types typically offer a mechanism, known as *boxing*, to wrap some or all of their value types in reference types. This permits the use of value types in contexts expecting reference types. The converse process (to unwrap the value type) is known as *unboxing*.

A *reference type* is a data type that refers to an object in memory.

A *pointer type*, on the other hand, refers to a memory address.

Reference types are pointers that are implicitly dereferenced. The objects being referred to are dynamically allocated on the heap whereas value types are allocated automatically on the stack.


---

- Value: https://www.wikiwand.com/en/Value_(computer_science)
- Reference: https://www.wikiwand.com/en/Reference_(computer_science)
- Pointer: https://www.wikiwand.com/en/Pointer_(computer_programming)


# Value

https://www.wikiwand.com/en/Value_(computer_science)

A value is the representation of some entity that can be manipulated by a program. The members of a type are the values of that type.

The "value of a variable" is given by the corresponding mapping in the environment. In languages with _assignable variables_ it becomes necessary to distinguish between the _r-value_ (or contents) and the _l-value_ (or location) of a variable.

In imperative programming languages, values can generally be accessed or changed at any time. In pure functional and logic languages, variables are bound to expressions and keep a single value during their entire lifetime due to the requirements of referential transparency. In imperative languages, the same behavior is exhibited by (named) constants (symbolic constants), which are typically contrasted with (normal) variables. 

In declarative (high-level) languages, values have to be referentially transparent. This means that the resulting value is independent of the location in which a (sub)expression needed to compute the value is stored. Only the contents of the location (the bits, whether they are 1 or 0) and their interpretation are significant.


## Assignment: l-values and r-values

Some languages use the idea of l-values and r-values, deriving from the typical mode of evaluation on the left and right hand side of an assignment statement. An lvalue refers to an object that persists beyond a single expression. An rvalue is a temporary value that does not persist beyond the expression that uses it.

The notion of l-values and r-values was introduced by Combined Programming Language (CPL). The notions in an expression of r-value, l-value, and r-value/l-value are analogous to the parameter modes of input parameter (has a value), output parameter (can be assigned), and input/output parameter (has a value and can be assigned), though the technical details differ between contexts and languages.


## R-values and addresses

In many languages, notably the C family, l-values have storage addresses that are programmatically accessible to the running program (e.g. via address-of operator like "&" in C/C++), meaning that they are variables or dereferenced references to a certain memory location.

R-values can be "l-values" or "non-l-values" - a term only used to distinguish from l-values.

Consider the C expression 4 + 9. When executed, the computer generates an integer value of 13, but because the program has not explicitly designated where in the computer this 13 is stored, the expression is a non-l-value.

On the other hand, if a C program declares a variable x and assigns the value of 13 to x, then the expression x has a value of 13 and is an l-value.

In C, the term l-value originally meant something that could be assigned to (hence the name, indicating it is on the left side of the assignment operator), but since the reserved word const (constant) was added to the language, the term is now 'modifiable l-value'. In C++11 a special semantic-glyph && exists, to denote the use/access of the expression's address for the compiler only; i.e., the address cannot be retrieved using the address-of, &, operator during the run-time of the program (see the use of move semantics).

This type of reference can be applied to all r-values including non-l-values as well as l-values. Some processors provide one or more instructions which take an immediate value, sometimes referred to as "immediate" for short. An immediate value is stored as part of the instruction which employs it, usually to load into, add to, or subtract from, a register. The other parts of the instruction are the opcode, and destination. The latter may be implicit. (A non-immediate value may reside in a register, or be stored elsewhere in memory, requiring the instruction to contain a direct or indirect address [e.g., index register address] to the value.)

The l-value expression designates (refers to) an object. A non-modifiable l-value is addressable, but not assignable. A modifiable l-value allows the designated object to be changed as well as examined. An r-value is any expression, a non-l-value is any expression that is not an l-value. One example is an "immediate value" (look below) and consequently not addressable..



## In assembly language

A value can be virtually any kind of data by a given data type, for instance a string, a digit, a single letter.

Processors often support more than one size of immediate data, e.g. 8 or 16 bit, employing a unique opcode and mnemonic for each instruction variant. If a programmer supplies a data value that will not fit, the assembler issues an "Out of range" error message. Most assemblers allow an immediate value to be expressed as ASCII, decimal, hexadecimal, octal, or binary data. Thus, the ASCII character 'A' is the same as 65 or 0x41. The byte order of strings may differ between processors, depending on the assembler and computer architecture.

