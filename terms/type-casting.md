# Type Casting

Cast is performed between applicable types using the binary operator `as`

```rust
let x: i32 = 100;
let y: u32 = x as u32;
```

You can only cast between types that are safe to cast between. It is forbidden to cast `[i16; 4]` to `char`; this is called a **non-scalar cast** (although there are unsafe mechanisms to overcome it).

```rust
let points = 10i32;
let mut saved_points: u32 = 0;
saved_points = points as u32;
```

If points was negative, the sign would be lost after casting to `u32`; when casting from a wider value like a float to an integer, the decimal part is truncated:

```rust
let f2 = 3.14;
saved_points = f2 as u32; // truncated to 3
```
