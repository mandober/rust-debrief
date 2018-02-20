# Dynamic dispatch

Sometimes, we want to take the decision of which implementation to use at runtime instead of letting the compiler monomorphize the code.

There's two approaches to that.

## Dispatch through Enums
If the number of possible choices is limited, an Enum can be used:

```rust
enum Operation {
    Get,
    Set(String),
    Count
}

fn execute(op: Operation) {
    match op {
        Operation::Get => execute_get(),
        Operation::Set(s) => execute_set(s),
        Operation::Count => execute_count()
    }
}
```

Alternative Form

```rust
enum Operation {
    Get,
    Set(String),
    Count
}

impl Operation {
    fn execute(&self) {
        match &self {
            &Operation::Get => execute_get(),
            &Operation::Set(s) => execute_set(s),
            &Operation::Count => execute_count()
        }
    }
}
```
