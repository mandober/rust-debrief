# PartialEq

- `std::cmp::PartialEq` since 1.0.0. It is a lang item.
- equality comparisons which are partial ≡ (equivalence) ⥽ (relation)
- allows for partial equality, for types without a full ≡ ⥽
- Equality ∀a,b,c:
  - symmetric (a == b ⟹ b == a)
  - transitive (a == b ∧ b == c ⟹ a == c)
  - must be implemented symmetrically and transitively: if   
    `T: PartialEq<U>` and `U: PartialEq<V>` then
    `U: PartialEq<T>` and `T: PartialEq<V>`
- Implementing (methods: `eq`, `ne`)
  - requires impl just `eq` method, `ne` follows from it
  - Derivable trait. When derived on
    - `struct`: two instances are eq if all fields are eq.
    - `enum`: a variant is eq to itself and `ne` to others.



```rust
#[lang = "eq"]
pub trait PartialEq<Rhs = Self> where Rhs: ?Sized {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}
```

This trait allows for partial equality, for types that do not have a full equivalence relation. For example, in floats `NaN != NaN`, so floats implement `PartialEq` but not `Eq` (Eq is a marker trait that marks reflexivity).

Formally, the equality must be (for all a, b and c):
- symmetric: a == b implies b == a
- transitive: a == b and b == c implies a == c.

Note that these requirements mean that the trait itself
must be implemented symmetrically and transitively:
  if `T: PartialEq<U>` and `U: PartialEq<V>`
then `U: PartialEq<T>` and `T: PartialEq<V>`.


## Deriving
This trait can be used with `#[derive]`. When derived on `struct`, two instances are equal if all fields are equal, and not equal if any fields are not equal. On `enum`, each variant is equal to itself and not equal to the other variants.


## Implementing
`PartialEq` only requires `eq` method to be implemented; `ne` is defined in terms of it by default.

Any manual implementation of `ne` must respect the rule that `eq` is a strict inverse of `ne`; that is, `!(a == b)` iff `a != b`.

Implementations of `PartialEq`, `PartialOrd`, and `Ord` must agree with each 
other. It's easy to accidentally make them disagree by deriving some of the 
traits and manually implementing others.


## Example
Books base their eq on ISBN (even if formats differ):

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

`eq` takes self and other by ref so comparision works with refs, but also with owned types.

```rust
// takes self and other by ref
&my == &shoes[0]

// or if not given explicitly:
my == shoes[0]
// can also be written as:
my.eq(&shoes[0])
```