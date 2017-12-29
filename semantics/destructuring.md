# Destructuring

## Destructuring tuple

```rust
let pair = (1, true);
let (i, b) = pair;
println!("Destructured {:?} and {:?}", i, b);
```


## Destructuring struct

```rust
struct Point {
    x: f32,
    y: f32,
}

// Destructure the point using a `let` binding
let Point { x: a, y: b } = point;

println!("Destructured {:?} and {:?}", a, b);
//: Destructured 0.3 and 0.4
```
