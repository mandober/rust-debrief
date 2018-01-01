# Structs

A struct is a nominal struct type defined with the keyword `struct`.

An example of a `struct` item and its use:

```rust
struct Point {x: i32, y: i32}
let p = Point {x: 10, y: 11};
let px: i32 = p.x;
```

A tuple struct is a nominal tuple type, also defined with the keyword struct.

For example:

```rust
struct Point(i32, i32);
let p = Point(10, 11);
let px: i32 = match p { Point(x, _) => x };
```

A unit-like struct is a struct without any fields, defined by leaving off the list of fields entirely. Such a struct implicitly defines a constant of its type with the same name. For example:

```rust
struct Cookie;
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
```

is equivalent to

```rust
struct Cookie {}
const Cookie: Cookie = Cookie {};
let c = [Cookie, Cookie {}, Cookie, Cookie {}];
```

The precise memory layout of a struct is not specified. One can specify a particular layout using the repr attribute.