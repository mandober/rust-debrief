# Associated constants

- Associated constants since v.1.20 on traits, structs, enums




Since v.1.20 Rust allows defining constants associated to:
- traits
- structs
- enums

For example, the following defines an `ID` constant that is not associated to any particular instance and can be used as `Struct::ID` (static property).

```rust
struct Struct;

impl Struct {
  const ID: u32 = 0;
}
```

When defining an associated constant for a trait, it is not required to define a value for it, letting the concrete trait implementation do it.

```rust
trait Trait {
  const ID: u32;
}

struct Struct;

impl Trait for Struct {
  const ID: u32 = 5;
}
```
