- Feature Name: `repr_packed`
- Start Date: 2015-12-06
- RFC PR: [rust-lang/rfcs#1399](https://github.com/rust-lang/rfcs/pull/1399)
- Rust Issue: [rust-lang/rust#33158](https://github.com/rust-lang/rust/issues/33158)

# Summary
[summary]: #summary

Extend the existing `#[repr]` attribute on structs with a `packed = "N"` option to
specify a custom packing for `struct` types.

# Motivation
[motivation]: #motivation

Many C/C++ compilers allow a packing to be specified for structs which
effectively lowers the alignment for a struct and its fields (for example with
MSVC there is `#pragma pack(N)`). Such packing is used extensively in certain
C/C++ libraries (such as Windows API which uses it pervasively making writing
Rust libraries such as `winapi` challenging).

At the moment the only way to work around the lack of a proper
`#[repr(packed = "N")]` attribute is to use `#[repr(packed)]` and then manually
fill in padding which is a burdensome task. Even then that isn't quite right
because the overall alignment of the struct would end up as 1 even though it
needs to be N (or the default if that is smaller than N), so this fills in a gap
which is impossible to do in Rust at the moment.

# Detailed design
[design]: #detailed-design

The `#[repr]` attribute on `struct`s will be extended to include a form such as:

```rust
#[repr(packed = "2")]
struct LessAligned(i16, i32);
```

This structure will have an alignment of 2 and a size of 6, as well as the
second field having an offset of 2 instead of 4 from the base of the struct.
This is in contrast to without the attribute where the structure would have an
alignment of 4 and a size of 8, and the second field would have an offset of 4
from the base of the struct.

Syntactically, the `repr` meta list will be extended to accept a meta item
name/value pair with the name "packed" and the value as a string which can be
parsed as a `u64`. The restrictions on where this attribute can be placed along
with the accepted values are:

* Custom packing can only be specified on `struct` declarations for now.
  Specifying a different packing on perhaps `enum` or `type` definitions should
  be a backwards-compatible extension.
* Packing values must be a power of two.

By specifying this attribute, the alignment of the struct would be the smaller
of the specified packing and the default alignment of the struct. The alignments
of each struct field for the purpose of positioning fields would also be the
smaller of the specified packing and the alignment of the type of that field. If
the specified packing is greater than or equal to the default alignment of the
struct, then the alignment and layout of the struct should be unaffected.

When combined with `#[repr(C)]` the size alignment and layout of the struct
should match the equivalent struct in C.

`#[repr(packed)]` and `#[repr(packed = "1")]` should have identical behavior.

Because this lowers the effective alignment of fields in the same way that
`#[repr(packed)]` does (which caused [issue #27060][gh27060]), while accessing a
field should be safe, borrowing a field should be unsafe.

Specifying `#[repr(packed)]` and `#[repr(packed = "N")]` where N is not 1 should
result in an error.

Specifying `#[repr(packed = "A")]` and `#[repr(align = "B")]` should still pack
together fields with the packing specified, but then increase the overall
alignment to the alignment specified. Depends on [RFC #1358][rfc1358] landing.

# Drawbacks
[drawbacks]: #drawbacks

# Alternatives
[alternatives]: #alternatives

* The alternative is not doing this and forcing people to continue using
  `#[repr(packed)]` with manual padding, although such structs would always have
  an alignment of 1 which is often wrong.
* Alternatively a new attribute could be used such as `#[pack]`.

# Unresolved questions
[unresolved]: #unresolved-questions

* The behavior specified here should match the behavior of MSVC at least. Does
  it match the behavior of other C/C++ compilers as well?
* Should it still be safe to borrow fields whose alignment is less than or equal
  to the specified packing or should all field borrows be unsafe?

[gh27060]: https://github.com/rust-lang/rust/issues/27060
[rfc1358]: https://github.com/rust-lang/rfcs/pull/1358
