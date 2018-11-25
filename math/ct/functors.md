# Functors

Functors, or **function objects**, are similar to function pointers, and can be used in similar ways. A functor is an object of a class type that implements the function-call operator, allowing the object to be used within expressions using the same syntax as a function call. Functors are more powerful than simple function pointers, being able to contain their own data values, and allowing the programmer to emulate closures. They are also used as callback functions if it is necessary to use a member function as a callback function.[4]

Many "pure" object-oriented languages do not support function pointers. Something similar can be implemented in these kinds of languages, though, using references to interfaces that define a single method (member function). CLI languages such as C# and Visual Basic .NET implement type-safe function pointers with delegates.

In other languages that support first-class functions, functions are regarded as data, and can be passed, returned, and created dynamically directly by other functions, eliminating the need for function pointers.

Extensively using function pointers to call functions may produce a slow-down for the code on modern processors, because branch predictor may not be able to figure out where to branch to (it depends on the value of the function pointer at run time) although this effect can be overstated as it is often amply compensated for by significantly reduced non-indexed table lookups.
