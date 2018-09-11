# Subtyping and coercion in Rust

Monday, March 10, 2014    
http://featherweightmusings.blogspot.com/2014/03/subtyping-and-coercion-in-rust.html

Subtyping and coercion are two related concepts for enabling polymorphic re-use in programming languages.

Generaly (not just in Rust), subtyping is a relation on types which says that T is a subtype of U if T is in some sense a more specific type than U.

> Subtyping relation means that T is a subtype of U, if T is more specific then U.

More precisely, let `T` and `U` denote sets of values, so `T` set is a subset of `U` set, iff `T` values are more specific then `U` values.

That gets complicated when thinking about __existential types__ if we don't have explicit values for the introduction of such types, and if we do, then things get a little circular and not very helpful for thinking about real programming languages.

__Inclusion polymorphism__ (_Liskov substitution principle_ or _strong behavioral subtyping_): If an expression has type T, it can be used anywhere an expression of type U is expected.

__Coercion__ is an operation on values (expressions) where a value of type T can be changed to a value of type U.

An example is using an integer as a float - this is allowed transparently in many languages, but the compiler must insert code which does the low-level conversion from integer to float. Usually, it's assumed that coercions are cheap.

When actually writing code, subtyping and coercion often look the same. For example, "subtyping" between pointer types in C++ with multiple inheritance is technically a coercion because of the pointer adjustment required by the implementation using multiple vtables.


## Subtyping in Rust
The only subtyping Rust has is due to contravariance with respect to lifetime variables.

Rust has coercions between structs and traits given by `impl`s (well, between references to such things). It should (soon hopefully) have coercions between sub-traits and (maybe, eventually) sub-structs. Plus coercions between numeric types which are less interesting.

---

I would like to further classify subtyping and coercions along some axes.

The key difference is that subtyping does not change the underlying value and coercion does.

Both subtyping and coercion may be explicit or implicit.

I sometimes refer to implicit subtyping as _subsumption_, but I'm not sure if that is common or correct.

In Rust
- subtyping is always implicit
- coercion is sometimes implicit (trait objects) and sometimes explicit; numeric conversions require `as` (until recently, coercion used to be always explicit).
- In Rust, only subtyping is used in the type inference algorithm.

The last axis I have thought of is a bit more hazy and a bit more of an implementation detail than a fundamental. It is that coercion restricts access to the value coerced. If coercion changes a value, it would be unsafe to continue to access the value via the old type. Either coercion must copy a value (i.e. only the new, copied value has the new type), or we must ensure that the old value cannot be accessed whilst the coerced value exists. Rusts linearity rules ensure this.


With DST, coercion becomes much more complicated. In particular we will add _covariant coercions_ which must coerce fields of a struct. That is, they change things deep inside the struct, not just at the surface of a value.

In contrast to subtyping, covariant coercion is always safe because we can no longer access the coerced value via its old type.

Thinking about proving safety gave me the insight that coercion actually _changes the type_ of a value, whereas subtyping _gives multiple types to a single value_, which is not to say that coercion always implies monomorphic types.


We have talked a little about perhaps changing the relationship between subtyping and type inference. Perhaps not all subtyping should be taken into account by the type inference algorithm. And/or perhaps some coercions should be taken account of. I don't really know - I have a hard time visualising how type checking would be affected by these changes.

