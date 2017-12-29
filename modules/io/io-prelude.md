# The IO Prelude

Module `std::io::prelude` 1.0.0

The purpose of this module is to alleviate imports of many common 
IO traits by adding a glob import to the top of IO heavy modules.

```rust
use std::io::prelude::*;

// * Reexports:
pub use super::Read;
pub use super::Write;
pub use super::BufRead;
pub use super::Seek;
```
