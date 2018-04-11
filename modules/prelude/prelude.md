# Prelude


- module: `std::prelude`
- submodule: `std::prelude::v1`
- since: 1.0.0
- docs: [std](https://doc.rust-lang.org/std/prelude/)
- Prelude makes available frequently used things.
- auto imported, as if `extern crate std` was inserted into the crate root of every crate, and `use std::prelude::v1::*` into every module.


## Contents of prelude
- `std::marker::{Copy, Send, Sized, Sync}`
- `std::clone::Clone`
- `std::string::{String, ToString}`
- `std::vec::Vec`
- `std::boxed::Box`
- `std::option::Option::{self, Some, None}`
- `std::result::Result::{self, Ok, Err}`
- `std::cmp::{PartialEq, PartialOrd, Eq, Ord }`
- `std::default::Default`
- `std::ops::{Drop, Fn, FnMut, FnOnce}`
- `std::mem::drop`
- `std::borrow::ToOwned`
- `std::convert::{AsRef, AsMut, Into, From}`
- `std::iter::{Iterator, IntoIterator, DoubleEndedIterator,ExactSizeIterator, Extend}`


## Prelude re-exports

```rust
extern crate std;
// use std::prelude::v1::*;
use std::string::{String, ToString};
use std::vec::Vec;
use std::boxed::Box;
use std::option::Option::{self, Some, None};
use std::result::Result::{self, Ok, Err};
use std::marker::{Copy, Send, Sized, Sync};
use std::default::Default;
use std::ops::{Drop, Fn, FnMut, FnOnce};
use std::mem::drop;
use std::clone::Clone;
use std::borrow::ToOwned;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord};
use std::convert::{AsRef, AsMut, Into, From};
use std::iter::{Iterator, IntoIterator, DoubleEndedIterator,ExactSizeIterator, Extend};
```