# Module std::cmp
1.0.0
https://doc.rust-lang.org/stable/std/cmp/index.html

Functionality for ordering and comparison.

```tcc {cmd=true output="html"}
dir
```

```javascript {cmd="node.exe" output="markdown"}
const date = Date.now()
console.log(date.toString())
```

```python {cmd="py.exe" args=["-3"]}
print("This will run python3 program")
```

This module defines both `PartialOrd` and `PartialEq` traits which are used by 
the compiler to implement comparison operators. Rust programs may implement 
`PartialOrd` to overload the <, <=, >, and >= operators, and may implement 
`PartialEq` to overload the == and != operators.


@import "comparison.rs" {as="rust" code_block=true class="line-numbers"}


```rust
let x: u32 = 0;
let y: u32 = 1;

// these two lines are equivalent
assert_eq!(x < y, true);
assert_eq!(x.lt(&y), true);

// these two lines are also equivalent
assert_eq!(x == y, false);
assert_eq!(x.eq(&y), false);
```

## Structs

`Reverse`
A helper struct for reverse ordering.

## Enums

`Ordering`
An Ordering is the result of a comparison between two values.

## Traits

`Eq`
Trait for equality comparisons which are equivalence relations.

`Ord`
Trait for types that form a total order.

`PartialEq`
Trait for equality comparisons which are partial equivalence relations.

`PartialOrd`
Trait for values that can be compared for a sort-order.

## Functions

`max`
Compares and returns the maximum of two values.

`min`
Compares and returns the minimum of two values.