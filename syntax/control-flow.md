# Control flow


## if

```rust
let value = 2;

if value % 2 == 0 {
    // ...
} else if value == 5 {
    // ...
} else { /* ... */ }
```


## match

```rust
let maybe_value = Some(2);
match maybe_value {
    Some(value) if value == 2 => {
        // ...
    }
    Some(value) => {
        // ...
    },
    None => {
        // ...
    },
}
```


## if let

```rust
let maybe_value = Some(2);
    
if let Some(value) = maybe_value {
    // ...
} else { /* ... */ }
```


## loop

```rust
let mut value = 0;
loop {
    if value >= 10 {
        break;
    }
    value += 1;
}
```

## while

```rust
while value <= 10 {
    value += 1;
    // ...
}
```


## for

```rust
let range = 0..10;
for i in range {
    // ...
}
```

## while let

```rust
let mut range = 0..10;
while let Some(v) = range.next() {
    // ...
}
```

