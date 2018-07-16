# Variables

A variable is a memory location paired with a symbolic name (an identifier) that contains some quantity of information referred to as a value.

The variable name is used to refer to the stored value; this separation of name and content allows the name to be used independently of the exact information it represents.

The "value of variable" is given by the corresponding mapping in the symbol table in the environment.

Variables and scope:- (A) Automatic variables: - Each local variable in a function comes into existence only when the function is called, and disappears when the function is exited. Such variables are known as automatic variables. (B) External variables: - These are variables that are external to a function on and can be accessed by name by any function. These variables remain in existence permanently; rather that appearing and disappearing as functions are called and exited, retain their values even after the functions that set them have returned.


## Values

<!-- git book file include -->
{% include "value.md" %}

<!-- markdown-preview-enhanced: import file 1 -->
@import "value.md"

<!-- markdown-preview-enhanced: import file 2 -->
<!-- @import "mutability.md" -->
