# Top type

The top type, commonly denoted with the down tack symbol `⊤`, is the _universal type_, sometimes called the _universal supertype_ as all other types in any given type system are its subtypes.

In most cases it is the type which contains every possible object in the type system of interest. It is in contrast with the _bottom_ type, or the _universal subtype_, which every other type is supertype of, and in most cases it is the type that contains no members at all.

With regards to Rust and its parametric polymorphism, the most generic type parameter is `<T: ?Sized>`. Although Rust doesn't subtypes per se, subtypes are nevertheless somewhat present in relation to lifetimes. The most generic lifetime, `'static` is a subtype of any other lifetime type parameter: anytime some concrete lifetime of `'a` is required, `'static` can step in as replacement. In this regard,`'static` _is_ a _subtype_ of any concrete lifetime, not a supertype.

Generic type parameter,`<T>`, is not the most generic because it implies the `Sized` trait; namely, every type parameter is implicitly bound to `Sized` trait as if it were written: `<T: Sized>`. To relax this requirement, the syntax `<T: ?Sized>` means that the type _may_ or _may not_ be sized.
