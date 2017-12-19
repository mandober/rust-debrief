# Never type

Never is the bottom type i.e. the type that has no values. In type theory it is also called the zero or empty type, and is sometimes denoted with falsum, `⊥`.

A function whose return type is bottom type cannot return any value.

In subtyping systems, the bottom type is the subtype of all types. However, the converse is not true - a subtype of all types is not necessarily the bottom type. It is used to represent the return type of a function that does not return a value: for instance, one which loops forever, signals an exception, or exits.

Because the bottom type is used to indicate the lack of a normal return, it typically has no values. It contrasts with the top type, which spans all possible values in a system, and a unit type, which has exactly one value. The bottom type is sometimes confused with the so-called "void type", which is actually a unit type, albeit one with no defined operations.

In Rust, the bottom type is denoted by `!`. It is present in the type signature of functions guaranteed to never return, for example by calling `panic!()` or looping forever.
