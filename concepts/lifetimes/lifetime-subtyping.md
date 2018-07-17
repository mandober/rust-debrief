# Lifetime subtyping

There are 3 advanced features of lifetimes:
- Lifetime subtyping
- Lifetime bounds
- Trait object lifetimes


Lifetime subtyping is a way to specify that one lifetime should outlive another lifetime.

```rust
struct Context(&str);

struct Parser {
  context: &Context,
}

impl Parser {
  fn parse(&self) -> Result<(), &str> {
    Err(&self.context.0[1..])
  }
}
```

To get this code compiling, we need to fill in the lifetime parameters for the string slice in Context and the reference to the Context in Parser. The most straightforward way to do this is to use the same lifetime everywhere:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
  context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
  fn parse(&self) -> Result<(), &str> {
    Err(&self.context.0[1..])
  }
}
```

This compiles fine, and tells Rust that a `Parser` holds a reference to a `Context` with lifetime `'a`, and that `Context` holds a string slice that also lives as long as the reference to the `Context` in `Parser`.

Now, let’s add a function that takes an instance of `Context`, uses a `Parser` to parse that context, and returns what `parse` returns.

```rust
fn parse_context(context: Context) -> Result<(), &str> {
  Parser { context: &context }.parse()
}
```

This doesn't work. The errors are saying that `Parser` instance and `context` param live only until the end of `parse_context`. They need to outlive the fn and be valid before the fn starts and after it ends; but here they go out of scope at the end of fn because fn takes ownership of `context`.

Let's examine the references in the signature of the parse method, with eliding and when it is expanded:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> { context: &'a Context<'a> }

impl<'a> Parser<'a> {
    // elided:
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
    // expanded:
    fn parse<'a>(&'a self) -> Result<(), &'a str> {}
}

fn parse_context(context: Context) -> Result<(), &str> {
  Parser { context: &context }.parse()
}
```

The error part of the return value of `parse` has a lifetime that is tied to the lifetime of the `Parser` instance (that of `&self`).

That is reasonable: the returned str references the str in the `Context` instance held by the `Parser`, and the definition of `Parser` struct specifies that the lifetime of the reference to `Context` and the lifetime of the string slice that `Context` holds should be the same.

The problem is that `parse_context` returns the value returned from `parse`, so the lifetime of the return value of `parse_context` is tied to the lifetime of the `Parser` as well.

But the `Parser` instance created in the `parse_context` won't live past the end of it (it's temporary), and `context` will go out of scope at the end of the fn (`parse_context` takes ownership of it).

Rust thinks we're trying to return a reference to a value that goes out of scope at the end of the function, because we annotated all the lifetimes with the same lifetime parameter.

That told Rust the lifetime of the string slice that `Context` holds is the same as that of the lifetime of the reference to `Context` that `Parser` holds.

The `parse_context` function can't see that within the `parse` function, the string slice returned will outlive both `Context` and `Parser`, and that the reference `parse_context` returns refers to the string slice, not to `Context` or `Parser`.

---

https://doc.rust-lang.org/book/second-edition/ch19-02-advanced-lifetimes.html
