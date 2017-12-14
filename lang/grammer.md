# Rust Grammer

- token
- literal
- symbol
- unary operators
- binary operators
- keywords
- item
- path

**Tokens** are primitive productions in the grammar defined by regular, non-recursive, languages. A **literal** is an expression consisting of a single token, rather than a sequence of tokens, that immediately and directly denotes the value it evaluates to, rather than referring to it by name or some other evaluation rule. A literal is a form of constant expression, so is evaluated (primarily) at compile time. **Symbols** are a general class of printable tokens that play structural roles in a variety of grammar productions. They are a set of remaining miscellaneous printable tokens that do not otherwise appear as *unary operators*, *binary operators*, or *keywords*.

An **item** is a component of a crate. Items are organized within a crate by a nested set of modules. Every crate has a single "outermost" anonymous module; all further items within the crate have paths within the module tree of the crate. Items are entirely determined at compile-time, generally remain fixed during execution, and may reside in read-only memory. There are several kinds of items: modules, extern crate declarations, use declarations, function definitions, type definitions, struct definitions, enumeration definitions, union definitions, constant items, static items, trait definitions, implementations, extern blocks.

A **path** is a sequence of one or more path components logically separated by a namespace qualifier, `::`. If a path consists of only one component, it may refer to either an item or a variable in a local control scope. If a path has multiple components, it refers to an item.
Path components are usually identifiers, but they may also include angle-bracket-enclosed lists of type arguments. In expression context, the type argument list is given after a namespace qualifier in order to disambiguate it from a relational expression involving the less-than symbol, `<`. In type expression context, the final namespace qualifier is omitted.

Paths can be denoted with various leading qualifiers to change the meaning of how it is resolved:
- Paths starting with `::` are considered to be global paths where the 
  components of the path start being resolved from the crate root. Each identifier in the path must resolve to an item.
- Paths starting with the keyword `super` begin resolution relative to the
  parent module. Each further identifier must resolve to an item.
- Paths starting with the keyword self begin resolution relative to the current
  module. Each further identifier must resolve to an item.

Additionally keyword `super` may be repeated several times after the first `super` or `self` to refer to ancestor modules.