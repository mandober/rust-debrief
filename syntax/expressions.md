# Expressions

<!-- TOC -->

- [Debrief](#debrief)
- [Expression](#expression)
- [Place and Value Expressions](#place-and-value-expressions)
- [Move and Copy types](#move-and-copy-types)
- [Mutability](#mutability)
- [Temporary lifetimes](#temporary-lifetimes)
- [Implicit Borrows](#implicit-borrows)
- [Constant expressions](#constant-expressions)

<!-- /TOC -->

## Debrief
- Expression may have two roles:
  - it always produces a value
  - it may have side effects
- Expressions have two main categories:
  - __place expression__ (lvalue): represents a memory location
  - __value expression__ (rvalue): represents an actual value
- Within each expression, sub-expressions may occur in:
  - place expression context
  - value expression context
- The evaluation of an expression depends on:
  - its category (place expr or value expr)
  - the context it occurs within (place or value expr context)
- _Place expressions_ are paths which refer to:
  - local variables
  - static variables
  - dereferences, `*expr`
  - array indexing expressions, `expr[expr]`
  - field access, `expr.f`
  - parenthesized place expressions
- _Value expressions_: all other expressions
- __Place expression contexts__:
  - left operand of (compound) assignment
  - single operand of a unary borrow
  - operand of any implicit borrow
  - discriminant or subject of a `match` expression
  - right side of a `let` statement
- __Value expression contexts__: all other expression contexts
- Only these place expressions may be moved out of:
  - variables which are not currently borrowed
  - temporary values
  - fields of a place expr, which can be moved out and doesn't impl `Drop`
  - result of derefing expr of `Box` type, which can be moved out of.

- Expressions:




## Expression
An expression may have two roles: 
- it always produces a value, and 
- it may have side effects.

An expression evaluates to a value, and may have side effects during evaluation. Many expressions contain sub-expressions (operands).

The meaning of each kind of expression dictates several things:
- Whether or not to evaluate the sub-expressions when evaluating the expression
- The order in which to evaluate the sub-expressions
- How to combine the sub-expr values to obtain the value of the expression

In this way, the structure of expressions dictates the structure of execution. Blocks are just another kind of expression, so blocks, statements, expressions, and blocks again can recursively nest inside each other to an arbitrary depth.


## Place and Value Expressions
Expressions are divided into two main categories:
- **place expression** (lvalue) represents a **memory location**.
- **value expression** (rvalue) represents an **actual value**.

Within each expression, sub-expressions may occur in:
- **place context**
- **value context**

The evaluation of an expression depends on:
- its _category_ (place or value expr)
- the _context_ it occurs within (place or value expr context)

Place expressions are paths which refer to:
- local variables
- static variables
- dereferences, `*expr`
- array indexing expressions, `expr[expr]`
- field references, `expr.f`
- parenthesized place expressions
All other expressions are value expressions.

Place expression context:
- left operand of an assignment or compound assignment expression
- right side of a `let` statement
- single operand of a unary borrow
- operand of any implicit borrow
- discriminant or subject of a `match` expression
All other expression contexts are value expression contexts.


```rust
// PLACE EXPRESSION CONTEXTS:

// left operand of (compound) assignment
place_exp_context += 8;

// right side of a `let` statement
let x = place_exp_context;

// single operand of a unary borrow
&place_exp_context;

// operand of any implicit borrow
place_exp_context.method();
// assuming `&self` in method signature

// subject of match expression
match place_exp_context { 
  // discriminant of match expression
  place_exp_context2 => ...,
}
```


## Move and Copy types

When a place expression is evaluated in a value expression context, or is bound by value in a pattern, it denotes the value held in that memory location.

```rust
let mut x = 8_u8;
let r = &mut x;
*r = 6;
```

If the type of that value implements `Copy`, then the value will be copied. In the remaining situations, if that type is `Sized`, then it may be possible to move the value.

Only these place expressions may be moved out of:
- Variables which are not currently borrowed
- Temporary values
- Fields of a place expr which can be moved out of and doesn't implement `Drop`.
- The result of dereferencing an expression with type `Box<T>` and that can also be moved out of.

Moving out of a place expression that evaluates to a local variable, the location is deinitialized and cannot be read from again until it is reinitialized.

In all other cases, trying to use a place expression in a value expression context is an error.


## Mutability
Place expression must be mutable in order to be:
- assigned to
- mutably borrowed
- implicitly mutably borrowed
- bound to a pattern containing ref mut

We call these **mutable place expressions**.
Other place expressions are called **immutable place expressions**.

These expressions can be mutable place expression contexts:
- Mutable variables, which are not currently borrowed.
- Mutable static items.
- Temporary values.
- Fields, this evaluates the subexpr in a mutable place expression context.
- Dereferences of a `*mut T` pointer.
- Dereference of a variable, or field of a variable, with type `&mut T`.   
  This is an exception to the requirement of the next rule.
- Dereferences of a type that implements `DerefMut`, which requires the value being dereferenced is evaluated in a mutable place expression context.
- Array indexing of a type that implements `DerefMut`, which evaluates the value being indexed, but not the index, in mutable place expression context.


## Temporary lifetimes

When using a value expression in most place expression contexts, a temporary unnamed memory location is created initialized to that value and the expression evaluates to that location instead, except if promoted to `'static`. 

Promotion of a value expression to a `'static` slot occurs when the expression could be written in a constant and then borrowed; then dereferencing that borrow where the expression was originally written, without changing the runtime behavior.

That is, the promoted expression can be evaluated at compile-time and the resulting value does not contain interior mutability or destructors (these properties are determined based on the value where possible, e.g. `&None` always has the type `&'static Option<_>`, as it contains nothing disallowed).

Otherwise, the lifetime of temporary values is typically the innermost enclosing statement; the tail expression of a block is considered part of the statement that encloses the block, or the condition expression or the loop conditional expression if the temporary is created in the condition expression of an if or in the loop conditional expression of a while expression.

However, when a temporary value expression is being created that is assigned into a `let` declaration, the temporary is created with the lifetime of the enclosing block instead, as using the enclosing let declaration would be a guaranteed error (since a pointer to the temporary would be stored into a variable, but the temporary would be freed before the variable could be used).

The compiler uses simple syntactic rules to decide which values are being assigned into a `let` binding, and therefore deserve a longer temporary lifetime.

Examples:

```rust
// 1
let x = foo(&temp());
// 2
let x = temp().foo()
```

1. The expression `temp()` is a value expression. As it is being borrowed, a temporary is created which will be freed after the innermost enclosing statement; in this case, the `let` declaration.

2. This is the same as the previous example, except that the value of `temp()` is being borrowed via autoref on a method-call. Here we are assuming that `foo()` is an `&self` method defined in some trait, say Foo. In other words, the expression `temp().foo()` is equivalent to `Foo::foo(&temp())`.

```rust
// 3
let x = if foo(&temp()) { bar() } else { baz() };
// 4
let x = if temp().must_run_bar { bar() } else { baz() };
```

3. The expression `temp()` is a value expression. As the temporary is created in the condition expression of an `if`, it will be freed at the end of the condition expression; in this example before the call to bar or baz is made.

4. Here we assume the type of `temp()` is a struct with a boolean field `must_run_bar`. As the previous example, the temporary corresponding to `temp()` will be freed at the end of the condition expression.

```rust
// 5
while foo(&temp()) { bar(); }
```

5. The temporary containing the return value from the call to `temp()` is created in the loop conditional expression. Hence it will be freed at the end of the loop conditional expression; in this example before the call to bar if the loop body is executed.

```rust
// 6
let x = &temp();
// 7
let x = SomeStruct { foo: &temp() };
// 8
let x = [ &temp() ];
```

6. Here, the same temporary is being assigned into `x`, rather than being passed as a parameter, and hence the temporary's lifetime is considered to be the enclosing block.

7. As in the previous case, the temporary is assigned into a struct which is then assigned into a binding, and hence it is given the lifetime of the enclosing block.

8. As in the previous case, the temporary is assigned into an array which is then assigned into a binding, and hence it is given the lifetime of the enclosing block.

```rust
// 9
let ref x = temp();
```

9. In this case, the temporary is created using a `ref` binding, but the result is the same: the lifetime is extended to the enclosing block.



## Implicit Borrows

Certain expressions will treat an expression as a place expression by implicitly borrowing it. For example, it is possible to compare two unsized [slices] for equality directly, because the `==` operator implicitly borrows it's operands:

```rust
let a: &[i32];
let b: &[i32];
// ...
*a == *b;
// Equivalent form:
::std::cmp::PartialEq::eq(&*a, &*b);
```

Implicit borrows may be taken in the following expressions:
- Left operand in method-call expressions.
- Left operand in field expressions.
- Left operand in call expressions.
- Left operand in array indexing expressions.
- Operand of the dereference operator, `*`.
- Operands of comparison.
- Left operands of the compound assignment.


## Constant expressions

Certain types of expressions can be evaluated at compile time. These are called **constant expressions**. Certain places, such as in constants and statics, require a constant expression, and are always evaluated at compile time.

In other places, such as in `let` statements, constant expressions may be evaluated at compile time. If errors, such as out of bounds array indexing or overflow occurs, then it is a compiler error if the value must be evaluated at compile time, otherwise it is just a warning, but the code will most likely panic when run.

The following expressions are constant expressions, as long as any operands are also constant expressions and do not cause any `Drop::drop` calls to be ran:
- Literals
- Paths to fn and constants (recursively defining constants is not allowed)
- Tuple expressions
- Array expressions
- Struct expressions
- Enum variant expressions
- Block expressions, including unsafe blocks, which only contain items and possibly a constant tail expression
- Field expressions
- Index expressions, array indexing or slice with a usize
- Range expressions
- Closure expressions which don't capture variables from the environment
- Built in negation, arithmetic, logical, comparison or lazy boolean operators used on integer and floating point types, bool and char
- Shared borrows, except if applied to a type with interior mutability
- The dereference operator
- Grouped expressions
- Cast expressions, except pointer to address and fn pointer to address casts

