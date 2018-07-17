# Variable

Variables are a means to access and manipulate memory.

A variable represents a named memory block that contains some data, a value.

A variable has an associated symbolic name, the identifier, which also serves to provide a descriptive name for labeling data.


Variable attributes and properties
- Name, Identifier, Aliases
- Type
- Scope
- Lifetime
- Memory location (stack, heap)
- Mutability
- Privacy

Variable types: 
- global
- local
- constant
- static
- instance
- temporary
- argument
- parameter



name
Depending on the context, the name of variable can refer to the stored value, but also to the variable itself. This separation of name and content allows the name to be used independently of the exact information it represents. The identifier in source code can be bound to a value during run time, and the value of the variable may thus change during the program's execution.

A variable's memory location may be referred by several different identifiers, a situation known as _aliasing_. Assigning a value to the variable using one of the identifiers will change the value that can be accessed through the other identifiers.


A compiler replaces variable's names with the actual memory address(es) where the data is stored.

While a variable's name, type, and location often remain fixed, the data stored in the location may be changed during program execution.


A variable's scope determines where in a program a variable is available for use. A variable's scope is defined by where the variable is initialized or created.
