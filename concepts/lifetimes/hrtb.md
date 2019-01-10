# Higher-Ranked Trait Bounds (HRTB)


## nomicon

Rust's Fn traits are a little bit magic. For instance, we can write the following code:

```rust
struct Closure<F> { data: (u8, u16), func: F }

impl<F> Closure<F> where F: Fn(&(u8, u16)) -> &u8 {
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

fn main() {
    let clo = Closure { data: (0, 1), func: do_it };
    println!("{}", clo.call());
}
```

If we try to desugar this we'll run into trouble:

```rust
impl<F> Closure<F>
    // where F: Fn(&'??? (u8, u16)) -> &'??? u8 
{   fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}
```

How on Earth are we supposed to express the lifetimes on `F`'s trait bound? 
We need to provide some lifetime there, but the lifetime we care about can't be named until we enter the body of `call`! Also, that isn't some fixed lifetime; `call` works with any lifetime` &self` happens to have at that point.

This requires Higher-Rank Trait Bounds (HRTBs) to desugar:

```rust
impl<F> Closure<F> where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8 {
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}
```

Where `Fn(a, b, c) -> d` is itself just sugar for the unstable real `Fn` trait

`for<'a>` can be read as "for all choices of `'a`", and basically produces an infinite list of trait bounds that `F` must satisfy. 

There aren't many places outside of the `Fn` traits where we encounter HRTBs, and even for those we have a nice magic sugar for the common cases.



## reddit
Given a trait `Traitor<'a>` you can make a new trait `for<'a> Traitor<'a>`, which things only implement if they implement`Traitor<'a>` for all `'a`. You can then use this trait as a trait object, or in a where clause.

The origin of `for<'a>` has to do with closures that were brought into Rust using its trait system instead of them having their own specific types.

In functions, we can connect the borrow of input with the output:   
`fn foo<'a>(&'a Input) -> &'a Output`.

The equivalent with closure, using the `Fn` trait in this example,    
needed the `for` syntax to express `Fn` trait as a trait bound:   
`for<'a> Fn(&'a Input) -> &'a Output`

Although the `Fn(&Input) -> &Output` shorthand is used prevalently, the language needed the `for<'a>` to became a feature.

Short version: `for<'a> T: Trait<'a>` means that `T` implements `Trait<'a>` for all `'a` rather than a single, specific `'a`.

- [nomicon][1]
- [rfcs][2]
- [stackoverflow][3]
- [reddit][4]


[1]: https://doc.rust-lang.org/nomicon/hrtb.html
[2]: https://github.com/nox/rust-rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md
[3]: https://stackoverflow.com/questions/35592750/how-does-for-syntax-differ-from-a-regular-lifetime-bound/35595491#35595491
[4]: https://www.reddit.com/r/rust/comments/6uobit/fora_lifetime_syntax/


---


## Stack Overflow
From [SO][3]: Higher-Ranked Trait Bounds: `for<'_>` syntax

```rust
trait Trait<T> {}

fn foo<'a>(b: Box<Trait<&'a usize>>);
fn bar    (b: Box<for<'a> Trait<&'a usize>>);
```

In short, the difference between `foo` and `bar` is that 
- in `foo()` the lifetime for the internal `usize` ref is provided by the caller of the function, while 
- in `bar()` the same lifetime is provided by the function itself.
This distinction is very important for the implementation of `foo` or `bar`.

However, in the example above, when `Trait` has no methods which use the type parameter, this distinction is pointless, so let's imagine that `Trait` looks like this:

```rust
trait Trait<T> {
  fn do_something(&self, value: T);
}

fn foo<'a>(b: Box<Trait<&'a usize>>);
fn bar    (b: Box<for<'a> Trait<&'a usize>>);
```

Remember, lifetime parameters are very similar to generic type parameters.
When you use a generic function, you always specify all of its type parameters, providing concrete types, and the compiler monomorphizes the function. 

Same thing goes with lifetime parameters: when you call a function which have a lifetime parameter, you specify the lifetime, albeit implicitly:

```rust
// imaginary explicit syntax
// also assume there's TraitImpl::new::<T>() -> TraitImpl<T>,
// and TraitImpl<T>: Trait<T>
'a: {
    foo::<'a>(Box::new(TraitImpl::new::<&'a usize>()));
}
```

What `foo()` can do with this value is now restricted i.e. there's a restriction with which arguments it may call `do_something()`.
For example, this won't compile:

```rust
fn foo<'a>(b: Box<Trait<&'a usize>>) {
    let x: usize = 10;
    b.do_something(&x);
}
```

This won't compile because local variables have lifetimes which are strictly smaller than lifetimes specified by the lifetime parameters.

Therefore you can't call `b.do_something(&x)` because it requires its argument to have lifetime `'a`, which is strictly greater than that of `x`.

However, you can do this with `bar`:

```rust
fn bar(b: Box<for<'a> Trait<&'a usize>>) {
    let x: usize = 10;
    b.do_something(&x);
}
```

Works because bar can select the needed lifetime instead of the caller of bar.

This matters when you use closures which accept references. 
For example, suppose you want to write a `filter()` method on `Option<T>`:

```rust
impl<T> Option<T> {
    fn filter<F>(self, fx: F) -> Option<T> where F: FnOnce(&T) -> bool {
        match self {
            Some(val) => if fx(&val) { Some(val) } else { None }
            None => None
        }
    }
}
```

The closure here must accept a reference to `T` because otherwise it would be impossible to return the value contained in the option (this is the same reasoning as with `filter()` on iterators).

But what lifetime should `&T` in `FnOnce(&T) -> bool` have?

Remember, we don't specify lifetimes in function signatures only because there is lifetime elision in place; actually the compiler inserts a lifetime parameter for each reference inside a function signature. There should be some lifetime associated with `&T` in `FnOnce(&T) -> bool`. 

So, the most "obvious" way to expand the signature above would be:

```rust
fn filter<'a, F>(self, fx: F) -> Option<T> where F: FnOnce(&'a T) -> bool
```

However, this won't compile. As in the example with Trait above, lifetime `'a` is strictly longer than the lifetime of any local variable in this function, including value inside the match statement. Therefore, it is not possible to apply `fx` to `&val` because of lifetime mismatch.

On the other hand, if we expand the signature of `filter()` like this (and this is actually how lifetime elision for closures works in Rust now):

```rust
fn filter<F>(self, fx: F) -> Option<T> where F: for<'a> FnOnce(&'a T) -> bool
```

then calling `fx` with `&val` as an argument is perfectly valid: we can choose the lifetime now, so using the lifetime of a local variable is absolutely fine. And that's why HRTBs are important: you won't be able to express a lot of useful patterns without them.
