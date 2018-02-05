# Constants

- constant item is a named constant value which is not associated with a specific memory location in the program.
- constants are essentially inlined wherever they are used: they are copied directly into the relevant context.
- references to the same constant are not necessarily guaranteed to refer to the same memory address.
- constant values must not have destructors. 
- constant values permit most forms of data.
- constants may refer to the address of other constants, in which case the address will have elided lifetimes where applicable, otherwise (which is in most cases) defaults to the static lifetime.
- the compiler is still at liberty to translate the constant many times, so the address referred to may not be stable.
- constants must be explicitly typed. The type may be any type that doesn't implement `Drop` and has a `'static` lifetime: any references it contains must have static lifetimes.
- Rust 1.22.0 (2017-11-22): types that impl Drop are now allowed in const and static types.
- generally, constants should be preferred over statics, unless large data is being stored or single-address and mutability are required.




```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &'static str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```


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