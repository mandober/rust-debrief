# fmt module

- Utilities for formatting and printing Strings
- Online doc: [`std::fmt`](https://doc.rust-lang.org/std/fmt/)
- Contains the runtime support for `format!` macro
- `format!`is implemented in the compiler to emit calls to this module in order to format its args at runtime into strings.



## Usage

The `format!` macro's first argument is a format string i.e. a string literal as required by the compiler; it cannot be a variable. The compiler will parse the format string and determine if the list of args is fit to be passed to this format string.

Some examples of the `format!` extension are:

```rust
format!("Hello");                 // => "Hello"
format!("Hello, {}!", "world");   // => "Hello, world!"
format!("The number is {}", 1);   // => "The number is 1"
format!("{:?}", (3, 4));          // => "(3, 4)"
format!("{value}", value=4);      // => "4"
format!("{} {}", 1, 2);           // => "1 2"
format!("{:04}", 42);             // => "0042" with leading zeros
```



### Positional parameters

Each formatting argument is allowed to specify which value argument it's referencing, 
and if omitted it is assumed to be *the next argument*. 

For example, the format string `{} {} {}` would take three parameters, and they would be 
formatted in the same order as they're given. 
The format string `{2} {1} {0}`, however, would format arguments in reverse order.

Things can get a little tricky once you start intermingling the two types of positional 
specifiers. The "next argument" specifier can be thought of as an iterator over the 
argument. Each time a "next argument" specifier is seen, the iterator advances. 
This leads to behavior like this:
`format!("{1} {} {0} {}", 1, 2); // => "2 1 1 2"`

The internal iterator over the argument has not been advanced by the time the first `{}` 
is seen, so it prints the first argument. Then upon reaching the second `{}`, the 
iterator has advanced forward to the second argument. 

*Essentially, parameters which explicitly name their argument do not affect parameters 
which do not name an argument in terms of positional specifiers.*

A format string is required to use all of its arguments, otherwise it is a compile-time 
error. You may refer to the same argument more than once in the format string.


### Named parameters

Rust itself does not have a Python-like equivalent of named parameters to a function, 
but the `format!` macro is a syntax extension which allows it to leverage named parameters. 
Named parameters are listed at the end of the argument list and have the syntax:
`identifier = expression`

For example, the following format! expressions all use named argument:
```rust
format!("{argument}", argument = "test");   // => "test"
format!("{name} {}", 1, name = 2);          // => "2 1"
format!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b"
```

It is not valid to put positional parameters (those without names) after arguments which 
have names. Like with positional parameters, it is not valid to provide named parameters 
that are unused by the format string.


### Argument types

Each argument's type is dictated by the format string. 
There are various parameters which require a particular type, however. 
An example is the `{:.*}` syntax, which sets the number of decimal places in 
floating-point types:
`let formatted_number = format!("{:.*}", 2, 1.234567);`
`assert_eq!("1.23", formatted_number)`

> If this syntax is used, then the number of characters to print precedes the actual object 
> being formatted, and the number of characters must have the type `usize`.


### Formatting traits

When requesting that an argument be formatted with a particular type, you are actually 
requesting that an argument ascribes to a particular trait. This allows multiple actual 
types to be formatted via `{:x}` (like i8 as well as isize). 

The current mapping of types to traits is:
    nothing ⇒ Display
    ? ⇒ Debug
    o ⇒ Octal
    x ⇒ LowerHex
    X ⇒ UpperHex
    p ⇒ Pointer
    b ⇒ Binary
    e ⇒ LowerExp
    E ⇒ UpperExp

What this means is that any type of argument which implements the `fmt::Binary` trait 
can then be formatted with `{:b}`. Implementations are provided for these traits for a 
number of primitive types by the standard library as well. If no format is specified 
(as in `{}` or `{:6}`), then the format trait used is the `Display` trait.

When implementing a format trait for your own type, you will have to implement a method 
of the signature:
`fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {}`

...















