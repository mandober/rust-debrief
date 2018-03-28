# Declaration and Definition


## Declaration
A declaration is a language construct that specifies the _kind_ (the kind of entity being declared, e.g. variable, function, etc.) and the _properties_ of an identifier (type, visibility, etc.).

A declaration introduces a new name into the environment (current scope). Declarations are mandatory in Rust and every entity must be eventually declared, meaning that __forward declarations__ are not necessary, e.g. declaring a struct before referring to it.


Declarations are __explicit__ (`let`, `const`, `static`, `fn`, etc.), although they can be __implicit__ in a few well-defined places such as function's parameters and a `for` loop iterator.

The kind of an identifier (declarable entities):
- module (`mod`)
- function (`fn`)
- variable (`let`)
- constant (`const`)
- static (`static`)
- enumeration (`enum`)
- structure (`struct`)
- union (`union`)
- trait (`trait`)
- type alias (`type`)
- name imports (`crate`)
- name imports (`use`)


Importing names with `crate` and `use` makes entities (that were defined elsewhere) available in the current namespace; this doesn't really create new names on the crate level, but it does introduce new names in the current environment (module or function), so these two might as well be considered a declaration, especially the `use` statement which can also create name aliases by importing an entity and making it available under a different name.



The properties of an identifier include
- type (variables, constants, statics), type signature (functions)
- lifetime (e.g. `'static`)
- visibility/privacy modifier (`pub`)
- mutability (`mut`)
- generic type parameters
- ABI
- extern modifier
- safety modifier
- etc.

Not all entities support all the properties. Apart from variables, all the other entities support visibility modifier.


## Definition
Declaration alone doesn't allocate any resources, but a declaration followed by initialization may (e.g. defining an empty vector won't cause any allocation). Such a construct is called __definition__. For example, when a declaration of a variable also specifies its value it is a variable definition; a function declaration only specifies its signature (input types, output type, visibility modifier, ABI, extern and safety modifiers), but a function definition additionally specifies its body.

Not all languages make distinction between declaration and definition: in many languages declarations always include a definition, and may be referred to as either "declarations" or "definitions". However, this distinction is clear in languages where interface and implementation are separated: the interface contains declarations, the implementation contains definitions.

In fact, in Rust, only variables and functions have both, declaration and definition; all the other entities have only definition; moreover, only a function used inside a trait definition can avoid specifying its body.
