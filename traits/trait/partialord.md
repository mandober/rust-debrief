# `PartialOrd` trait
https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

Trait `std::cmp::PartialOrd` 1.0.0

Trait for values that can be compared for a sort-order.

```rust
#[lang = "ord"]
pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool { ... }
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}
```

The comparison must satisfy, for all a, b and c:
- antisymmetry: if a < b then !(a > b), as well as a > b implying !(a < b); and
- transitivity: a < b and b < c implies a < c. The same must hold for both == and >.

Note that these requirements mean that the trait itself 
must be implemented symmetrically and transitively: 
if   `T: PartialOrd<U>` and `U: PartialOrd<V>` 
then `U: PartialOrd<T>` and `T: PartialOrd<V>`.


## Derivable

This trait can be used with `#[derive]`.

When derived on `struct`, it will produce a lexicographic ordering based on the 
top-to-bottom declaration order of the struct's members. 

When derived on `enum`, variants are ordered by their top-to-bottom declaration order.

## How can I implement PartialOrd?

PartialOrd only requires implementation of the partial_cmp method, with the others generated from default implementations.

However it remains possible to implement the others separately for types which do not have a total order. For example, for floating point numbers, NaN < 0 == false and NaN >= 0 == false (cf. IEEE 754-2008 section 5.11).

PartialOrd requires your type to be PartialEq.

Implementations of PartialEq, PartialOrd, and Ord must agree with each other. It's easy to accidentally make them disagree by deriving some of the traits and manually implementing others.

If your type is Ord, you can implement partial_cmp() by using cmp():

use std::cmp::Ordering;

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.height == other.height
    }
}Run
You may also find it useful to use partial_cmp() on your type's fields. Here is an example of Person types who have a floating-point height field that is the only field to be used for sorting:

use std::cmp::Ordering;

struct Person {
    id: u32,
    name: String,
    height: f64,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.height == other.height
    }
}Run
Examples

let x : u32 = 0;
let y : u32 = 1;

assert_eq!(x < y, true);
assert_eq!(x.lt(&y), true);Run
Required Methods

fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>[−]

This method returns an ordering between self and other values if one exists.

Examples

use std::cmp::Ordering;

let result = 1.0.partial_cmp(&2.0);
assert_eq!(result, Some(Ordering::Less));

let result = 1.0.partial_cmp(&1.0);
assert_eq!(result, Some(Ordering::Equal));

let result = 2.0.partial_cmp(&1.0);
assert_eq!(result, Some(Ordering::Greater));Run
When comparison is impossible:

let result = std::f64::NAN.partial_cmp(&1.0);
assert_eq!(result, None);Run
Provided Methods





## Why the split between Eq/PartialEq and Ord/PartialOrd?

There are some types in Rust whose values are only partially ordered, or have only partial equality. Partial ordering means that there may be values of the given type that are neither less than nor greater than each other. Partial equality means that there may be values of the given type that are not equal to themselves.

Floating point types (f32 and f64) are good examples of each. Any floating point type may have the value NaN (meaning “not a number”). NaN is not equal to itself (NaN == NaN is false), and not less than or greater than any other floating point value. As such, both f32 and f64 implement PartialOrd and PartialEq but not Ord and not Eq.

As explained in the earlier question on floats, these distinctions are important because some collections rely on total orderings/equality in order to give correct results.