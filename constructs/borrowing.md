# Borrowing

- Rust requires any references to freeze the referent and its owners.
- A reference cannot outlive its referent
- A mutable reference cannot be aliased



A variable is a storage location paired with an associated symbolic name (an identifier) that contains some quantity of information referred to as a value. 

A value is a sequence of bits together with its interpretation and some meta information, like alignment. 

In Rust, all types have an alignment specified in bytes. The alignment of a type specifies what addresses are valid to store the value at.

Storage location is a memory address on the stack or the heap.

A type represents a constraint placed upon the interpretation of data in a type system, describing representation, interpretation and structure of values stored in memory.



Simple values like scalars are entirely stored on the stack, while complex values are represented with a stack-stored pointer that points to some data allocated on the heap. Contrary to the heap, manipulating the stack is very cheap and efficient.

In Rust, Copy types are values stored exclusively on the stack, which can be duplicated simply by copying bits within the stack.

Value is a sequence of bits together with its interpretation.

`Copy` types: types whose values can be duplicated simply by copying bits.
By default, variable bindings have move semantics.
However, if a type implements `Copy`, it instead has 'copy semantics':


Primitive scalar values (numbers, characters and booleans), compound primitives (array, tuple) that containt only scalars and immutable kinds of references (raw pointers, refrences and function pointers).

implement Copy trait, making them copy types. 

Copy types:
- integers
- floats
- characters
- booleans
- array (if all elements are scalars)
- tuple (if all members are scalars)
- function pointers`fn`
- immutable references

