# Pointer types

Pointer (or reference) types are types that allows indirect access to data. Rust has 5 primitive pointer types:
- Reference (shared and mutable)
- Slice
- String slice
- Function pointer
- Raw pointer (shared and mutable)

Other, non-primitive, pointer types are called **smart pointers** and they include types such as `Vec`, `String`, `Box`, `Cell`, `RefCell`, `Rc`, etc.



- All pointer types in Rust are explicit first-class values.
- They can be copied, stored into data structs, and returned from functions.

There are 2 varieties of pointer in Rust:

- References (&T)
  These point to memory owned by some other value. A reference type is written `&T`, or `&'a type` when you need to specify an explicit lifetime. Copying a reference is a "shallow" operation: it involves only copying the pointer itself. Releasing a reference has no effect on the value it points to, but a reference of a temporary value will keep it alive during the scope of the reference itself.

- Raw pointers (*const T or *mut T)
  Raw pointers are pointers without safety or liveness guarantees. Raw pointers are written as `*const T` or `*mut T`, for example `*const i32` means a raw pointer to a 32-bit integer. Copying or dropping a raw pointer has no effect on the lifecycle of any other value. Dereferencing a raw pointer or converting it to any other pointer type is an unsafe operation. Raw pointers are generally discouraged in Rust code; they exist to support interoperability with foreign code, and writing performance critical or low-level functions.


---
There are two kinds of reference:
- Shared reference: `&`
- Mutable reference: `&mut`
which must obey these rules:
1. A reference cannot outlive its referent
2. A mutable reference cannot be aliased

All pointers in Rust are explicit first-class values.
They can be copied, stored into data structs, and returned from functions.
Copying a reference is a shallow operation: it involves only copying the pointer itself.


- more abstract type than pointers
- in Rust called borrowing: 
  may be immutable or mutable:
  immutable borrowing involves *shared reference* `&T`
  mutable borrowing involves *mutable reference* `&mut T`
- `&` operator takes a (mut) ref of some type `T`: `&T` or `&mut T`
  `ref` keyword can also be used to take a (mut) ref: `ref T` or `ref mut T`
  `T` => `&T`, `*&T` == `T`
- A mutable reference to an immutable variable is forbidden
- When data is immutably borrowed, it also freezes.
  Frozen data can't be modified via the original object 
  until all references to it go out of scope


RULES: at one same time, a value can hand out:
- one or more immutable references (shared references) OR
- exactly one mutable reference

- You can’t move out of a borrowed value, because after the borrow ends the 
  value must stay valid. You can’t move out of it even if you move something 
  back (...unless you use mem::swap).
- If you want an owning pointer  use Box.
- If you want some basic reference counting  use Rc.
- If you really need to get around the restrictions Rust puts on you, use unsafe.


```rust
let x = 42;
// take a ref with &
let r = &x;
// with explicit type annotation
let r: &i32 = &x;
// if using ref keyword then no &
let ref r = x;
// with explicit type annotation
let ref r: &i32 = x;

// mut refs
let mut a = 42;
let mut x = 5;
{
  let b = &mut a;
  let y = &mut x;
  // y needs to be deref'ed cuz we're assigning a new value to the underlying
  // value, not re-binding y itself, which wouldn't work anyway since y is not 
  // mutable; even if it was, it was already type-locked to &i32 so we couldn't
  // assign i32 to it anyway.
  *y = 1;   // x is now 1
  *b += 7;  // a is now 49
  *y += *b; // x is now 50
}
assert!(x == 50);

// dereferencing
let c = 6;
let d = &c;
assert!(*d == 6);
```
Above, `y` needs to be dereferenced because we're changing the underlying value,
not re-binding `y` itself; re-binding `y` wouldn't work anyway since `y` is not 
mutable; and even if it was, it was already type-locked to `&i32` so we couldn't
assign `i32` to it anyway.

