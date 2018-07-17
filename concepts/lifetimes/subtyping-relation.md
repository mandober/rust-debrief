# Substitutability

https://github.com/rust-lang/rfcs/issues/391

outlives:
- has the same or longer lifetime (eq or longer)
- lives at least as long (eq or longer relation)
- `'a : 'b` means `'a` outlives `'b` (a >= b with regard to their lifetimes)


To determine subtyping relation between `&'a T` and `&'b T` 
  a ref to `T` is only soundly substitutable for another ref to `T` 
  if the data of the first reference outlives the second


i.e. the relation `&'x T  <:  &'y T` 
should only hold if the lifetime`'x` lives at least as long as `'y`

in other words, the lifetime `'y` is entirely contained within the lifetime `'x`. Note that the latter is denoted by `'y <:_1 'x`


This means that if you use this first definition for a subtyping relation on lifetimes, then `&'a T` is indeed contravariant with respect
to `'a: &'a T <: &'b T` iff `'b <:_1 'a`
That's how we got where we are today, where we keep talking about lifetimes being contravariant.

