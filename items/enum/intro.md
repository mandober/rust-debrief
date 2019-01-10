# enum

Maybe enum represents an optional (nullable) value.

Maybe solves the problem of type nullability: a type can be viewed as a
set with finite cardinality whose elements are concrete values.
A `bool` is a set of cardinality 2 whose elements are true and false.
A boolean value is either true or false, but what if the value is absent
(unknown, at the moment or always, or not just applicable).

e.g. a survey asking whether one is content with their current job, provining only a boolean "yes" and "no" as potential answers, but the participant is unemployed.

Rust doesn't have nullable types


An enumeration, `enum`, allows defining a new type by enumerating all its possible values; these values can be of any type, so enums are heterogeneous compound types (type constructors). For example, `Ordering` enum from std (`std::cmp::Ordering`) has 3 possible values, but at any given time, an instance of this enum holds ("collapses" into) only one of these values.

```rust
// Ordering enum from std is defined as:
pub enum Ordering {
  Less,
  Equal,
  Greater,
}

let a = Ordering::Less;
let b = Ordering::Equal;
let c = Ordering::Greater;
```

The "overall" type of a, b, c is `Ordering`, they are all of the same type - this means enums can be used as a workaround in situations when a type accepts only values of homogeneous type, like a vector does. In order to put different types into vector, these types can be grouped under an enum umbrella.

```rust
pub enum Element {
  Letter(char),
  Pair(u8, u8),
  Flag(bool),
}

#[derive(Debug)]
let vec: Vec<Element> = vec!(
  Element::Letter('x'),
  Element::Letter('y'),
  Element::Pair(5, 7),
  Element::Flag(true),
);

println!("vec: {:?}", vec);
```

Every new definition of enum in fact, defines a new type, but only some of the enums (from std) are broadly used (`Option`, `Result`, `Ordering`, etc.), while others have a specialized use case (as a carrier for a by-product or intermediate value). Even though they are all enums, colloquially the former are usually called "types" (the `Option` type), while the latter are *just* enums (granted, this is more the case with structs then enums).

The most famous enums are `Option` and `Result` type. They are both in the prelude along with their parent path, so referring to their variants doesn't have to include the parent path:

```rust
// Option and Result are available everywhere, they are automatically
// imported, not need for an explicit import like:
use std::option::Option::{self, Some, None}
use std::result::Result::{self, Ok, Err}

// each enum's parent path (the `self` part in import) is also imported
// so we can refer to variants directly:
let opt = Some(5);
// intead of:
let opt = Option::Some(5);
// or even:
let opt = ::std::option::Option::Some(5);
```

The convenience to refer directly to variants of a user-defined enum (skipping the name of parent) is not possible unless the parent's path is imported:

```rust
// user-defined enum
enum Element {
  Letter(char),
  Pair(u8, u8),
  Flag(bool),
}

// this is ok:
let l = Element::Letter('x');
// this gives an error:
let l = Letter('x');
// when the `Letter` variant is imported:
use Element::Letter;
// then it's ok to refer to it (without parent's supervision):
let l = Letter('x');
// but not to others:
let p = Pair(5, 7); // error
// so import 'em all:
use Element::*;

// imports can be renamed:
use Element::{Letter as Character, Flag as Boolean, Pair as Couple};
let m = Couple(4, 6);

// and/or aliased:
use Element::{Letter, Flag, Pair};
use Element::{Letter as Character, Flag as Boolean, Pair as Couple};
let n = Pair(4, 6);
let m = Couple(4, 6);
assert!(m == n);
```

## representation

- representation
- null pointer optimization
