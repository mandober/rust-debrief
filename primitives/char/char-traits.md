# char trait implementations

`char` implements these traits

```rust
Display
Debug
Default
Hash
TryFrom<u32>
FromStr
From<u8>
Pattern<'a>
AsciiExt
Eq
PartialEq<char>
Ord
PartialOrd<char>
```

## Display
Formats the value using the given formatter.

```rust
impl Display for char {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
}
```

