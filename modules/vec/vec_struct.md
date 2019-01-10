# struct `Vec`
https://doc.rust-lang.org/std/vec/struct.Vec.html

Struct `std::vec::Vec` 1.0.0

Vector, `Vec<T>`, a contiguous growable array type.

```rust
pub struct Vec<T> { /* fields omitted */ }

// from src:
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}
```


## Examples

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);
assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);
vec.extend([1, 2, 3].iter().cloned());

for x in &vec {
    println!("{}", x);
}
assert_eq!(vec, [7, 1, 2, 3]);
```

## vec!

The `vec!` macro is provided to make initialization more convenient:

```rust
let mut vec = vec![1, 2, 3];
vec.push(4);
assert_eq!(vec, [1, 2, 3, 4]);


// It can also initialize each element of a Vec<T> with a given value:
let vec = vec![0; 5];
assert_eq!(vec, [0, 0, 0, 0, 0]);


// Use a Vec<T> as an efficient stack:
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);
while let Some(top) = stack.pop() {
    // Prints 3, 2, 1
    println!("{}", top);
}
```

## Indexing

The Vec type allows to access values by index, because it implements the `Index` trait.

```rust
let v = vec![0, 2, 4, 6];
println!("{}", v[1]); // it will display '2'
```

If you try to access an index which isn't in the Vec you get a *panic*!

```rust
let v = vec![0, 2, 4, 6];
println!("{}", v[6]); // panic!
```
Always check if the index really exists.


## Slicing

A Vec can be mutable.
Slices, on the other hand, are *read-only* objects. To get a slice, use `&`.

```rust
fn read_slice(slice: &[usize]) {
    // ...
}

let v = vec![0, 1];
read_slice(&v);

// you can also do it like this:
let x: &[usize] = &v;
```

In Rust, it's more common to pass slices as arguments rather than vectors when 
you just want to provide a *read access*. The same goes for `String` and `&str`.


## Capacity and reallocation
The capacity of a vec is the amount of space allocated for any future elements 
that will be added onto the vector. If a vector's length exceeds its capacity, 
its capacity will automatically be increased, but its elements will have to be 
reallocated. It is recommended to use `Vec::with_capacity` whenever possible.


## Guarantees
Fundamentally, Vec is and always will be a *pointer, capacity, length* triplet.
The order of these fields is completely unspecified, and you should use the 
appropriate methods to modify these. 
The pointer will never be null, so this type is *null-pointer-optimized*.

Vec will allocate if and only if
`mem::size_of::<T>() * capacity() > 0`

If a Vec did allocate memory, then the memory it points to is on the heap, and 
its pointer points to `len` initialized elements in order, followed by 
`capacity - len` logically uninitialized elements.

Vec will never perform a "small optimization"
where elements are actually stored on the stack.

Vec will never automatically shrink itself, even if completely empty.
Emptying a Vec and then filling it back up to the same `len` should incur no 
calls to the allocator. If you wish to free up unused memory, use `shrink_to_fit`.

`push` and `insert` will never (re)allocate if the reported 
capacity is sufficient, but they will (re)allocate if `len==capacity`.
The reported capacity is completely accurate, and can be relied on.
Bulk insertion methods may reallocate, even when not necessary.

Vec does not guarantee any particular growth strategy
when reallocating when full, nor when `reserve` is called.

`vec![x; n]`, `vec![a, b, c, d]`, and `Vec::with_capacity(n)`, will all produce 
a Vec with exactly the requested capacity. If len==capacity, (as is the case for 
the vec! macro), then a `Vec<T>` can be converted to and from a `Box<[T]>` without 
reallocating or moving the elements.

using `unsafe` code to write to the excess capacity,
and then increasing the length to match, is always valid.

Vec does not currently guarantee the order in which elements are dropped.
