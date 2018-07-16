# Ownership

The concept of ownership enables Rust to maintain _memory safety_ without needing a _GC_.

Ownership, a _zero-cost abstraction_, is about managing memory _aliasing_.

Memory is managed through ownership rules that are enforces by the compiler:
- Every value has exactly one owner, a variable.
- When the owner goes out of scope, the value it owns is dropped.



Every value has exactly one owner, a variable.

When the owner (owning variable) goes out of scope, the value it owns is dropped.



A variable represents a named range of memory blocks.

Pointers contain a number that represents a memory location.

Every value has an owner - a variable is the value's owner.
There can only be one owner at a time.

A variable is a memory location paired with a symbolic name (an identifier) that contains some quantity of information referred to as a value.

The variable name is used to refer to the stored value;
this separation of name and content allows the name to be used independently of the exact information it represents.

