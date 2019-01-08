# Binding

- Binding
- Late binding
- Early binding
- Static binding


[Chapter 3:: Names, Scopes, and Bindings - ppt video online download](http://slideplayer.com/slide/1507329/)

[Names and Bindings. - ppt video online download](http://slideplayer.com/slide/3520661/)

[hern University, Houston. - ppt download](http://slideplayer.com/slide/9198162/)

[Naming convention (programming) | Wikiwand](https://www.wikiwand.com/en/Naming_convention_(programming))

[Namespace | Wikiwand](https://www.wikiwand.com/en/Namespace)

[Name binding | Wikiwand](https://www.wikiwand.com/en/Name_binding)

[Function overloading | Wikiwand](https://www.wikiwand.com/en/Function_overloading)


---

Name binding is the association of entities (program components) with identifiers.

An identifier bound to an object is said to reference that object.

Machine languages have no built-in notion of identifiers, but name-object bindings as a service and notation for the programmer is implemented by programming languages.

Binding is intimately connected with scoping, as scope determines which names bind to which objects – at which locations in the program code (lexically) and in which one of the possible execution paths (temporally).

Use of an identifier` id` in a context that establishes a binding for `id` is called a _binding occurrence_ or _defining occurrence_. In all other occurrences (in expressions, assignments, function calls, etc.), an identifier stands for what it is bound to; such occurrences are called _applied occurrences_.

The binding of names before the program is run is called _static_ (or early) binding. Bindings performed as the program runs are _dynamic_ (or late or virtual) bindings.

Rebinding should not be confused with mutation: rebinding is a change to the referencing identifier, while mutation is a change to the referenced value. 

---

## Referencing Environment
A referencing environment is a set of active bindings at any point during programs execution. It corresponds to a sequence of scopes that can be examined in order to find the current binding for a given name. 

Shallow and Deep Bindings 
When the referencing environment of a routine is not created until the routine is usually called, it is late binding. The late binding of the referencing environment is known as shallow binding. If the environment is bound at the time the reference is first created, it is early binding. The early binding of the referencing environment is called deep binding. 

Shallow and Deep Bindings in Statically Scoped Language 
Shallow binding has never been implemented in any statically scoped language. Shallow bindings require more work by a compiler. Deep binding in a statically scoped languages is an obvious choice. 

Symbol Table
A symbol table is a dictionary that maps names to the information the compiler knows about them. It is used to keep track of the names in statically scoped program. Its most basic operations are insert (to put a new mapping) and lookup (to retrieve the binding information for a given name)
Static scope rules in most languages require that the referencing environment be different in different parts of the program. It is possible to implement a semantic analyzer such that new mappings are inserted at the beginning of the scope and removed at the end.

The Problems 
The straightforward approach to maintaining a referencing environment is not practical due to: –Nested scope: an inner binding must hide its outer binding. –Forward reference: names are sometimes used before they are declared. 

Multilevel Symbol Table
Most static scope rules can be handled by augmenting a simple symbol table to allow embedding symbol tables. When an inner scope is entered, the compiler executes the enter_scope operation. It executes the leave_scope operation when exits. 

Lookup Operation
When a lookup operation is initiated, the current symbol table is examined. If a given name is not found, an immediate outer symbol table is examined. This process is repeated until a binding is found for the name or an outermost symbol table is reached. 

---

## Classes of Binding Times (from late to early)
1. Execution Time (Late Binding).
  - Variables to their values.
  - Variables to a particular storage location (termed dynamic storage allocation).
  - At entry to a subprogram or block. Example: formal to actual parameters and formal parameters to actual locations.
  - At arbitrary points during execution. Example: variables to values. In some languages, variables to types. Consider Prolog - variable type is determined dynamically
2. Load time: globals bound to location
3. Link time: body of external function bound to call instruction
4. Compile time (Early Binding).
  - Bindings chosen by programmer. Variable names, types, names.
  - Bindings chosen by translator. Example: particular machine instruction for a statement. Example: initial values of variables (if none specified) Example: in C declaration defines type but gives no space
5. Language implementation time.
  Example: Association of enumerated type values with integers. Example: maxint
6. Language design-time
  probably the most important binding time. Example: structure of language fixed, set of all basic data types, set of statements: syntax and semantics fixed, predefined types.



---

## Binding
is the operation of associating things, like a name and the entity it represents. Binding time is the moment when the binding is performed and it is usually divided into early or static (at compilation time) and late or dynamic binding (at run time).
Polymorphism allowing a name (function, variable) to be bound to more than one entity.
Aliasing is multiple bindings for the same entity.

## Early/Late Binding
Early binding constrains the type of the variable. Late binding lets that be decided when a value is assigned.
Function known at compilation time, or left to be matched when the call is being executed.
Late binding waits until the value/data assigned to a variable is needed before evaluating or loading it.

## Dynamic Binding
The exact meaning of each identifier (variable/function) is determined when the instruction is executed based on context.
Example in Lisp: function A makes reference to a "global" variable x.
Function B declares a local variable x and then calls function A.
Within that function call, x is the local variable from B.

## Binding Time
Language design - fundamental aspects of the language, built-in functions, keywords.
Language implementation - details such as the size of each type, file representation, runtime exceptions.
Programming - algorithms, design of data structures.
Compilation - mapping between higher-level constructs and machine code, static data.
Linking - between function calls and external entities and their actual code.
Load - virtual addresses, dynamic libraries.
Runtime - virtual functions, values to variables, many more.

## Object Lifetime
Object - any entity in the program. Variables, functions.
Object lifetime - the period between the object creation and destruction.
Binding lifetime - the period between the creation and destruction of the binding. Usually the binding lifetime is a subset of the object lifetime.
Dangling reference - when the binding exists after the destruction of the object. Example: deleting a pointer but not making it NULL.
Leak memory - when an object still exists but there is no binding to it. Example: making a pointer NULL without deleting it first. Solved by garbage collection.

## Object Allocation
Static objects - they have an absolute address that exists for the duration of the program. Global variables, static local variables, runtime tables, function space for languages that don't support recursion, constants. Stack objects - Last in, first out (LIFO). Function space for languages that support recursion,Heap objects - can be allocated and deallocated at arbitrary times. Dynamically allocated parts of linked data structures, dynamically resized objects.


## Function Space
The stack of function calls contains a frame for each function. One frame contains: 
arguments, return values, local variables, elaboration-time constants
- temporaries: intermediate values produced in complex computations
- bookkeeping information: return address, reference to the calling frame, debugging information.


## Heap Management
There is usually a list of all the memory blocks not in use. When an allocation demand is made, the OS searches the heap for a free block of at least the requested size.
First fit - returning the first block that fits the request.
Best fit - returning the smallest block that fits the request.
Worst fit - the largest block, to avoid fragmenting the memory
Pool - dividing the list into sublists by size.
Compact - moving the allocating heaps closer together to create larger free blocks. When moving an object one needs to update all the references to it.


## Scope and Rules
Scope of a binding - the textual region of the program in which a binding is active. Scope - sometimes a region of a program of maximal size in which no binding changes scope. Referencing environment - the set of active bindings at any given point in the program execution. The scope of bindings is determined by binding rules, included in the description of the language.

## Scope of a Binding
Usually the scope of a binding is determined statically i.e. at compilation time. When a function is called that has a local variable, the binding between the variable name and the instance of the variable local to the call is created. Any previous bindings for that same variable name are deactivated in the process (or hidden). When the function call ends, the previous binding for the name is restored.

## Static Scopes
Static scope - when the scope of a binding is determined during compilation.
Sometimes called lexical scope. Current binding - the matching declaration whose block most closely surrounds the point in the program where the name is mentioned. Global scope - some languages only support global variables (Basic)
Local static scope - for languages that do not support recursion and for static variables in other.

## Nested Declarations
In Lisp the variables in a let are declared in parallel. In the above expression B is equal to 1. Usually in a nested declaration it is not possible to access the value of the previous binding. Exception: equivalences of memory like in Fortran, multiple references to the same objects. For classes it's possible to specify the scope of a function call (the :: operator in C++).

## Dynamic Scope
Bindings that are defined at runtime. They depend on the order in which functions are called. Current binding - the one encountered the most recently during the execution and that has not yet been destroyed. Type checking for dynamic scoping is done at runtime.

## Scope Implementation
Static scope relies on a symbol table which is a map or dictionary. For each symbol it contains information about it. 
Dynamic scope uses an association list or a central reference table. An association list is a pair name/value. For dynamic scoping, it is a list of symbols (name) associated with the scope (value). When new declarations are made, they are pushed into the list (which works as a stack). When a scope ends, it is popped out of the list. The binding is determined by a linear search in the list starting from the most recent definition going backwards.
