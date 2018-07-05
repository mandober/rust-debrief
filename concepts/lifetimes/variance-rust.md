# Variance and lifetimes

https://medium.com/@kennytm/variance-in-rust-964134dd5b3e

Subtyping relationship:
- Rust:   `a: b`   (lifetime `'a` outlives lifetime `'b`)
- math: `A <: B` (A is subtype of B)

- Subtyping appears in Rust only related to lifetimes
- `'longer` lifetime is a subtype of a `'shorter` lifetime
- `'longer: 'shorter`
- `'static` is the bottom type of lifetimes

If a function takes a specific lifetime `'a` and we provided it with a `'static` lifetime, it will be ok - hence, `'static` is a subtype of any lifetime since it can always jump in to replace required lifetime.

Intuitively, a type is more general, while a subtype, of that particulat type,  is its specialization.

There is only one `'static` lifetime region, while there are many many `'short` lifetime regions, so `'static` is a subtype.


## Variance
Variance is a property of a generic parameter of a type constructor, e.g. `Vec<T>`, which affects subtype relationship among constructed types.

Rust v.1.24 recognizes 4 kinds of variances:
1. Invariant: eg. `Cell<T>` and `Cell<U>` are completely different types
2. Covariant: eg. `Vec<T>` is a subtype of `Vec<U>` iff `T` is subtype of `U`
3. Contravariant: eg. `fn(T)` is subtype of `fn(U)` iff `T` is supertype of `U`
4. Bivariant: means parameter is unused; thus `X<T>` and `X<U>` are always   
   compatible even if `T` and `U` are completely different.


## Invariant
- e.g.`Cell<T>` and `Cell<U>` are completely different types
- types have no variance relation
- types are _never compatible_
- Invariant = does not change


## Bivariant
type parameter is unused :  `X<T>` and `X<U>`  
are always compatible even if `T` and `U` are completely different.

- type parameter is unused
- `X<T>` and `X<U>`are _always compatible_
- even if T and U are completely different types.


## Covariant
- eg. `Vec<T>` is a subtype of `Vec<U>` iff `T` is subtype of `U`
- Arrows point in the same direction.
- Covariant = changes in the same way

```
Vec<T> <: Vec<U>  (is subtype of)
if  T  <:     U   (is subtype of)

Vec<&'a T>  <:  Vec<&'b U>  (is subtype of)
if  &'a T  <:       &'a U
```

## Contravariant
- eg. `fn(T)` is subtype of `fn(U)` iff `T` is supertype of `U`
- Arrows point in the opposite directions
- Covariant = changes in the same way

```
fn(T)   <:   fn(U)    fn(T) is subtype of fn(U) 
   T     :>     U    iff T is supertype of   U

       fn(T) is subtype   of fn(U)
iff       T  is supertype of    U
i.e. iff  U  is subtype   of    T
```


## Variance Arithmetic

Let the set of variances be:
* `0` : invariant
* `+` : covariant
* `-` : contravariant
* `∞` : bivariant

We define the following operations:

1. `Transform (V × W)`
- Combines variances where the type constructors are composed
- Example: `Vec<fn(T)>` has variance of `Vec<T> × fn(T)`
- Unit element is `+` (covariant)
- Implemented in `rustc::ty::Variance::xform` (rustc source)

```
╔═══╦═══╤═══╤═══╤═══╗
║ × ║ 0 │ + │ − │ ∞ ║
╠═══╬═══╪═══╪═══╪═══╣
║ 0 ║ 0 │ 0 │ 0 │ 0 ║
╟───╫───┼───┼───┼───╢
║ + ║ 0 │ + │ − │ ∞ ║
╟───╫───┼───┼───┼───╢
║ − ║ 0 │ − │ + │ ∞ ║
╟───╫───┼───┼───┼───╢
║ ∞ ║ ∞ │ ∞ │ ∞ │ ∞ ║
╚═══╩═══╧═══╧═══╧═══╝
```

2. `GLB (V ∧ W)`
- short for __greatest-lower-bound__, also known as *meet* or *infimum*
- Combines variances where two types form a tuple.
- Example: (Vec<T>, fn(T)) has variance of Vec<T> ∧ fn(T).
- Unit element is ∞ (bivariant).
- Implemented in rustc_typeck::variance::xform::glb in rustc source.

```
╔═══╦═══╤═══╤═══╤═══╗
║ ∧ ║ 0 │ + │ − │ ∞ ║
╠═══╬═══╪═══╪═══╪═══╣
║ 0 ║ 0 │ 0 │ 0 │ 0 ║
╟───╫───┼───┼───┼───╢
║ + ║ 0 │ + │ 0 │ + ║
╟───╫───┼───┼───┼───╢
║ − ║ 0 │ 0 │ − │ − ║
╟───╫───┼───┼───┼───╢
║ ∞ ║ 0 │ + │ − │ ∞ ║
╚═══╩═══╧═══╧═══╧═══╝
```


## Variance of built-in type constructors

* Primitives, u32, str, bool, extern type, etc
  `∞`
* Generic parameter, `T`
  `+`
* References, `&'a<'b> _`, `&'a<'b> mut _`
  `'b` = `+` × `'a`
* Shared pointer, `&X<T>`, `*const X<T>`
  `T` = `+` × X
* Unique pointer, &mut X<T>, *mut X<T>)
  `T` = `0` (invariant) × X
* Arrays and slices ([X<T>])
  `T` = `+` (covariant) × X
* Aggregates (tuples, struct, enum, union)
  GLB of all fields
* Trait<Input<T>, Output=Output<U>> + 'a<'b>)
  T = 0 (invariant) × Input
  U = + (covariant) × Output
  'b = + (covariant) × 'a
* Function pointers (fn(Input<T>) -> Output<U>)
  T = − (contravariant) × Input
  U = + (covariant) × Output


