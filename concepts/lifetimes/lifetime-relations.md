[official Rust FAQ](https://www.rust-lang.org/en-US/faq.html)

- Lifetime
- Lifetime variance
- Lifetime subtyping


## Lifetime

Lifetime bound on a type: `T: 'a`

What does it mean to have a type parameter `T` and a lifetime bound like this: `where T: 'a`

The intuitive and high level explanation is:
`T` maybe contains references, and if it does, all of them are valid for at least the scope `'a`.
If `T` doesn't contain references, then we are fine anyway.
Having no references, we satisfy `T: 'a` for all `'a`.

Also note that if we have the type `&'a T`, then we always require `T: 'a`. 

How can read this:
If we have a reference valid for the scope `'a`, then the type `T` and anything it contains, must be valid to use for at least the scope `'a` too.


What is `U: 'static`
This is just a special case of the previous question, but it often shows up.

The intuitive explanation is that `U: 'static` means that `U` does not contain references. Or `U` does not contain references, except `'static` ones.
More formally, we can say that we have an indefinite lease on values of type `U` and there is no point in the program where the value goes invalid.
It doesn't mean values of `U` are in a static variable or must live forever.
It means that we are allowed to keep them around forever.
But we don't have to. There are no strings attached and no best-used-before-date, because `'static` is the longest lifetime there is.


## Lifetime Subtyping

if S is a subtype of T, any term of type S can be safely used in a context where a term of type T is expected.

Subtyping only ever applies to the lifetime part of types.

In Rust subtyping relation is that `&'a X` is a subtype of `&'b X`
if `'a` is longer or equal lifetime as `'b`,
which is usally written as `'a: 'b` and read as "a outlives b".
We can see that clearly, where a `&'b X` is expected,
we can safely use any `&X` with a lifetime that outlives `'b`.


## Lifetime Variance

Variance is about how subtype relations change when type constructors are applied.

If we have a type `Vec<T>` in Rust, we call `Vec` a type constructor because it takes a type and the result is the type `Vec<T>`.

S a subtype of T is also written `S <: T`.

Variance tells us: If I know that `S` is a subtype of `T`, what the subtype relation between `Construct<S>` and `Construct<T>`?

The important variance cases for a type constructor `Vec`, with respect to its type parameter, are: 
- **Invariant**
  - `Construct<S> <: Construct<T>` only if S is equal to T
- **Covariant**
  - `Construct<S> <: Construct<T>` if `S <: T`
  - co for the same; the direction of the relation does not change
- **Contravariant**
  - `Construct<T> <: Construct<S>` if `S <: T`
  - contra for the opposite; the direction of the relation is flipped

Covariant is sometimes called just variant. Note that variance is with-respect-to a particular parameter of the constructor.

