# Naming conventions

* underscore:
  - single underscore: pattern that matches anything, but doesn't bind the value

* snake_case:
  - variable identifier
  - function, method name
  - identifier that starts with an underscore supresses "unused" warning
  - primitive types
  - lifetime annotation

* SCREAMING_SNAKE_CASE:
  - constants
  - statics

* kebab-case:
  - crate names on cargo (auto translated in/from snake_case for `use` or `extern` statements)

* PascalCase:
  - types, newtypes, enum, struct
  - generic type parameters
  - trait names




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