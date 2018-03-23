## Variable

A **variable** is a storage location (identified by a memory address), paired with an associated symbolic name (an identifier), which contains some known or unknown quantity of information referred to as a value.

Depending on the context, the name of variable can refer to the stored value, but also to the variable itself. This separation of name and content allows the name to be used independently of the exact information it represents. The identifier in source code can be bound to a value during run time, and the value of the variable may thus change during the program's execution.

A variable's storage location may be referred by several different identifiers, a situation known as **aliasing**. Assigning a value to the variable using one of the identifiers will change the value that can be accessed through the other identifiers.

Compilers have to replace variables' symbolic names with the actual locations of the data. While a variable's name, type, and location often remain fixed, the data stored in the location may be changed during program execution.



## Value

A value is the representation of some entity that can be manipulated by a program. The members of a type are the values of that type.

The "value of a variable" is given by the corresponding mapping in the environment. In languages with assignable variables it becomes necessary to distinguish between the r-value (or contents) and the l-value (or location) of a variable.

In imperative programming languages, values can generally be accessed or changed at any time. In pure functional and logic languages, variables are bound to expressions and keep a single value during their entire lifetime due to the requirements of referential transparency. In imperative languages, the same behavior is exhibited by (named) constants (symbolic constants), which are typically contrasted with (normal) variables. 

In declarative (high-level) languages, values have to be referentially transparent. This means that the resulting value is independent of the location in which a (sub)expression needed to compute the value is stored. Only the contents of the location (the bits, whether they are 1 or 0) and their interpretation are significant.

## Variable Properties

Name
Type
Value, default value
Scope
Lifetime
Privacy
Mutability
Storage (memory location)

Variables come in different shapes: global, local, constant, static, instance, temporary, etc. Arguments and parameters.

Variables also provide a way of labeling data with a descriptive name.


## Scope

A variable's scope determines where in a program a variable is available for use. A variable's scope is defined by where the variable is initialized or created.



---

https://www.wikiwand.com/en/Variable_(computer_science)
https://www.wikiwand.com/en/Value_(computer_science)
