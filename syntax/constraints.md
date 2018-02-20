# Constraints

- `T: Debug` generic type T constrained to types that implement `Debug` trait.
- `T: 'a` type T must outlive `'a`, it cannot transitively contain any refs with lifetimes shorter than `'a`.
- `T: 'static` T contains no borrowed references other than `'static`
- `'b: 'a` generic lifetime `'b` must outlive lifetime `'a`.
- `T: ?Sized` allow generic type parameter to be a dynamically-sized type.
- `'a + trait, trait + trait` compound type constraint.


`'a, T: 'a`
Commonly found in struct used for return type when impl Iterator trait. 
