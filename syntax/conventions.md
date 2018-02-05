# Naming conventions

* snake_case: identifier, function/method name, primitives, lifetimes
* SCREAMING_SNAKE_CASE: constants, statics
* CamelCase: types (enum, struct, newtypes), generic type parameters, traits
* kebab-case: cargo crate names; auto translated in/from snake_case in `use` or `extern` statements



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