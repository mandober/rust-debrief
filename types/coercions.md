# Type coercions

```rust
let a = [1, 2];
let v1: &[i32; 2] = &a;
let arr = v1.to_owned();
let v2: &[i32] = &a;
let vec = v2.to_owned();
//let v3: &Vec<i32> = &a;
```

