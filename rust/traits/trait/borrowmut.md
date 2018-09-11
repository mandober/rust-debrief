# BorrowMut trait
https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html

Trait `std::borrow::BorrowMut` 1.0.0

A trait for mutably borrowing data.

```rust
pub trait BorrowMut<Borrowed>: Borrow<Borrowed> 
where Borrowed: ?Sized, 
{
    fn borrow_mut(&mut self) -> &mut Borrowed;
}
```

Similar to `Borrow`, but for mutable borrows.


## Required Methods

```rust
fn borrow_mut(&mut self) -> &mut Borrowed
```

Mutably borrows from an owned value.

Examples:

```rust
use std::borrow::BorrowMut;

fn check<T: BorrowMut<[i32]>>(mut v: T) {
    assert_eq!(&mut [1, 2, 3], v.borrow_mut());
}

let v = vec![1, 2, 3];

check(v);
```