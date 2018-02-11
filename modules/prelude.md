# Prelude

- module: `std::prelude`
- submodule: `std::prelude::v1`
- since: 1.0.0
- docs: [std](https://doc.rust-lang.org/std/prelude/)

Prelude makes available frequently used things. It is auto imported, as if `extern crate std` was inserted into the crate root of every crate, and 
`use std::prelude::v1::*` into every module.

- `std::string::{String, ToString}`
- `std::vec::Vec`
- `std::boxed::Box`
- `std::option::Option::{self, Some, None}`
- `std::result::Result::{self, Ok, Err}`
- `std::marker::{Copy, Send, Sized, Sync}`
- `std::default::Default`
- `std::ops::{Drop, Fn, FnMut, FnOnce}`
- `std::mem::drop`
- `std::clone::Clone`
- `std::borrow::ToOwned`
- `std::cmp::{PartialEq, PartialOrd, Eq, Ord }`
- `std::convert::{AsRef, AsMut, Into, From}`
- `std::iter::{Iterator, IntoIterator, DoubleEndedIterator,ExactSizeIterator, Extend}`
