# Variance

https://www.wikiwand.com/en/Covariance_and_contravariance_(computer_science)
https://www.wikiwand.com/en/Subtyping

Variance refers to how subtyping between more complex types relates to subtyping between their components.

Depending on the variance of the type constructor, the subtyping relation of the simple types may be either preserved (covariant), reversed (contravariant), or ignored (invariant) for the respective complex types.

In the OCaml programming language, for example, "list of Cat" is a subtype of "list of Animal" because the list constructor is *covariant*. Meaning, the subtyping relation of the simple types are preserved for the complex types.

While "function from Animal to String" is a subtype of "function from Cat to String" because the function type constructor is *contravariant* in the argument type. Here the subtyping relation of the simple types is reversed for the complex types.

By making type constructors covariant or contravariant instead of invariant, more programs will be accepted as well-typed. On the other hand, programmers often find contravariance unintuitive, and accurately tracking variance to avoid runtime type errors can lead to complex typing rules.In order to keep the type system simple and allow useful programs, a language may treat a type constructor as invariant even if it would be safe to consider it variant, or treat it as covariant even though that could violate type safety.


## Formal definition
Within the type system of a programming language, a typing rule or a type constructor is:
- __covariant__ if it _preserves_ the ordering of types (<=), which orders types from more specific to more generic
- __contravariant__ if it _reverses_ this ordering
- __bivariant__ if _both apply_, `I<A> <= I<B>` and `I<B> <= I<A>` at the same time
- __invariant__ (nonvariant) if _neither applies_.

