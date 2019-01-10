# Trait Any

- Trait `std::any::Any`, 1.0.0
- A type to emulate dynamic typing.
- Most types implement Any. Types that contains a non-`'static` ref do not.

```rust
pub trait Any: 'static {
  // Required method
  fn get_type_id(&self) -> TypeId;
}
```

`get_type_id` is a nightly-only experimental API. (get_type_id [#27745](https://github.com/rust-lang/rust/issues/27745))



## Examples

```rust
#![feature(get_type_id)]

use std::any::{Any, TypeId};

fn is_string(s: &Any) -> bool {
    TypeId::of::<String>() == s.get_type_id()
}

fn main() {
    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
}
```