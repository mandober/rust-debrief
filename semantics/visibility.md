# Visibility

- `pub` is the only visibility qualifier at the moment; everything is private by default, items qualified with `pub` are public i.e. visible outside its namespace.

`pub` can be used on:
- `mod`
- `fn`
- `use` (re-exporting)
- `struct`
- individual `struct` fields
- `enum` (not on its variants)
