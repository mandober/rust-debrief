# HashMap methods

## Methods

```rust
// new
new
with_capacity
with_hasher
with_capacity_and_hasher
// post creation capacity manipulation
reserve
shrink_to_fit
// query meta
hasher
capacity
len
is_empty
// query
keys
contains_key
values
values_mut
iter
iter_mut
// create
insert
// read
get
get_mut
// update
entry
retain
// delate
remove
remove_entry
clear
drain
```

## Trait Implementations

```rust
Clone
PartialEq
Eq
Debug
Default
Index<&'a Q>
IntoIterator
FromIterator<(K, V)>
Extend<(K, V)>
Extend<(&'a K, &'a V)>
```

## Auto Trait Implementations

```rust
Send
Sync
```
