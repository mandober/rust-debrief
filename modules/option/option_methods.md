# Option methods


`is_some` returns true if the option is a `Some` value.
`is_none` returns true if the option is a `None` value.

`as_ref` Converts from `Option<T>` to `Option<&T>`.
`as_mut` Converts from `Option<T>` to `Option<&mut T>`.

`expect` Unwraps an option, yielding the content of a `Some`.
`unwrap` Moves the value `v` out of the `Option<T>` if it is `Some(v)`.
`unwrap_or` Returns the contained value or a default.
`unwrap_or_else` Returns the contained value or computes it from a closure.

`unwrap_or_default` Returns the contained value or a default. Consumes the self 
argument then, if Some, returns the contained value, otherwise if None, returns 
the default value for that type.


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
wrapped value and returns the result. Some langs call this operation *flatmap*.

`or` Returns the option if it contains a value, otherwise returns optb.
`or_else` Returns the option if it contains a value, otherwise calls f and returns the result.

`get_or_insert` Inserts v into the option if it is None, then returns a mutable 
reference to the contained value.

`get_or_insert_with` Inserts a value computed from f into the option if it is 
None, then returns a mutable reference to the contained value.

`take` Takes the value out of the option, leaving a None in its place.

`cloned` Maps an Option<&T> to an Option<T> by cloning the contents of the option.
