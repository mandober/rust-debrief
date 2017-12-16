# Top type

The top type, commonly denoted with the down tack symbol `⊤`, is the universal type, sometimes called the universal supertype as all other types in any given type system are subtypes of top.

In most cases it is the type which contains every possible object in the type system of interest. It is in contrast with the bottom type, or the universal subtype, which every other type is supertype of and in most cases it is the type that contains no members at all.

Rust has parametric polymorphism - the most generic type parameter is  
`<T: ?Sized>`.

Note: `<T>` is not, as it implies the `Sized` trait - every type parameter is implicitly bound to `Sized`. The sytax `?Sized` means that the type may or may not be sized.

BTW, the most generic type class parameter in Haskell is `forall a. a`
