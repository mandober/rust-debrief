# Grammer

<!-- TOC -->

- [Productions](#productions)
- [Modules](#modules)
- [Items](#items)
- [Paths](#paths)

<!-- /TOC -->


## Productions
- Tokens are primitive productions defined by regular, non-recursive, language.
- A literal is a compile-time evaluated expression that immediately and directly denotes the value it evaluates to.
- Symbols are printable structural tokens: `:: -> # [ ] ( ) { } , ;`
- An item is a component of a crate
- Items are organized within a crate by a nested set of modules.

## Modules
- Every crate has a single, outermost, _anonymous_ module.
- All further items in the crate have paths in the module tree of the crate.
- Items are entirely determined at compile-time
- They generally remain fixed during execution
- They may reside in read-only memory

## Items
- modules
- extern crate declarations
- use declarations
- function definitions
- type definitions
- struct definitions
- enumeration definitions
- union definitions
- constant items
- static items
- trait definitions
- implementations
- extern blocks


## Paths
- A path is a sequence of one or more path components logically separated by a namespace qualifier, `::`.
- If a path consists of only one component, it may refer to either an item or a variable in a local control scope.
- If a path has multiple components, it refers to an item.
- Path components are usually identifiers, but they may also include angle- bracket enclosed lists of type arguments.
- In expression context, the type argument list is given after a namespace qualifier in order to disambiguate it from a relational expression involving the less-than symbol, `<`.
- In type expression context, the final namespace qualifier is omitted.
- Paths starting with qualifier
  - `::` resolve from the crate root
  - `super` resolve relative to the parent module
  - `self` resolve relative to the current module
  - `super` can reoccur after `super` or `self` to refer to ancestor modules
