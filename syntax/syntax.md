# Syntactic notations

<!-- TOC -->

- [Symbols](#symbols)
- [Operators and Symbols](#operators-and-symbols)
- [List](#list)
- [Misc](#misc)
- [Paths](#paths)
- [Generics](#generics)
- [Constraints](#constraints)
- [Macros and attributes](#macros-and-attributes)
- [Special types](#special-types)
- [Parenthesis and tuples](#parenthesis-and-tuples)
- [Notations](#notations)
- [Comments](#comments)

<!-- /TOC -->



## Symbols

```
::  ->  #  [  ]  (  )  {  }  ,  ;
```

## Operators and Symbols

```
! != % %=
```

- `!` denotes macro expansion



## List
* `-` (`- expr`): negation. unary. overloadable:`Neg`
* `+` (`expr + expr`): arithmetic addition. trait:`Add`
* `+=` (`var += expr`): addition and assignment. trait:`AddAssign`
* `-` (`expr - expr`): arithmetic subtraction. trait:`Sub`
* `-=` (`var -= expr`): subtraction and assignment. trait:`SubAssign`
* `*` (`expr * expr`): multiplication. trait:`Mul`
* `*=` (`var *= expr`): multiplication and assignment. trait:`MulAssign`
* `/` (`expr / expr`): arithmetic division. trait:`Div`
* `/=` (`var /= expr`): arithmetic division & assignment. trait:`DivAssign`
* `%` (`expr % expr`): remainder. trait:`Rem`
* `%=` (`var %= expr`): remainder and assignment. trait:`RemAssign`
* `=` (`var = expr`, `ident = type`): assignment/equivalence.
* `!=` (`var != expr`): non-equality comparison. trait: `PartialEq`
* `<` (`expr < expr`): less-than comparison. trait:`PartialOrd`
* `<=` (`var <= expr`): less-than or equal-to comparison. trait:`PartialOrd`
* `==` (`var == expr`): equality comparison. trait:`PartialEq`
* `>` (`expr > expr`): greater-than comparison. trait:`PartialOrd`
* `>=` (`var >= expr`): greater-than or equal-to comparison. trait:`PartialOrd`
* `^` (`expr ^ expr`): bitwise exclusive or. trait:`BitXor`
* `^=` (`var ^= expr`): bitwise exclusive or & assignment. trait:`BitXorAssign`
* `|` (`expr | expr`): bitwise or. trait:`BitOr`
* `|=` (`var |= expr`): bitwise or & assignment. trait:`BitOrAssign`
* `!` (`!expr`): bitwise or logical complement. trait: `Not`
* `&` (`expr & expr`): bitwise and. trait:`BitAnd`
* `&=` (`var &= expr`): bitwise and & assignment. trait:`BitAndAssign`
* `<<` (`expr << expr`): left-shift. trait:`Shl`
* `<<=` (`var <<= expr`): left-shift & assignment. trait:`ShlAssign`
* `>>` (`expr >> expr`): right-shift. trait:`Shr`
* `>>=` (`var >>= expr`): right-shift & assignment. trait:`ShrAssign`
* `&&` (`expr && expr`): logical and.
* `||` (`expr || expr`): logical or.
* `..` (`variant(x, ..)`, `struct_type { x, .. }`): rest pattern binding
* `...` (`expr...expr`) inclusive range in a match pattern.
* `=>` (`pat => expr`): part of match arm syntax.
* `|` (`pat | pat`): pattern alternatives.
* `@` (`ident @ pat`): pattern binding.
* `_`: "ignored" pattern binding. numbers separator.
* `!` (`ident!(…)`, `ident!{…}`, `ident![…]`): denotes macro expansion
* `&` (`&expr`, `&mut expr`): borrow.
* `&` (`&type`, `&mut type`, `&'a type`, `&'a mut type`): borrowed pointer type.
* `*` (`*expr`): dereference.
* `*` (`*const type`, `*mut type`): raw pointer.
* `+` (`trait + trait`, `'a + trait`): compound type constraint.
* `,`: argument and element separator.
* `->` (`fn(…) -> type`, `|…| -> type`): function and closure return type.
* `.` (`expr.ident`): member access.
* `..` (`..`, `expr..`, `..expr`, `expr..expr`): right-exclusive range literal.
* `..` (`..expr`): struct literal update syntax.
* `...` (`...expr`, `expr...expr`) in an expression: inclusive range
* `:` (`pat: type`, `ident: type`): constraints.
* `:` (`ident: expr`): struct field initializer.
* `:` (`'a: loop {…}`): loop label.
* `;`: statement and item terminator.
* `;` (`[…; len]`): part of fixed-size array syntax.
* `|` (`|…| expr`): closures.
* `?` (`expr?`): Error propagation. early return on `Err(_)`, otherwise unwraps


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
