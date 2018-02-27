# Formatting 


## Macros
Macros that use formatting:
- `print`       printing to stout
- `println`     to stout, with newline i.e. LINE FEED (\n, U+000A)
- `eprint`      printing to sterr
- `eprintln`    printing to sterr, with a newline
- `write`       write formatted data into a buffer
- `writeln`     write formatted data into a buffer, newline appended
- `format`      create a String
- `format_args` formatted string creation and output


## Placeholder
Format placeholder is `{}`.
- display: `{}` Type must implement `Display` trait
- debug: `{:?}` Type must implement `Debug` trait

## Formats
- display: `{}`
- debug: `{:?}`
- pretty debug: `{:#?}`
- pointer: `{:p}`

## Numbers
- binary: `{:b}`
- octal: `{:o}`
- lower hexadecimal: `{:x}`
- upper hexadecimal: `{:X}`
- lower exponential notation: `{:e}`
- upper exponential notation: `{:E}`
- decimals: `{:+n.m}` n is width, m is precision
- width: `{:n}` n is total width of output in characters
- precision: `{:.m}` n is number of digits after the point
- force sign: `{:+n.m}`




## Examples

```rust
let dec = 3.2f32;
println!("{:+007.2}", dec); // +003.20
// where:
//  +00 = literal text
//    7 = total character width of output
//   .2 = 2 digits after decimal point
```
