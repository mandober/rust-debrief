# Lifetime

- Lifetime params are part of generics, Generic Lifetime Parameters (GLP).
- Lifetimes are annotated with Generic Lifetime Annotations (GLA)   
  using lowercase, preceded by a single quote, e.g. `'a`
- GLA tell Rust how GLP relate to each other: they are descriptive, not 
  prescriptive; borrowck compares scopes to determine if all exp are valid.
- GLP are mostly tied to references, even through every expression has a 
  lifetime, which is the scope for which it is valid.
- The same ref with different lifetime are different types.
- `'static` is max lifetime, valid throughout the entire program.
- __outlives__: some lifetime lives at least as long as another (or self)
- Lifetime Elision Rules (LER):
  1. Each param of ref type gets its own distinct GLP
  2. If there's 1 input GLP, that GLA is set to all output GLPs
  3. In methods, with 2+ input GLP, GLA of `self` is set to all output GLPs



<!-- TOC -->

- [Lifetimes](#lifetimes)
- [Lifetimes as bounds](#lifetimes-as-bounds)
- [Outlives relation](#outlives-relation)
- [Lifetime Elision Rules (LER)](#lifetime-elision-rules-ler)
- [The static lifetime](#the-static-lifetime)
- [Structs and lifetimes](#structs-and-lifetimes)
- [Type distinction](#type-distinction)
- [Syntax](#syntax)

<!-- /TOC -->


## Lifetimes




The word "lifetime" can be used in two distinct, but similar, ways:
- The lifetime of a **reference**, corresponding to the span of time in which that reference is **used**.
- The lifetime of a **value**, corresponding to the span of time before that value gets **dropped**.

Lifetimes and scopes are linked to each other, but to distinguish the two, the latter is referred to as value's **scope**.


A scope generally corresponds to some block or, more specifically, from the `let` statement to the end of the enclosing block, with one exception: the scope of a temporary value is sometimes the enclosing statement. A lifetime can span only a single expression, but also until the end of the enclosing block.

When a reference uses a temporary, its lifetime spans a single expression:

```rust
fn foo() {
    let mut data = vec!['a', 'b', 'c']; // --+ 'scope
    capitalize(&mut data[..]);          //   |
//  ^~~~~~~~~~~~~~~~~~~~~~~~~ 'lifetime //   |
    data.push('d');                     //   |
} // <---------------------------------------+

fn capitalize(data: &mut [char]) {}
```

When a reference is assigned into a variable, its lifetime gets extended. The compiler requires the lifetime to be the innermost expression (often a block) that encloses both statements:

```rust
fn bar() {
    let mut data = vec!['a', 'b', 'c'];
    {
        let slice = &mut data[..]; // <-+ 'lifetime
        capitalize(slice);         //   |
    } // <------------------------------+
    data.push('d');
}
```

Without the block the push call that mutates the data would error. The block makes the scope of `slice` smaller, thus the resulting lifetime is smaller. Introducing a block like this is kind of artificial, prompting an RFC for Non-Lexical Lifetimes ([NLL](https://github.com/nikomatsakis/nll-rfc/blob/master/0000-nonlexical-lifetimes.md))


The basic idea of the borrow checker is that values may not be mutated or moved while they are borrowed. 
To enforce this, whenever a borrow is created, the compiler assigns the resulting reference a lifetime. 
This lifetime corresponds to the span of the code where the reference may be used. 
The compiler infers this lifetime to be the smallest possible while still encompassing all the uses of the reference.


---

Lifetimes are annotated with Generic Lifetime Annotations (GLA), using lowercase, preceded by a single quote, e.g. `'a`.

Every reference has a lifetime, which is the scope for which that reference is valid. Lifetime annotations are descriptive (not prescriptive) - they only describe how the lifetimes of references relate to each other; borrow checker then compares scope of references to determine that all references are valid, thus avoiding problems like dangling pointer.

```rust
// annotations of the lifetimes of o and i:
{                    //..................
  let o;             //                 ↑
  {                  //........         'outter
    let i = 5;       //       ↑
                     //       'inner
    o = &i;          //       ↓
                     //........
  }
  println!("{}", o); //                 ↓
                     //..................
}
```

Above, the variable `o` is declared in the outer scope. Later, it takes a reference to the `i` variable, that is valid only in the inner scope. When the inner scope ends, `i` is dropped, so `o` cannot be allowed to refer to `i` after that because it would create a dangling pointer.

Since lifetimes are scopes, they can be partially ordered based on the "outlives" relationship: `o` "outlives" `i`, or `i` doesn't live long enough (for reference to it taken by `o` to be valid).

You can never create owning type inside a function and then have that fn return a reference to that type; that owning type wouldn't live long enough as it would be dropped at the end of fn. When a ref is returned from a fn its referent must be outside that fn, and ref's lifetime parameter must match the lifetime of one of the input parameters.


## Lifetimes as bounds

Just like generic types can be bounded so can lifetimes.  
The bound symbol, `:`, has a slightly different meaning. 

This reads as:
- `T: 'a` all references in `T` must outlive lifetime `'a`.
- `T: Trait + 'a` type `T` must implement trait `Trait` and all references in `T` must outlive lifetime `'a`.



```rust
struct Items<'a, T:'a> {
  v: &'a Collection<T>
}
```

Here, the constraint T:'a indicates that the data being iterated over must live at least as long as the collection (logically enough).


## Outlives relation

The "outlives" relation between two lifetimes, `'a` and `'b`, is written as
`'a, 'b: 'a`. Here, lifetime `'b` outlives the lifetime `'a`.

In fact, __"outlives"__ means that some lifetime lives __at least as long__ as some other lifetime (but possibly longer).

> Outlives is "longer then or equal" relation: `'a: 'a`.
> It is similar to "greater then or equal" relation: `a >= a`.

Because of the "or equal" part in this "greater then or equal" relation, it can be said that every lifetime outlives itself: for any lifetime `'a`, `'a` outlives `'a` i.e. any lifetime "outlives" itself. 
This is also similar to how it is said that every type is a subtype of itself.


## Lifetime Elision Rules (LER)

Every reference has some lifetime associated with it, but the compiler lets you elide them in common cases:

1. Each elided lifetime in input becomes a distinct lifetime param.
2. If there is 1 input lifetime position (elided or not), that lifetime is 
  assigned to all elided output lifetimes.
3. If there are 2+ input lifetimes, but one of them is self, the lifetime of 
  self is assigned to all elided output lifetimes. Other input lifetime positions get distinct lifetimes according to first rule.

Otherwise, it is an error to elide an output lifetime. It can also happened that these assumptions are not what's intended, in which case lifetimes must be annotated manually.


```rust
// LER 1
fn print(s: &str);        // elided
fn print<'a>(s: &'a str); // expanded

fn print(x: &i32, y: &i32);               // elided
fn print<'a, 'b>(x: &'a i32, y: &'b i32); // expanded

fn debug(lvl: usize, s: &str);        // elided
fn debug<'a>(lvl: usize, s: &'a str); // expanded

fn new(buf: &mut [u8]) -> BufWriter;            // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded


// LER 2
fn substr(s: &str, until: usize) -> &str;           // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str; // expanded


// LER 3
fn get_mut(&mut self) -> &mut T;           // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

fn cut(&self, x: &i32) -> &Self;                  // elided
fn cut<'a, 'b>(&'a self, x: &'b i32) -> &'a Self; // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided, expand:
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;

fn foo(&self, x: &i32, y: &i32) -> &Self; // elided
fn foo<'a, 'b, 'c>(&'a self, x: &'b i32, y: &'c i32) -> &'a Self; // expanded


// Error, requires lifetime annotations:
fn frob(s: &str, t: &str) -> &str;
// otherwise when expanded output lifetime is ambiguous:
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str;


// Illegal, no inputs
fn get() -> &str;
```


## The static lifetime

The lifetime named `static` is special; a ref with static lifetime is valid throughout the entire program.

```rust
// static lifetime
let a: &'static str = "always";
```

String literals have the type `&'static str` because they are stored in the data segment of the final binary. Since the binary is always available, the lifetime of all string literals is `'static`.

Globals also have static lifetime:

```rust
// This adds an `u8` to the data segment of the binary, `x` is a ref to it.
static FOO: u8 = 5;
let x: &'static u8 = &FOO;
```


## Structs and lifetimes

Besides owned values, structs can hold references in their fields, but similar to other generics, GLP must be declared first (in the angle brackets) after the name of the struct, so that we can use the GLP in the body of the struct definition.

```rust
// struct that holds a ref
struct Foo<'a, T> { inner: Option<&'a T> }
//          ^ declaretion           ^ usage
```


## Type distinction

Just like `T` and `&T` are distinct types, so is the reference with a lifetime distinct from the reference without it, even though the referent type is the same. These are all distinct types:  
`T`, `&T`, `&'a T`, `&'static T`, `&mut T`, `&'a mut T`, `&'static mut T`


```rust
// this type:
struct Foo { inner: Option<u8> }
// is distinct from this one:
struct Foo2<'a> { inner: Option<&'a u8> }
// impl block for the first is:
impl Foo {}
// while for the second it is:
impl<'a> Foo2<'a> {}
```


## Syntax

Lifetimes appear in places similar to other generics:

```rust
&&str;    // ref to string slice with inferred lifetime
&&'a str; // ref to string slice with explicit lifetime annotation

// calling a method, turbofish syntax
let x: &'c i32 = Default::default::<'c>(&'c num);
let z: &'d i32 = as_str::<'d>(&'d val);
Foo::mut_share::<'c>(&'c mut foo);
Foo::share::<'d>(&'d foo);

// lifetimes must be first declared, then used
struct Iter<'a, T: 'a> { inner: Option<&'a T> }

// lifetimes can act as bounds
impl<'a, T: 'a> Iterator for Iter<'a, T> {}
```


Generics: type parameters, trait bounds and lifetimes

```rust
use std::fmt::{Display, Debug};

fn long<'a, T>(x: &'a str, y: &'a str, msg: T) -> &'a str
  where T: Display + Debug {
    println!("display: {}", msg);
    println!("debug: {:?}", msg);
    if x.len() > y.len() { x } else { y }
}
```
