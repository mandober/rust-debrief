# Type Inference

Rust uses a static type system, which means that types of variables, function arguments, structure fields, and so on must be known at compile time; the compiler will check that correct types are used everywhere. However, Rust also uses type inference, which allows the compiler to automatically deduce types based on how variables are used.

This is very convenient because you no longer need to explicitly state types, which in some cases may be cumbersome (or impossible) to write. 

Rust also considers future uses of a variable to deduce its type – not only the initializer.

Rust uses the widely known and thoroughly researched **Hindley-Milner** inference algorithm. This algorithm is most commonly used in functional programming languages. It can handle global type inference (inferring all types in an entire program, even the types of function arguments, returns, structure fields, etc.). Global type inference can be slow in large projects and can cause types to change with unrelated changes in the code base, thus Rust uses inference only for local variables.

You must explicitly write types when declaring functions, structs, enum, union, trait, type alias, implementation.

This strikes a good balance between expressiveness, speed, and robustness. Types also make good documentation for functions, methods, and structures.

