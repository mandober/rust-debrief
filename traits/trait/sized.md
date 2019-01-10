# `Sized` trait
https://doc.rust-lang.org/std/marker/trait.Sized.html

Trait `std::marker::Sized` 1.0.0

Types with a *constant size known at compile time*.

```rust
#[lang = "sized"]
pub trait Sized { }
```

Type parameters have an implicit bound of `Sized`.
Special syntax `?Sized` can be used to
remove this bound if it's not appropriate.


```rust
// type params are implicitly Sized-bounded
struct Foo<T>(T); // so this is the same as:
struct Foo<T>(T: Sized);
// But, this gives error because
struct FooUse(Foo<[i32]>);
//error: Sized is not implemented for [i32]

// to remove the Sized trait bound use ?Sized syntax
struct Foo<T: ?Sized>(T);
struct FooUse(Foo<[i32]>);
```

The one exception is the implicit `Self` type of a trait, which does not get an 
implicit `Sized` bound. This is because a `Sized` bound prevents the trait from 
being used to form a trait object:

```rust
struct Impl;

trait Foo { }
impl Foo for Impl { }
let x: &Foo = &Impl; // OK

trait Foo: Sized { }
impl Foo for Impl { }
let y: &Foo = &Impl; // error: the trait `Foo` cannot be made into an object
```

