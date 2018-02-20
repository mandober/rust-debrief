# Syntactic notations

<!-- TOC -->

- [Arithmetic](#arithmetic)
- [Comparison](#comparison)
- [Bitwise](#bitwise)
- [Logical](#logical)
- [Patterns](#patterns)
- [Operators](#operators)
- [Misc](#misc)
- [Paths](#paths)
- [Generics](#generics)
- [Constraints](#constraints)
- [Macros and attributes](#macros-and-attributes)
- [Special types](#special-types)
- [Parenthesis and tuples](#parenthesis-and-tuples)
- [Notations](#notations)
- [Comments](#comments)
- [Symbols](#symbols)
- [Operators and remaining symbols](#operators-and-remaining-symbols)

<!-- /TOC -->


## Arithmetic
- `-`  negation: `- expr`. Trait `Neg`
- `+`  addition: `expr + expr`. Trait `Add`
- `/`  division: `expr / expr`. Trait `Div`
- `%`  remainder: `expr % expr`. Trait `Rem`
- `-`  subtraction: `expr - expr`. Trait `Sub`
- `*`  multiplication: `expr * expr`. Trait `Mul`
- `+=` addition and assignment: `var += expr`. Trait `AddAssign`
- `/=` division and assignment: `var /= expr`. Trait `DivAssign`
- `%=` remainder and assignment: `var %= expr`. Trait `RemAssign`
- `-=` subtraction and assignment: `var -= expr`. Trait `SubAssign`
- `*=` multiplication and assignment: `var *= expr`. Trait `MulAssign`
- `=`  assignment: `var = expr`

## Comparison
- `=` equality: `ident = type`
- `!=` (`var != expr`): non-equality comparison, `PartialEq`
- `<` (`expr < expr`): less-than comparison, `PartialOrd`
- `<=` (`var <= expr`): less-than or equal-to, `PartialOrd`
- `==` (`var == expr`): equality comparison, `PartialEq`
- `>` (`expr > expr`): greater-than comparison, `PartialOrd`
- `>=` (`var >= expr`): greater-than or equal-to, `PartialOrd`

## Bitwise
* `^` (`expr ^ expr`): bitwise exclusive or, `BitXor`
* `^=` (`var ^= expr`): bitwise exclusive or & assignment, `BitXorAssign`
* `|` (`expr | expr`): bitwise or, `BitOr`
* `|=` (`var |= expr`): bitwise or & assignment, `BitOrAssign`
* `!` (`!expr`): bitwise or logical complement,  `Not`
* `&` (`expr & expr`): bitwise and, `BitAnd`
* `&=` (`var &= expr`): bitwise and & assignment, `BitAndAssign`
* `<<` (`expr << expr`): left-shift, `Shl`
* `<<=` (`var <<= expr`): left-shift & assignment, `ShlAssign`
* `>>` (`expr >> expr`): right-shift, `Shr`
* `>>=` (`var >>= expr`): right-shift & assignment, `ShrAssign`

## Logical
* `&&` (`expr && expr`): logical and.
* `||` (`expr || expr`): logical or.


## Patterns
* `..` (`variant(x, ..)`, `struct_type { x, .. }`): "the rest" pattern
* `...` (`expr...expr`) inclusive range (in match).
* `=>` (`pat => expr`): part of match arm syntax.
* `|` (`pat | pat`): pattern alternatives.
* `@` (`ident @ pat`): pattern binding.
* `_`: "ignored" pattern (no actual binding). Number separator.


## Operators
- `!` macro expansion: `ident!(…)`, `ident!{…}`, `ident![…]`
- `&` borrow: `&expr`, `&mut expr`
- `&` borrowed pointer: `&T`, `&mut T`
- `&` borrowed pointer with a lifetime: `&'a T`, `&'a mut T`
- `*` dereference: `*expr`
- `*` raw pointer: `*const T`, `*mut T`
- `+` compound type constraint: `trait + trait`, `'a + trait`
- `,` arg and element separator.
- `->` return type: `fn(…) -> T`, `|…| -> T`
- `.` member access: `expr.ident`
- `..` right-exclusive range literal: `..`, `expr..`, `..expr`, `expr..expr`
- `..` struct literal update syntax: `..expr`
- `...` inclusive range: `...expr`, `expr...expr`
- `:` constraints: `pat: T`, `ident: T`
- `:` struct field initializer: `ident: expr`
- `:` loop label: `'a: loop {…}`
- `;` statement and item terminator
- `;` part of fixed-size array syntax: `[…; len]`
- `|` closures: `|…| expr`
- `?` Error propagation, `expr?`, early return on `Err(_)`, or unwraps


## Misc
* `'ident`: named lifetime or loop label.
* `…u8`, `…i32`, `…f64`, `…usize`, …: numeric literal of specific type.
* `"…"`: string literal.
* `r"…"`, `r#"…"#`, `r##"…"##`, …: raw string literal, escapes not processed.
* `b"…"`: byte string literal, constructs a `[u8]` instead of a string.
* `br"…"`, `br#"…"#`, `br##"…"##`, …: raw byte string literal
* `'…'`: character literal.
* `b'…'`: ASCII byte literal.
* `|…| expr`: closure.


## Paths
* `ident::ident`: path
* `::path`: absolute path
* `self::path`: path relative to the current module
* `super::path`: path relative to the parent of the current module
* `type::ident`, `<type as trait>::ident`: associated constants, fns, types
* `<type>::…`: associated item for a type which cannot be directly named
* `trait::method(…)`: disambiguating a method call by naming the trait
* `type::method(…)`: disambiguating a method call by naming the type
* `<type as trait>::method(…)`:disambiguating a method by naming trait and type


## Generics
* `path<…>`: type parameter, `Vec<u8>`
* `path::<…>`, `method::<…>`: specifies parameters to generic type, function, or method in an expression, `"42".parse::<i32>()`
* `fn ident<…> …`: define generic function.
* `struct ident<…> …`: define generic structure.
* `enum ident<…> …`: define generic enumeration.
* `impl<…> …`: define generic implementation.
* `for<…> type`: higher-ranked lifetime bounds.
* `type<ident=type>`: a generic type where one or more associated types have specific assignments, `Iterator<Item=T>`


## Constraints
* `T: U`: generic parameter `T` constrained to types that implement `U`.
* `T: 'a`: `T` must outlive `'a`, it cannot transitively contain any references with lifetimes shorter than `'a`.
* `T : 'static`: `T` contains no borrowed references other than `'static`
* `'b: 'a`: generic lifetime `'b` must outlive lifetime `'a`.
* `T: ?Sized`: allow generic type parameter to be a dynamically-sized type.
* `'a + trait`, `trait + trait`: compound type constraint.


## Macros and attributes
* `#[meta]`: outer attribute.
* `#![meta]`: inner attribute.
* `$ident`: macro substitution.
* `$ident:kind`: macro capture.
* `$(…)…`: macro repetition.


## Special types
* `!`: never type (diverging functions)


## Parenthesis and tuples
* `()`: empty tuple, unit type; both literal and type.
* `(expr)`: parenthesized expression.
* `(expr,)`: single-element tuple expression.
* `(type,)`: single-element tuple type.
* `(expr, …)`: tuple expression.
* `(type, …)`: tuple type.
* `expr(expr, …)`: function call expression. Also used to initialize tuple `struct`s and tuple `enum` variants.
* `ident!(…)`, `ident!{…}`, `ident![…]`: macro invocation.
* `expr.0`, `expr.1`, …: tuple indexing..


## Notations
* `{…}`: block expression.
* `Type {…}`: `struct` literal.
* `[…]`: array literal.
* `[expr; len]`: array literal containing `len` copies of `expr`.
* `[type; len]`: array type containing `len` instances of `type`.
* `expr[expr]`: collection indexing. trait:`Index`, `IndexMut`
* `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`: collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, `RangeFull` as the "index".


## Comments
* `//`: line comment.
* `//!`: inner line doc comment.
* `///`: outer line doc comment.
* `/*…*/`: block comment.
* `/*!…*/`: inner block doc comment.
* `/**…*/`: outer block doc comment.


## Symbols

```
::  ->  #  [  ]  (  )  {  }  ,  ;
```

## Operators and remaining symbols

```
! != % %=
```

- `!` denotes macro expansion

