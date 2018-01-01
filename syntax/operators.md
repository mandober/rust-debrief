# Operators

* Unary
  - Negation: `-`
  - Dereference: `*`
  - Logical negation: `!`
  - Borrowing: `&` and `&mut`
* Binary
  - Arithmetic: `+, -, *, /, %`
  - Bitwise: `&, |, ^, >>, <<`
  - Lazy boolean: `||`, `&&`
  - Comparison: `==, !=, >, >=, <, =<`
  - Type cast: `as`
  - Assignment: `=`
  - Compound assignment: `+=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=`



<!-- TOC -->

- [Unary operators](#unary-operators)
- [Binary operators](#binary-operators)
  - [Arithmetic operators](#arithmetic-operators)
  - [Bitwise operators](#bitwise-operators)
  - [Lazy boolean operators](#lazy-boolean-operators)
  - [Comparison operators](#comparison-operators)
  - [Type cast expressions](#type-cast-expressions)
  - [Assignment expressions](#assignment-expressions)
  - [Compound assignment expressions](#compound-assignment-expressions)
- [Operator precedence](#operator-precedence)
- [Links](#links)

<!-- /TOC -->


## Unary operators

* written as prefix
* Negation: `-`
  - Signed integers and floats support negation. 
  - There's no minus sign.
  - It is an error to apply negation to unsigned types.
* Dereference: `*`
  - When applied to a pointer, it denotes the pointed-to location.
  - For pointers to mut locations, the resulting value can be assigned to.
  - On non-pointer types, it calls the `deref` of `std::ops::Deref`, or the `deref_mut` of `std::ops::DerefMut` (if implemented by the type and required for an outer expression that will or could mutate the dereference), and produces the result of dereferencing the `&` or `&mut` borrowed pointer returned from the overload method.
* Logical negation: `!`
  - On the boolean type, this flips between true and false.
  - On ints, inverts the individual bits in the 2's complement.
* Borrowing: `&` and `&mut`
  - When applied to a value produces a ref to that value.
  - The value is then in the borrowed state for the duration of the ref:
    - shared: the value may not be mutated, but it may be read/shared again.
    - mutable: the value may not be accessed until the borrow ends.



## Binary operators
in order of operator precedence


### Arithmetic operators
- sugar for calls to traits defined in the `std::ops`
- overridable
- defaults:
  - Addition: `add` of `std::ops::Add` trait
  - Subtraction: `sub` of `std::ops::Sub` trait
  - Multiplication: `mul` of `std::ops::Mul` trait
  - Quotient: `div` of `std::ops::Div` trait
  - Remainder: `rem` of `std::ops::Rem` trait

### Bitwise operators
- sugar for calls to methods of built-in traits
- overridable
- Bitwise `&`, `|`, `^` applied to booleans are equivalent to logical  
  `&&`, `||` and `!=` evaluated eagerly.
- Defaults:
  - AND: `bitand` of `std::ops::BitAnd` trait
  - OR: `bitor` of `std::ops::BitOr` trait
  - XOR: `bitxor` of `std::ops::BitXor` trait
  - LSHIFT: `shl` of `std::ops::Shl` trait
  - RSHIFT: `shr` of `std::ops::Shr` trait

### Lazy boolean operators
- `||` and `&&` may be applied to booleans
- `||` denotes logical OR, `&&` denotes logical AND
- short-circuted

### Comparison operators
- sugar for calls to built-in traits
- overridable
- Defaults:
  - `==`: Equal to. `eq` on `std::cmp::PartialEq` trait.
  - `!=`: Unequal to. `ne` on `std::cmp::PartialEq` trait.
  - `<`: Less than. `lt` on `std::cmp::PartialOrd` trait.
  - `>`: Greater than. `gt` on `std::cmp::PartialOrd` trait.
  - `<=`: Less than or equal. `le` on `std::cmp::PartialOrd` trait.
  - `>=`: Greater than or equal. `ge` on `std::cmp::PartialOrd` trait.


### Type cast expressions
- binary operator `as`
- casts the LHS value to the RHS type
- Some of `as` conversions are implicit coercions    
  (argument passing, let assignment with an explicit type)
- Implicit conversions don't lose information, minimal or none side-effects

### Assignment expressions
- assignment expression is a pattern and an expression
- Evaluating it either copies or moves

### Compound assignment expressions
- `+, -, *, /, %, &, |, ^, <<, >>` operators may be combined with `=`
- such expressions have the unit type


## Operator precedence
- Operators at the same precedence level are evaluated left-to-right
- Unary operators have the same precedence
- Unary operators are stronger than binary


Operator           | Associativity
-------------------|--------------
`?`                |
`- * ! & &mut`     | Unary
`as` `:`           | left to right
`* / %`            | left to right
`+ -`              | left to right
`<< >>`            | left to right
`&`                | left to right
`^`                | left to right
`|`                | left to right
`== != < > <= >=`  | require parentheses
`&&`               | left to right
`||`               | left to right
`.. ...`           | require parentheses
`<-`               | right to left
`= += -= *= /= %=` | right to left
`&= |= ^= <<= >>=` |




## Links
- [`std::ops`](https://doc.rust-lang.org/std/ops/index.html)
- [operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
