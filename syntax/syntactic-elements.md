# Syntactic elements

<!-- TOC -->

- [Symbols](#symbols)
- [Arithmetic](#arithmetic)
- [Comparison](#comparison)
- [Bitwise](#bitwise)
- [Logical](#logical)
- [Patterns](#patterns)
- [Operators](#operators)
- [Literals](#literals)
- [Paths](#paths)
- [Generics](#generics)
- [Constraints](#constraints)
- [Parenthesis and tuples](#parenthesis-and-tuples)
- [Comments](#comments)
- [Macros and attributes](#macros-and-attributes)

<!-- /TOC -->


## Symbols

`::  ->  #  [  ]  (  )  {  }  ,  ;`

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
- `=`  assignment/eq.: `var = expr`

## Comparison
- `==` equality: `var == expr`. Trait `PartialEq`
- `!=` inequality: `var != expr`. Trait `PartialEq`
- `<`  less-than: `expr < expr`. Trait `PartialOrd`
- `>`  greater-than: `expr > expr`. Trait `PartialOrd`
- `<=` less-than or equal-to:`var <= expr` . Trait `PartialOrd`
- `>=` greater-than or equal-to: `var >= expr`. Trait `PartialOrd`


## Bitwise
* `|`   OR: `expr | expr`. Trait `BitOr`
* `^`   XOR: `expr ^ expr`. Trait `BitXor`
* `&`   AND: `expr & expr`. Trait `BitAnd`
* `!`   NOT (or logical complement): `!expr`. Trait `Not`
* `|=`  OR with assignment: `var |= expr`. Trait `BitOrAssign`
* `^=`  XOR with assignment: `var ^= expr`. Trait `BitXorAssign`
* `&=`  AND with assignment: `var &= expr`. Trait `BitAndAssign`
* `<<`  left-shift: `expr << expr`. Trait `Shl`
* `>>`  right-shift: `expr >> expr`. Trait `Shr`
* `<<=` left-shift with assignment: `var <<= expr`. Trait `ShlAssign`
* `>>=` right-shift with assignment: `var >>= expr`. Trait `ShrAssign`

## Logical
* `&&` logical and: `expr && expr`
* `||` logical or: `expr || expr`
* `!`  logical complement: `!expr`. Trait `Not`


## Patterns
* `..`  "and the rest": `variant(x, ..)`, `struct_type { x, .. }`
* `...` inclusive range: `expr...expr`
* `=>`  part of match arm syntax: `pat => expr`
* `|`   pattern alternatives: `pat | pat`
* `@`   pattern binding: `ident @ pat`
* `_`   "ignored" pattern (no actual binding), catch-all: `Some(_)`


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


## Literals

- `'ident` named lifetime: `'inner`
- `'ident` loop label: `'loop`
- numeric literal type suffix: `…u8`, `…i32`, `…f64`, `…usize`, …
- character literal: `'…'`
- string literal: `"…"`
- raw string literal: `r"…"`, `r#"…"#`, `r##"…"##`, …
- byte string literal, constructs a `[u8]` instead of a string: `b"…"`
- raw byte string literal: `br"…"`, `br#"…"#`, `br##"…"##`, …
- ASCII byte literal: `b'…'`
- `|…| expr` closure: `|x, y| x + y`
- `_` decorative number separator: `1_000_usize`
- `!` never type (see diverging functions)
- `{…}`: block expression.
- `Type {…}`: `struct` literal.
- `[…]`: array literal.
- `[expr; len]`: array literal containing `len` copies of `expr`.
- `[type; len]`: array type containing `len` instances of `type`.
- `expr[expr]`: collection indexing. trait:`Index`, `IndexMut`
- `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`: collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, `RangeFull` as the "index".


## Paths
* `ident::ident` path: `std::mem`
* `::path`: absolute path
* `self::path`: path relative to the current module
* `super::path`: path relative to the parent of the current module
* `type::ident`, `<type as trait>::ident`: associated constants, fns, types
* `<type>::…`: associated item for a type which cannot be directly named
* `trait::method(…)`: disambiguating a method call by naming the trait
* `type::method(…)`: disambiguating a method call by naming the type
* `<type as trait>::method(…)`: disambiguating a method call


## Generics
* `path<…>` Generic Type Parameter (GTP): `Vec<u8>`
* `path::<…>`, `method::<…>`: GTP in fn: `"42".parse::<i32>()`
* `fn ident<…> …`: define generic function.
* `struct ident<…> …`: define generic structure.
* `enum ident<…> …`: define generic enumeration.
* `impl<…> …`: define generic implementation.
* `for<…> type`: higher-ranked lifetime bounds.
* `type<ident=type>` GTP where one or more associated types have specific assignments: `Iterator<Item=T>`
- `_` inferred (inferable) part of type annotation: `Vec<_>`


## Constraints
* `T: U`: generic `T` constrained to types that implement `U`.
* `T: 'a`: generic `T` must outlive `'a`, it cannot transitively contain any references with lifetimes shorter than `'a`.
* `T : 'static`: generic `T` contains no borrowed refs other than `'static`
* `'b: 'a`: generic lifetime `'b` must outlive lifetime `'a`.
* `T: ?Sized`: allow generic type param to be dynamically-sized type.
* `'a + trait`, `trait + trait`: compound type constraint.


## Parenthesis and tuples
* `()`: empty tuple, unit type; both literal and type
* `(expr)`: parenthesized expression: : `let t = (5 + 3) * 8`
* `(expr,)`: single-element tuple expression: `let t = (5,)`
* `(type,)`: single-element tuple type: `(usize,)`
* `(expr, …)`: tuple expression
* `(type, …)`: tuple type
* `expr(expr, …)`: fn call expr. Also used to initialize tuple structs and tuple enum variants
* `ident!(…)`, `ident!{…}`, `ident![…]`: macro invocation
* `expr.0`, `expr.1`, …: tuple indexing


## Comments
* `//`: line comment.
* `//!`: inner line doc comment.
* `///`: outer line doc comment.
* `/*…*/`: block comment.
* `/*!…*/`: inner block doc comment.
* `/**…*/`: outer block doc comment.


## Macros and attributes
* `#[meta]`: outer attribute.
* `#![meta]`: inner attribute.
* `$ident`: macro substitution.
* `$ident:kind`: macro capture.
* `$(…)…`: macro repetition.
