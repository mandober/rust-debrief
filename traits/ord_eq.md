# Binary rel

- https://doc.rust-lang.org/std/cmp/trait.Eq.html
- https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
- https://doc.rust-lang.org/std/cmp/trait.Ord.html
- https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html


```
PartialEq<Rhs: ?Sized=Self>: transitive,     symmetric              eq, (ne)
Eq: PartialEq         : transitive,     symmetric, reflexive   -
PartialOrd: PartialEq : transitive, antisymmetric              partial_cmp, (lt,le,gt,ge)
Ord: Eq + PartialOrd  : transitive, antisymmetric, total       cmp, (max, min)
```


- Eq has no extra methods, it is only informing the compiler that this is an 
  equivalence relation rather than a partial equivalence relation.
- Deriving Eq requires all fields are Eq.
- if   `A: PartialEq<B>` and `B: PartialEq<C>`   
  then `B: PartialEq<A>` and `A: PartialEq<C>`.


binary rel | transitive | symmetric | reflexive
-----------|------------|-----------|--------
Eq         | ✔         | ✔         | ✔
PartialEq  | ✔         | ✔


binary rel | transitive | antisymmetric | total
-----------|------------|---------------|------
Ord        | ✔         | ✔             | ✔
PartialOrd | ✔         | ✔             |



## Transitive relations
- All 4 relations are transitive

A binary relation `R` over a set `X` is transitive if whenever an element `a` is related to an element `b` and `b` is related to an element `c` then `a` is also related to `c`.

Transitivity (or transitiveness) is a key property of both _partial order_ relations and _equivalence_ relations.

Formal definition: `∀a,b,c ∈ X: (aRb ∧ bRc) ⇒ aRc`

PartialOrd: transitivity: 
       Ord: transitivity: 
 PartialEq: transitivity:
        Eq: transitivity: `a <= b` ∧ `b <= c` ⟹ `a <= c`




A binary relation `R` (`⥽`) over a set `X` is transitive
IF (`a` is related to `b`) AND (`b` is related to `c`)
THEN `a` is also related to `c`.

> Transitivity: `∀ (a,b,c) ∈ X: (a⥽b ∧ b⥽c) ⇒ (a⥽c)`

Transitivity (or transitiveness) is a key property of both partial order relations and equivalence relations.



## Eq
- equivalence relations
- `pub trait Eq: PartialEq<Self> { }`
- derivable
- `Eq: PartialEq`
- Trait for equality comparisons which are equivalence relations.
- This property cannot be checked by the compiler, and therefore `Eq` implies `PartialEq`, and has no extra methods.
* In addition to `a == b` and `a != b` being strict inverses,  
  `∀ a, b, c`:
  - **reflexive**: `a == a`
  - **symmetric**: `a == b` ⟹ `b == a`
  - **transitive**: `a == b` ∧ `b == c` ⟹ `a == c`



## PartialEq

```rust
pub trait PartialEq<Rhs = Self> where Rhs: ?Sized {
  fn eq(&self, other: &Rhs) -> bool;
  fn ne(&self, other: &Rhs) -> bool { /* ... */ };
}
```

- derivable
- the equality must be `∀ a, b, c`:
  - **symmetric**: `a == b` ⟹ `b == a`
  - **transitive**: `a == b` ∧ `b == c` ⟹ `a == c`.


## Ord
- `pub trait Ord: Eq + PartialOrd<Self> { ... }`
- Ord: PartialOrd + Eq 
  (Eq: PartialEq)


- `Ord` requires that the type also be `PartialOrd` and `Eq`   
  (and `Eq` requires `PartialEq`).
- Trait for types that form a [total order](www.wikipedia.com/en/Total_order)
- This trait can be used with `#[derive]`. When derived on structs, it will 
  produce a lexicographic ordering based on the top-to-bottom declaration order 
  of the struct's members. When derived on enums, variants are ordered by their 
  top-to-bottom declaration order.
- Total order: total and antisymmetric and transitive


* An order is a total order if `∀ a, b, c`:
  - __total__:   
  - __antisymmetric__:   
    *exactly one* of `a < b`, `a == b` or `a > b` is true
  - __transitive__: 
    `a < b` ∧ `b < c` ⟹ `a < c`.  
    `a > b` ∧ `b > c` ⟹ `a > c`.  
    `a == b` ∧ `b == c` ⟹ `a == c`.  


## PartialOrd
- `pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>`
- Trait for values that can be compared for a sort-order.
- The comparison must satisfy antisymmetry, transitivity
- Note that these requirements mean that the trait itself must be implemented 
  symmetrically and transitively: if `T: PartialOrd<U>` and `U: PartialOrd<V>` 
  then `U: PartialOrd<T>` and `T: PartialOrd<V>`.


* The comparison must satisfy `∀ a, b, c`:
  - __antisymmetry__:
    `a < b` => `!(a > b)`
    `a > b` => `!(a < b)`
  - __transitivity__: 
    `a < b` ∧ `b < c` ⟹ `a < c`   
    `a > b` ∧ `b > c` ⟹ `a > c`   
    `a == b` ∧ `b == c` ⟹ `a == c`   


- `Ord`: total and antisymmetric, transitive
- `PartialOrd`: antisymmetric, transitive



