# Keywords

- strict
- weak
- reserved
  * unstable
  * future use
  * deprecated



- `true`     boolean true literal
- `false`    boolean false literal
- `const`    constant raw pointer, `*const T`
- `static`   lifetime lasting the entire program execution, `'static`
- `impl`     inherent implementation block
- `self`     method subject
- `where`    type constraint clauses
- `box`      creates a Box
- `crate`    macro variable: name of the crate where the macro is defined
- `as`       primitive type casting
- `type`     type alias and associated type definition
- `Self`     type alias for the type implementing a trait
- `for`      higher-ranked lifetime syntax

Flow
- `loop`     unconditional infinite loop
- `while`    conditional loop
- `continue` continue to the next loop iteration
- `break`    exit a loop immediately
- `for`      iterator loop
- `in`       part of for loop syntax
- `if`       conditional branching
- `else`     conditional branching fallback

Qualifiers
- `ref`      by-reference binding qualifier
- `mut`      mutability qualifier
- `pub`      visibility qualifier in struct fields, impl blocks, modules
- `move`     make a closure take ownership of all its captures

Patterns
- `let`       pattern matching, variable binding
- `match`     pattern matching
- `if let`    pattern matching
- `else`      fallback for `if let` constructs
- `while let` pattern matching


Functions
- `fn`       definition of function
- `return`   return from function
- `fn`       function pointer type
- `extern`   external function
- `unsafe`   denotes unsafe functions


Traits
- `trait`    definition of trait
- `impl`     trait implementation block
- `for`      part of trait impl syntax
- `as`       disambiguating specific trait containing an item

Items
- `struct`   definition of structure
- `union`    definition of union [weak]
- `enum`     definition of enumeration
- `const`    constant items
- `static`   global variable
- `crate`    external crate linkage
- `extern`   external crate and variable linkage
- `use`      import symbols into scope
- `unsafe`   denotes unsafe code block, traits, implementations
- `as`       renaming items in `use` and `extern crate` statements
- `mod`      module declaration
- `self`     current module
- `super`    parent module of the current module

Weak
- `default`  [weak] use of default in front of impl block is deprecated.
- `'static`  [weak] keyword; static lifetime annotation.
- `dyn`      [weak] keyword
- `catch` [weak, unstable] create new scope; `?` with `#![feature(catch_expr)]`

Reserved
- `priv`     visibility
- `yield`    generators:`#![feature(generators,generator_trait)]`
- `alignof`  [reserved]
- `offsetof` [reserved]
- `sizeof`   [reserved]
- `typeof`   [reserved]
- `abstract` [reserved]
- `final`    [reserved]
- `virtual`  [reserved]
- `unsized`  [reserved]
- `pure`     [reserved]
- `become`   [reserved]
- `do`       [reserved]
- `override` [reserved]
- `macro`    [reserved]
- `proc`     [reserved]
