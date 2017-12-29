# Constants

- constants are values that are bound to an name (identifier)
- their type must be annotated
- they are always immutable
- naming convention: upper case, separated with underscores
- their value can only be a constant expression: the compiler automatically substitutes their names with their value at compile-time
- *constants*
  - declared with `const` keyword
  - constants are more local in scope than statics
  - constants can be declared in any scope, including global
- *statics*
  - static, global, constants
  - declared with `static` keyword
  - statics live in the global scope; they are available in any scope.



# Constants

constants may only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
`const MAX_POINTS: u32 = 100_000`


Constants are valid for the entire time a program runs, within the scope they were declared in, making them a useful choice for values in your application domain that multiple parts of the program might need to know about.


```rust
use std::f32::consts; // to pull in consts::PI
static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";
// lifetime specifier: 'static
// is the longest possible lifetime
// such an object stays alive throughout the entire application
fn main() {
	const PI: f32 = 3.14;
	// use the PI value from the standard library:
	println!("{}", PI); // 3.14
	println!("{}", consts::PI); // 3.141593
}
```