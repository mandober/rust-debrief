# Statements

- A statement is a component of a block, which is a component of an outer expression or function.
- Rust has two kinds of statements:
  1. declaration statement
  2. expression statement
- Declaration statement introduces names into the enclosing statement block.
- Declared names may denote new variables or new items:
  - Declaring an item (fn, enum, struct, type, static, trait, impl, mod) within a statement block, just narrows its scope containing all of its uses. It is otherwise identical to an item declaration outside the statement block.
  - `let` statement introduces a new set of variables, given by a pattern.
  - expression statement evaluates an expression and ignores its result.



## Statements
Rust is primarily an expression language. This means that most forms of value-producing or effect-causing evaluation are directed by the uniform syntax category of expressions. Each kind of expression can typically nest within each other kind of expression, and rules for evaluation of expressions involve specifying both the value produced by the expression and the order in which its sub-expressions are themselves evaluated. In contrast, statements in Rust serve mostly to contain and explicitly sequence expression evaluation.

A statement is a component of a block, which is in turn a component of an outer expression or function.

Rust has two kinds of statements:
1. declaration statement
2. expression statement



## Declaration statements

A declaration statement is one that introduces one or more names into the enclosing statement block. The declared names may denote new variables or new items.

### Item declarations
An item declaration statement has a syntactic form identical to an item declaration within a module.

Declaring an item — a function, enumeration, struct, type, static, trait, impl or module — locally within a statement block is simply a way of restricting its scope to a narrow region containing all of its uses; it is otherwise identical in meaning to declaring the item outside the statement block.

Note: there is no implicit capture of the function's dynamic environment when declaring a function-local item.


#### let statements
A let statement introduces a new set of variables, given by a pattern. The pattern may be followed by a type annotation, and/or an initializer expression. When no type annotation is given, the compiler will infer the type, or signal an error if insufficient type information is available for definite inference. Any variable introduced by a variable declaration are visible from the point of declaration until the end of the enclosing block scope.


#### Expression statements
An expression statement is one that evaluates an expression and ignores its result. As a rule, an expression statement's purpose is to trigger the effects of evaluating its expression. An expression that consists of only a block expression or control flow expression, that doesn't end a block and evaluates to () can also be used as an expression statement by omitting the trailing semicolon.

```rust
v.pop();          // Ignore the element returned from pop

if v.is_empty() {
    v.push(5);
} else {
    v.remove(0);
}                 // Semicolon can be omitted.

[1];              // Separate expression statement, not an indexing expression.
```



## Scopes can return values

```rust
fn main() {
    let foo = 1;
    let bar = {
        // Shadows earlier declaration.
        let foo = 2;
        foo
    };
    println!("{}", foo);
    println!("{}", bar);
}
```

