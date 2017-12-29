# Values

A value is a sequence of bits together with its interpretation.

The term value type is commonly used to refer to one of two kinds of data types:
*types of values* or *types of objects* with deep copy semantics.

Since a value is a sequence of bits together with its interpretation, a value type is a correspondence between a set of data and a set of abstract or concrete entities sharing characteristic attributes. The set of entities is sometimes called a species.

For example, a value type `i16` can establish the correspondence between a sequence of 16 bits and integer values from −32,768 to 32,767 through a "two's complement" representation.
Value types do not include constraints on how their values are stored; e.g, the type `i16` does not determine byte order, alignment, or even the number of 8-bit bytes used to store the 16 bits of the value type's representation.

Since the values underpinning value types are not stored, value types also do not include a notion of mutation.

A type that does determine constraints for storage in random-access memory is often called an *object type*.

Programming languages that distinguish between value types and reference types typically offer a mechanism, called *boxing*, to wrap some or all of their value types in reference types. This permits the use of value types in contexts expecting reference types. The converse process (to unwrap the value type) is known as *unboxing*.

A *reference type* is a data type that refers to an object in memory. A *pointer type*, on the other hand, refers to a memory address. Reference types can be thought of as pointers that are implicitly dereferenced. The objects being referred to are dynamically allocated on the heap whereas value types are allocated automatically on the stack.

