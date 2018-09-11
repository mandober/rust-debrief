# Option methods


- `is_some` true if `Some`
- `is_none` true if `None`
- `as_ref`    from `Option<T>` to `Option<&T>`
- `as_mut`    from `Option<T>` to `Option<&mut T>`
- `expect`            move `v` out of `Some(v)` or panic with a message
- `unwrap`            move `v` out of `Some(v)` or panic
- `unwrap_or`         return `v` or a default
- `unwrap_or_else`    return `v` or compute it from a closure.
- `unwrap_or_default` returns `v` or a default.

consumes the self argument then, if Some, returns the contained value, otherwise if None, returns the default value for that type.



`map` Maps an `Option<T>` to `Option<U>` by applying a fn to a contained value.
`map_or` Apply fn to the contained value (if any), or returns a default (if not).
`map_or_else` Apply fn to the contained value (if any), or computes a default (if not).

`ok_or` Transforms the `Option<T>` into a `Result<T, E>`,
        via map: `Some(v)` -> `Ok(v)` and `None` -> `Err(err)`

`ok_or_else` Transforms the `Option<T>` into a `Result<T, E>`,
             mapping `Some(v)` to `Ok(v)` and `None` to `Err(err())`.

`iter` Returns an iterator over the possibly contained value.
`iter_mut` Returns an mutable iterator over the possibly contained value.

`and` Returns `None` if the option is `None`, otherwise returns `optb`.
`and_then` Returns `None` if the option is `None`, otherwise calls `f` with the 
wrapped value and returns the result.

`or` Returns the option if it contains a value, otherwise returns optb.
`or_else` Returns the option if it contains a value, otherwise calls f and returns the result.

`get_or_insert` Inserts v into the option if it is None, then returns a mutable 
reference to the contained value.

`get_or_insert_with` Inserts a value computed from f into the option if it is 
None, then returns a mutable reference to the contained value.

`take` Takes the value out of the option, leaving a None in its place.

`cloned` Maps an Option<&T> to an Option<T> by cloning the contents of the option.
