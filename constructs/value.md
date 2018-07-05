A value is a sequence of bits together with its interpretation.



The term _value type_ is commonly used to refer to one of two kinds of data types: *types of values* or *types of objects* with deep copy semantics.

Since the values underpinning value types are not stored, value types also do not include a notion of mutation.

A type that does determine constraints for storage in RAM is often called an _object_ type.


Programming languages that distinguish between value types and reference types typically offer a mechanism, called *boxing*, to wrap some or all of their value types in reference types. This permits the use of value types in contexts expecting reference types. The converse process (to unwrap the value type) is known as *unboxing*.


A *reference type* is a data type that refers to an object in memory. A *pointer type*, on the other hand, refers to a memory address. Reference types can be thought of as pointers that are implicitly dereferenced. The objects being referred to are dynamically allocated on the heap whereas value types are allocated automatically on the stack.

