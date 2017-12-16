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
* Docs
  - [`std::ops`](https://doc.rust-lang.org/std/ops/index.html)
  - [operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html)



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

<!-- /TOC -->


## Unary operators
Unary operators are all written as prefix operators, before the expression:

- Negation: `-`   
  Signed integer types and floating-point types support negation. It is an error to apply negation to unsigned types.
- Dereference: `*`  
  When applied to a pointer, it denotes the pointed-to location. For pointers to mutable locations, the resulting value can be assigned to. On non-pointer types, it calls the `deref` method of the `std::ops::Deref` trait, or the `deref_mut` method of the `std::ops::DerefMut` trait (if implemented by the type and required for an outer expression that will or could mutate the dereference), and produces the result of dereferencing the `&` or `&mut` borrowed pointer returned from the overload method.
- Logical negation: `!`  
  On the boolean type, this flips between true and false. On integer types, this inverts the individual bits in the two’s complement representation of the value.
- Borrowing: `&` and `&mut`  
  When applied to a value, these operators produce a reference (pointer) to that value. The value is also placed into a borrowed state for the duration of the reference. For a shared borrow (`&`), this implies that the value may not be mutated, but it may be read or shared again. For a mutable borrow (`&mut`), the value may not be accessed in any way until the borrow expires.


## Binary operators
Binary operators expressions are given in order of operator precedence.

### Arithmetic operators
Syntactic sugar for calls to built-in traits, defined in the `std::ops` module of the std library. They are overridable. Defaults:

- Addition and array/string concatenation: `+`   
  Calls the `add` method on the `std::ops::Add` trait.
- Subtraction: `-`: `sub` on `std::ops::Sub` trait.
- Multiplication: `*`: `mul` on `std::ops::Mul` trait.
- Quotient: `/`: `div` on `std::ops::Div` trait.
- Remainder: `%`: `rem` on `std::ops::Rem` trait.


### Bitwise operators
Syntactic sugar for calls to methods of built-in traits. They are overridable. Bitwise `&`, `|` and `^` applied to boolean arguments are equivalent to logical `&&`, `||` and `!=` evaluated in non-lazy fashion. Defaults:

- AND: `&`. Calls the `bitand` method of the `std::ops::BitAnd` trait.
- OR: `|`. Calls the `bitor` method of the `std::ops::BitOr` trait.
- XOR: `^`. Calls the `bitxor` method of the `std::ops::BitXor` trait.
- LSHIFT: `<<`. Calls the `shl` method of the `std::ops::Shl` trait.
- RSHIFT: `>>`. Calls the `shr` method of the `std::ops::Shr` trait.


### Lazy boolean operators
The operators `||` and `&&` may be applied to operands of boolean type.  
The `||` operator denotes logical `OR`, and `&&` denotes logical `AND`.  
They are short-circuted.


### Comparison operators
Comparison operators are, like the arithmetic operators and bitwise operators, syntactic sugar for calls to built-in traits. They are overridable. Defaults:
- `==`: Equal to. Calls `eq` method on `std::cmp::PartialEq` trait.
- `!=`: Unequal to. Calls `ne` on `std::cmp::PartialEq` trait.
- `<`: Less than. Calls `lt` on `std::cmp::PartialOrd` trait.
- `>`: Greater than. Calls `gt` on `std::cmp::PartialOrd` trait.
- `<=`: Less than or equal. Calls `le` on `std::cmp::PartialOrd` trait.
- `>=`: Greater than or equal. Calls `ge` on `std::cmp::PartialOrd` trait.


### Type cast expressions
A type cast expression is denoted with the binary operator `as`. Executing an `as` expression casts the value on the left-hand side to the type on the right-hand side. Some of the conversions which can be done through the `as` operator can also be done implicitly at various points in the program, such as argument passing and assignment to a `let` binding with an explicit type. Implicit conversions are limited to "harmless" conversions that do not lose information and which have minimal or no risk of surprising side-effects on the dynamic execution semantics.


### Assignment expressions
An assignment expression consists of a pattern followed by an equals sign and an expression. Evaluating an assignment expression either copies or moves its right-hand operand to its left-hand operand.

### Compound assignment expressions
The +, -, *, /, %, &, |, ^, <<, and >> operators may be followed with the = operator. Any such expression always has the unit type.

### Operator precedence
The precedence of binary operators is ordered as follows, from strong to weak:

```
as :
* / %
+ -
<< >>
&
^
|
== != < > <= >=
&&
||
.. ...
<-
=
```

Operators at the same precedence level are evaluated left-to-right. Unary operators have the same precedence level and are stronger than any of the binary operators.
