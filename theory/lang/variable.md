# Variable

Properties
- Name, Identifier, Aliases
- Type
- Scope
- Lifetime
- Memory location (stack, heap)
- Mutability
- Privacy


A variable is a memory address paired with an associated symbolic name (identifier), which contains some known or unknown quantity of information referred to as a _value_.

name
Depending on the context, the name of variable can refer to the stored value, but also to the variable itself. This separation of name and content allows the name to be used independently of the exact information it represents. The identifier in source code can be bound to a value during run time, and the value of the variable may thus change during the program's execution.

A variable's memory location may be referred by several different identifiers, a situation known as _aliasing_. Assigning a value to the variable using one of the identifiers will change the value that can be accessed through the other identifiers.


Compilers have to replace variables' symbolic names with the actual locations of the data. While a variable's name, type, and location often remain fixed, the data stored in the location may be changed during program execution.


Variables come in different shapes: global, local, constant, static, instance, temporary, etc. Arguments and parameters.

Variables also provide a way of labeling data with a descriptive name.

A variable's scope determines where in a program a variable is available for use. A variable's scope is defined by where the variable is initialized or created.
