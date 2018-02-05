# `Deref` trait
https://doc.rust-lang.org/std/ops/trait.Deref.html

Trait `std::ops::Deref` 1.0.0

Used for immutable dereferencing operations, like `*v`.

```rust
#[lang = "deref"]
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

In addition to being used for explicit dereferencing operations with the (unary)`*` operator in immutable contexts, `Deref` is also used implicitly by the compiler in many circumstances. This mechanism is called _Deref coercion_. In mutable contexts, `DerefMut` is used.

Implementing `Deref` for smart pointers makes accessing the data behind them convenient, which is why they implement `Deref`. On the other hand, the rules regarding `Deref` and `DerefMut` were designed specifically to accommodate smart pointers. Because of this, `Deref` should only be implemented for smart pointers to avoid confusion.

For similar reasons, this trait should never fail. Failure during dereferencing can be extremely confusing when `Deref` is invoked implicitly.


## Deref coercion

If `T` implements `Deref<Target = U>`, 
and `x` is a value of type `T`,
then:
in immutable contexts, `*x` on non-pointer
types is equivalent to `*Deref::deref(&x)`.

Values of type `&T` are coerced to values of type `&U`
`T` implicitly implements all the (immutable) methods of the type `U`.


## Examples
A struct with a single field which is accessible by dereferencing the struct.

```rust
use std::ops::Deref;

struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

let x = DerefExample { value: 'a' };
assert_eq!('a', *x);
```

## Associated Types
```rust
type Target: ?Sized
```
The resulting type after dereferencing.


## Required Methods
```rust
fn deref(&self) -> &Self::Target
```
Dereferences the value.



## Implementors

```rust
impl<'b, T> Deref for Ref<'b, T>
  where T: ?Sized
  type Target = T;

impl<'a, T> Deref for &'a T
  where T: ?Sized,
  type Target = T;

impl<'a, T> Deref for &'a mut T
where    T: ?Sized, 
  type Target = T;

impl<T> Deref for ManuallyDrop<T>
  type Target = T;

impl<'b, T> Deref for RefMut<'b, T>
where    T: ?Sized, 
  type Target = T;

impl<T> Deref for Rc<T> 
where    T: ?Sized, 
  type Target = T;

impl<'a, B> Deref for Cow<'a, B> 
where    B: ToOwned + ?Sized, 
  type Target = B;

impl<T> Deref for Arc<T> 
where    T: ?Sized, 
  type Target = T;

impl<'a, T> Deref for PeekMut<'a, T> 
where    T: Ord, 
  type Target = T;

impl<T> Deref for Box<T> 
where    T: ?Sized, 
  type Target = T;

impl Deref for String
  type Target = str;

impl<T> Deref for Vec<T>
  type Target = [T];

impl Deref for CString
  type Target = CStr;

impl Deref for OsString
  type Target = OsStr;

impl<T> Deref for AssertUnwindSafe<T>
  type Target = T;

impl Deref for PathBuf
  type Target = Path;

impl<'mutex, T: ?Sized> Deref for MutexGuard<'mutex, T>
  type Target = T;

impl<'rwlock, T: ?Sized> Deref for RwLockReadGuard<'rwlock, T>
  type Target = T;

impl<'rwlock, T: ?Sized> Deref for RwLockWriteGuard<'rwlock, T>
  type Target = T;

impl<T: ?Sized> Deref for Box<T>

impl<T: ?Sized> Deref for Arc<T>

impl<T: ?Sized> Deref for Rc<T>

impl<'a, T: Ord> Deref for PeekMut<'a, T>

impl<'a, B: ?Sized> Deref for Cow<'a, B> 
where    B: ToOwned, 

impl Deref for String

impl<T> Deref for Vec<T>
```
