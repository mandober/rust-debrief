# Functions

A function definition consists of a block, along with a name and a set of parameters. Other than a name, all these are optional.

Functions are declared with the keyword `fn`. Functions may declare a set of input variables as parameters, through which the caller passes arguments into the function, and the output type of the value the function will return to its caller on completion.

When referred to, a *function definition* yields a *first-class value* of the corresponding zero-sized [function item type](function-item-type.md), which when called evaluates to a direct call to the function.

When referred to, a **function definition** yields a first-class value of the corresponding zero-sized *function item type*, which when called evaluates to a direct call to the function.

When referred to, a **function item** yields a zero-sized value of its *function item type*. That type explicitly identifies the function - its name, its type arguments, and its early-bound lifetime arguments; so the value does not need to contain an actual function pointer, and no indirection is needed when the function is called.





For example, this is a simple function:

```rust
fn answer_to_life_the_universe_and_everything() -> i32 {
    return 42;
}
```

As with let bindings, function arguments are *irrefutable patterns*, so any pattern that is valid in a let binding is also valid as an argument:

```rust
fn first((value, _): (i32, i32)) -> i32 {
    value
}
```

The block of a function is conceptually wrapped in a block that binds the argument patterns and then returns the value of the function's block. This means that the tail expression of the block, if evaluated, ends up being returned to the caller. As usual, an explicit return expression within the body of the function will short-cut that implicit return, if reached.

For example, the function above behaves as if it was written as:

```rust
// argument_0 is the actual first argument passed from the caller
let (value, _) = argument_0;
return {
    value
};
```

