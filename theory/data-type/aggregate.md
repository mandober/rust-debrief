# Aggregate

A data aggregate (or just aggregate) is a group of primitives that are logically contiguous in memory and that are viewed collectively as one datum (for instance, an aggregate could be 3 logically contiguous bytes, the values of which represent the 3 coordinates of a point in space). When an aggregate is entirely composed of the same type of primitive, the aggregate may be called an array; in a sense, a multi-byte word primitive is an array of bytes, and some programs use words in this way.

In the context of these definitions, a byte is the smallest primitive; each memory address specifies a different byte. The memory address of the initial byte of a datum is considered the memory address (or base memory address) of the entire datum.
