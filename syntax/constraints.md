# Constraints

- `T: U`
  generic parameter T constrained to types that implement `U`.
- `T: 'a`
  `T` must outlive `'a`, it cannot transitively contain any references with lifetimes shorter than `'a`.
- `T: 'static`
  `T` contains no borrowed references other than `'static`
- `'b: 'a`
  generic lifetime `'b` must outlive lifetime `'a`.
- `T: ?Sized`
  allow generic type parameter to be a dynamically-sized type.
- `'a + trait, trait + trait`
  compound type constraint.


