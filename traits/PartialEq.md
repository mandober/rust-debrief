# `PartialEq` trait

Trait `std::cmp::PartialEq` 1.0.0

Trait for equality comparisons which are partial equivalence relations.


```rust
#[lang = "eq"]
pub trait PartialEq<Rhs = Self>
  where Rhs: ?Sized, {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool { ... }
}
```

This trait allows for partial equality, for types that do not have a full 
equivalence relation. For example, in floating point numbers NaN != NaN, so 
floating point types implement `PartialEq` but not `Eq`.

Formally, the equality must be (for all a, b and c):
- symmetric: a == b implies b == a
- transitive: a == b and b == c implies a == c.

Note that these requirements mean that the trait itself
must be implemented symmetrically and transitively:
  if `T: PartialEq<U>` and `U: PartialEq<V>`
then `U: PartialEq<T>` and `T: PartialEq<V>`.


## Deriving
This trait can be used with `#[derive]`. When derived on `struct`, two instances 
are equal if all fields are equal, and not equal if any fields are not equal. 
On `enum`, each variant is equal to itself and not equal to the other variants.

## Implementing
`PartialEq` only requires __`eq`__ method to be implemented;
`ne` is defined in terms of it by default.
Any manual implementation of `ne` must respect the rule that
`eq` is a strict inverse of `ne`; that is, !(a == b) if and only if a != b.

Implementations of `PartialEq`, `PartialOrd`, and `Ord` must agree with each 
other. It's easy to accidentally make them disagree by deriving some of the 
traits and manually implementing others.


## Example impl: books are considered the same book if their ISBN matches, 
even if the formats differ:

```rust
enum BookFormat {
    Paperback,
    Hardback,
    Ebook
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.isbn == other.isbn
    }
}

let b1 = Book { isbn: 3, format: BookFormat::Paperback };
let b2 = Book { isbn: 3, format: BookFormat::Ebook };
let b3 = Book { isbn: 10, format: BookFormat::Paperback };

assert!(b1 == b2);
assert!(b1 != b3);
```


## Example
```rust
struct Shoe {
    size: i32,
    style: String,
}

impl PartialEq for Shoe {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("scandal") },
        Shoe { size: 10, style: String::from("dasboot") },
    ];

    let my = Shoe { 
        size: 10, 
        style: String::from("crocks"),
    };
    
    if my == shoes[0] { println!("shoobidoobee") }

    // shoes are still there:
    println!("{:?}, {:?}", my, shoes[0]);
}
```
`eq` takes refs: ref to self and other so comparision works
when refs are given: `&my == &shoes[0]`, but even if not given explicitly:
`my == shoes[0]`. Also coul dbe written as `my.eq(&shoes[0])`

