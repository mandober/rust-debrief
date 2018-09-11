enum Result

- Result is a type that represents either success (Ok) or failure (Err).
- https://doc.rust-lang.org/nightly/std/result/enum.Result.html
- `Result<T, E>` is the type used for returning and propagating errors.
- Variants:
  - `Ok(T)` contains the success value
  - `Err(E)` contains the error value



```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Enums
`Result`   type that represents either success (Ok) or failure (Err).

## Structs
`IntoIter` iterator over the value in a Ok variant of a Result.
`Iter`     iterator over a reference to the Ok variant of a Result.
`IterMut`  iterator over a mutable reference to the Ok variant of a Result.

## Methods on Result enum

`impl<T, E> Result<T, E>`
- is_ok
- is_err
- ok
- err
- as_ref
- as_mut
- map
- map_err
- iter
- iter_mut
- and
- and_then
- or
- or_else
- unwrap_or
- unwrap_or_else

`impl<T, E> Result<T, E> (where E: Debug)`
- unwrap
- expect
- unwrap_err
- expect_err
- unwrap_or_default

