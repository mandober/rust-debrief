# Definition

Declaration alone doesn't allocate any resources, but a declaration followed by initialization may (e.g. defining an empty vector won't cause any allocation). Such a construct is called __definition__. For example, when a declaration of a variable also specifies its value it is a variable definition; a function declaration only specifies its signature (input types, output type, visibility modifier, ABI, extern and safety modifiers), but a function definition additionally specifies its body.

Not all languages make distinction between declaration and definition: in many languages declarations always include a definition, and may be referred to as either "declarations" or "definitions". However, this distinction is clear in languages where interface and implementation are separated: the interface contains declarations, the implementation contains definitions.

In fact, in Rust, only variables and functions have both, declaration and definition; all the other entities have only definition; moreover, only a function used inside a trait definition can avoid specifying its body.
