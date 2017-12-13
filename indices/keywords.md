# Keywords

The keywords are reserved by the Rust language and may not be used as identifiers such as names of functions, variables, parameters, struct fields, modules, crates, constants, macros, static values, attributes, types, traits, or lifetimes.

Majority of keywords are strict - they can only be used in their correct contexts. For example, it is not allowed to declare a variable named "struct".

Small subset of keywords are "weak" - they have special meaning only in certain contexts. For example, it is possible to declare a variable or method with the name "union".

Language also reserves a set of keywords, either for future use or because they have been deprecated. Unstable keywords are a subset of reserved keywords that can be used today under a feature flag.

- `loop`     unconditional infinite loop
- `while`    conditional loop
- `continue` continue to the next loop iteration
- `break`    exit a loop immediately
- `for`      iterator loop
- `in`       part of for loop syntax
- `if`       conditional branching
- `else`     fallback for if and if let constructs
- `return`   return from function
- `true`     boolean true literal
- `false`    boolean false literal
- `let`      variable binding
- `ref`      by-reference binding
- `mut`      mutability qualifier
- `pub`      visibility qualifier in struct fields, impl blocks, modules
- `match`    pattern matching
- `move`     make a closure take ownership of all its captures
- `struct`   definition of structure
- `union`    definition of union; weak keyword
- `enum`     definition of enumeration
- `fn`       definition of function
- `trait`    definition of trait
- `impl`     trait implementation block
- `for`      part of trait impl syntax
- `as`       disambiguating specific trait containing an item
- `const`    constant items
- `static`   global variable
- `const`    constant raw pointer, `*const T`
- `static`   lifetime lasting the entire program execution, `'static`
- `fn`       function pointer type
- `impl`     inherent implementation block
- `self`     method subject
- `where`    type constraint clauses
- `box`      creates a Box
- `crate`    external crate linkage
- `crate`    macro variable representing the crate in which the macro is defined
- `extern`   external crate, function, and variable linkage
- `use`      import symbols into scope
- `as`       renaming items in use and extern crate statements
- `mod`      module declaration
- `self`     current module
- `super`    parent module of the current module
- `as`       primitive type casting
- `type`     type alias and associated type definition
- `Self`     type alias for the type implementing a trait
- `for`      higher-ranked lifetime syntax
- `unsafe`   denotes unsafe code block, functions, traits, and implementations
- `default`  [weak] use of default in front of impl block is deprecated.
- `'static`  [weak] keyword; static lifetime annotation.
- `dyn`      [weak] keyword
- `catch`    [weak, unstable] creates a new scope; use of `?` with `#![feature(catch_expr)]`
- `priv`     [reserved] visibility
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
- `yield`    [reserved] generators:`#![feature(generators,generator_trait)]`
- `macro`    [reserved]
- `proc`     [reserved]
