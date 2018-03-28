# Binding

Name binding is the association of entities (program components) with identifiers.

An identifier bound to an object is said to reference that object.

Machine languages have no built-in notion of identifiers, but name-object bindings as a service and notation for the programmer is implemented by programming languages.

Binding is intimately connected with scoping, as scope determines which names bind to which objects – at which locations in the program code (lexically) and in which one of the possible execution paths (temporally).

Use of an identifier` id` in a context that establishes a binding for `id` is called a _binding occurrence_ or _defining occurrence_. In all other occurrences (in expressions, assignments, function calls, etc.), an identifier stands for what it is bound to; such occurrences are called _applied occurrences_.

The binding of names before the program is run is called _static_ (or early) binding. Bindings performed as the program runs are _dynamic_ (or late or virtual) bindings.

Rebinding should not be confused with mutation: rebinding is a change to the referencing identifier, while mutation is a change to the referenced value. 
