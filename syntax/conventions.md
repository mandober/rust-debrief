# Naming conventions


## Styles
- single word in lowercase:
  - primitives: `bool`, `u8`, `char`
- snake_case: lowercase with underscore instead of space:
  - identifiers, function and method names, modules, etc.
  - lifetime parameters (usually only a single letter): `'a`, `'static`
- SCREAMING_SNAKE_CASE: uppercase with underscores instead of spaces:
  - constants, statics
- CamelCase (PascalCase): title case without spaces (camelCase is not used):
  - Non-primitives: `String`, `RefCell`, `BTreeMap` 
  - Generic type parameters (usually only a single letter): `T`
  - Traits
- kebab-case: lowercase with dash instead of space:
  - crates; auto translated in/from snake_case in `use` or `extern`


## Methods
Method names follow certain conventions

Suffix
- `_or` may return supplied argument
- `_or_else` may calculate return value from supplied param-less closure
- `_or_default` may return type's default value
- `_mut` gives mutable ref

Prefix
- `as_` Cheap conversion without consummation of original value
- `to_` Expensive conversion without consummation
- `into_` Variable cost with consummation




## Examples

```rust
// variable
let average_result: u16 = 56_650;

// constant
const MAX_POINTS: u32 = 100_000;

// static
static BOOK_NAME: &'static str = "Rust Debrief";

// primitive types
impl Collectable for i32 {}

// composite types
impl Collectable for Vec<T> {}

// newtype
struct JaggedEdge {}
```