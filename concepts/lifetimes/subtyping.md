# Subtyping and variance

- Subtyping on lifetimes is in terms of "outlives" relationship
- The bigger scope is a subtype of the smaller scope:   
  you can provide `'static` when `'a` is expected.
- Higher-Ranked Lifetime (HRL) is subtype of concrete lifetime; taking an
  arbitrary lifetime is strictly more general than taking a specific one.
- Variance is a property that type constructors have with respect to their args
- Type constructor in Rust is a generic type with unbound args.   
  a `Vec` is a type constructor that takes `T` and returns `Vec<T>`.   
  also `&` and `&mut`, they take 2 inputs, a lifetime and a type to point to.
- Type constructor's variance is how the subtyping of its inputs affects the
  subtyping of its outputs. 
- There are 2 kinds of variance in Rust:
  - `F` is variant over `T`, if `T`, being a subtype of `U`, implies   
    `F<T>` is a subtype of `F<U>` (subtyping "passes through")
  - `F` is invariant over `T` otherwise (no subtyping relation can be derived)




## Subtyping

Subtyping derives entirely from lifetimes. Since lifetimes are scopes, we can partially order them based on the "contains" ("outlives") relationship. We can even express this as a generic bound.

Subtyping on lifetimes is in terms of that relationship: 
if `'a: 'b` i.e. if "a contains b" (or "a outlives b"), 
then `'a` is a subtype of `'b`.

The bigger scope is a subtype of the smaller scope, 
because you can provide `'static` when at least `'a` is expected.

Higher-ranked lifetimes (HRL) are also subtypes of every concrete lifetime. This is because taking an arbitrary lifetime is strictly more general than taking a specific one.


## Variance

Variance is a property that type constructors have with respect to their args.
A type constructor in Rust is a generic type with unbound args.

For instance `Vec` is a type constructor that takes `T` and returns `Vec<T>`.
`&` and `&mut` are type constructors that take 2 inputs: a lifetime, and a type to point to.

A type constructor's variance is how the subtyping of its inputs affects the subtyping of its outputs. 

There are 2 kinds of variance in Rust:
- `F` is variant over `T` 
  if `T`, being a subtype of `U`, implies
  `F<T>` is a subtype of `F<U>` 
  (subtyping "passes through")
- `F` is invariant over `T` otherwise 
  (no subtyping relation can be derived)

In other languages, what is refered here as "just" variance is in fact _covariance_. Rust has _contravariance_ for functions: `fn(T)` is contravariant in `T`, which is used in matching methods in trait impl to the trait definition. Traits don't have inferred variance, so `Fn(T)` is invariant in `T`.

Some important variances:
1. `&'a T` is variant over `'a` and `T` (as is `*const T` by metaphor)
2. `&'a mut T` is variant over `'a` but invariant over `T`
3. `Box<T>`, `Vec<T>`, etc. are variant over `T`
4. `UnsafeCell<T>`, `Cell<T>`, `RefCell<T>`, `Mutex<T>` and all other interior
   mutability types are invariant over `T` (as is `*mut T` by metaphor)
5. `Fn(T) -> U` is invariant over `T`, but variant over `U`


1. `&'a T`: variant over `'a` and `T`

`&'a T` is variant over `'a`: we can pass longer where shorter lifetime is expected, e.g. `'static` when at least `'a` is expected.

`&'a T`: variant over `T`: we can pass `&&'static str` where `&&'a str` is expected. The additional level of indirection should not prevent us from being able to pass longer where shorter lifetime is expected.


2. `&'a mut T`: variant over `'a`, invariant over `T`

To see why` &mut` should be invariant over `T`, consider the following code:

```rust
fn overwrite<T: Copy>(input: &mut T, new: &mut T) {
    *input = *new;
}

fn main() {
    let mut forever: &'static str = "hello";
    {
      let stringy = String::from("world");
      overwrite(&mut forever, &mut &*stringy);
    }
    // Oops, printing free'd memory
    println!("{}", forever);
}
```

The signature of `overwrite` is clearly valid: it takes mutable references to two values of the same type, and overwrites one with the other.

If `&mut T` was variant over `T`, then `&mut &'static str` would be a subtype of `&mut &'a str`, since `&'static str` is a subtype of `&'a str`.

Therefore the lifetime of `forever` would successfully be "shrunk" down to the shorter lifetime of string, and overwrite would be called successfully. 

`stringy` would subsequently be dropped, and `forever` would point to freed memory when we print it!

Therefore `&mut` should be invariant.

This is the general theme of variance vs invariance: if variance would allow you to store a short-lived value into a longer-lived slot, then you must be invariant.

However it is sound for `&'a mut T` to be variant over `'a.` 

The key difference between `'a` and `T` is that `'a` is a property of the reference itself, while `T` is something the reference is borrowing.

If you change `T`'s type, then the source still remembers the original type. However if you change the lifetime's type, no one but the reference knows this information, so it's fine.

Put another way: `&'a mut T` owns `'a`, but only borrows `T`.
