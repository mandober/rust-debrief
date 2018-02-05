# Option: implemented traits

- Clone, Copy, Hash, Debug, Default, From,
- Eq, PartialEq, Ord, PartialOrd,
- IntoIterator, FromIterator,
- Try (LAB)


# Implemented traits

<!-- TOC -->

- [Default: default](#default-default)
- [From: from](#from-from)
- [Debug: fmt](#debug-fmt)
- [Clone](#clone)
  - [clone](#clone)
  - [clone_from](#clone_from)
- [Hash](#hash)
  - [hash](#hash)
  - [hash_slice](#hash_slice)
- [PartialEq](#partialeq)
  - [eq](#eq)
  - [ne](#ne)
- [Ord](#ord)
  - [cmp](#cmp)
  - [max](#max)
  - [min](#min)
- [PartialOrd](#partialord)
  - [partial_cmp](#partial_cmp)
  - [lt](#lt)
  - [le](#le)
  - [gt](#gt)
  - [ge](#ge)
- [IntoIterator: into_iter](#intoiterator-into_iter)
  - [impl1: into_iter](#impl1-into_iter)
  - [impl2: into_iter](#impl2-into_iter)
  - [impl3: into_iter](#impl3-into_iter)
- [FromIterator: from_iter](#fromiterator-from_iter)
- [Try](#try)
  - [into_result](#into_result)
  - [from_ok](#from_ok)
  - [from_error](#from_error)

<!-- /TOC -->



The `Option` type ([src](https://doc.rust-lang.org/stable/src/core/option.rs.html)):

```rust
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Option<T> {
    None,    // No value
    Some(T), // Some value T
}
```



## Default: default
Returns None.

```rust
impl<T> Default for Option<T> {
  fn default() -> Option<T>;
}
```


## From: from
Performs the conversion.

```rust
impl<T> From<T> for Option<T> {
  fn from(val: T) -> Option<T>; // 1.12.0
}
```


## Debug: fmt
Formats the value using the given formatter.

```rust
impl<T> Debug for Option<T> where T: Debug {
  fn fmt(&self, __arg_0: &mut Formatter) -> Result<(), Error>;
}
```



## Clone

```rust
impl<T> Clone for Option<T> 
 where T: Clone
{
  fn clone(&self) -> Option<T>;
  fn clone_from(&mut self, source: &Self)
}
```

### clone
Returns a copy of the value.

### clone_from
Performs copy-assignment from source.




## Hash

```rust
impl<T> Hash for Option<T>
 where T: Hash
{
  fn hash<__HT>(&self, __arg_0: &mut __HT) where __HT: Hasher;
  fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher;
}
```

### hash
Feeds this value into the given [Hasher].

### hash_slice
Feeds a slice of this type into the given [Hasher]. Since 1.3.0.





## PartialEq

```rust
impl<T> PartialEq<Option<T>> for Option<T> where T: PartialEq<T> {
  fn eq(&self, __arg_0: &Option<T>) -> bool;
  fn ne(&self, __arg_0: &Option<T>) -> bool;
}
```

### eq
This method tests for self and other values to be equal, and is used by `==`

### ne
This method tests for `!=`.





## Ord

```rust
impl<T> Ord for Option<T>
 where T: Ord
{
  fn cmp(&self, __arg_0: &Option<T>) -> Ordering;
  fn max(self, other: Self) -> Self; // 1.21.0
  fn min(self, other: Self) -> Self; // 1.21.0
}
```

### cmp
This method returns an Ordering between self and other.

### max
Compares and returns the maximum of two values. Since: 1.21.0

### min
Compares and returns the minimum of two values. Since: 1.21.0





## PartialOrd

```rust
impl<T> PartialOrd<Option<T>> for Option<T> 
  where T: PartialOrd<T>
{
  fn partial_cmp(&self, __arg_0: &Option<T>) -> Option<Ordering>;
  fn lt(&self, __arg_0: &Option<T>) -> bool;
  fn le(&self, __arg_0: &Option<T>) -> bool;
  fn gt(&self, __arg_0: &Option<T>) -> bool;
  fn ge(&self, __arg_0: &Option<T>) -> bool
}
```

### partial_cmp
This method returns an ordering between `self` and `other` values if one exists.

### lt
This method tests less than (for `self` and `other`) and is used by the `<` operator.

### le
This method tests less than or equal to (for `self` and `other`) and is used by the `<=` operator.

### gt
This method tests greater than (for `self` and `other`) and is used by the `>` operator.

### ge
This method tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator.






## IntoIterator: into_iter

```rust
impl    <T> IntoIterator for         Option<T> {};
impl<'a, T> IntoIterator for &'a     Option<T> {}; // 1.4.0
impl<'a, T> IntoIterator for &'a mut Option<T> {}; // 1.4.0
```

### impl1: into_iter
Returns a consuming iterator over the possibly contained value.

```rust
impl<T> IntoIterator for Option<T> {
  // The type of the elements being iterated over.
  type Item = T;
  
  // Which kind of iterator are we turning this into?
  type IntoIter = IntoIter<T>;

  // Returns a consuming iterator over the possibly contained value.
  fn into_iter(self) -> IntoIter<T>;

}
```

Examples

```rust
let x = Some("string");
let v: Vec<&str> = x.into_iter().collect();
assert_eq!(v, ["string"]);

let x = None;
let v: Vec<&str> = x.into_iter().collect();
assert!(v.is_empty());
```

### impl2: into_iter
Creates an iterator from a value. Since 1.4.0.

```rust
impl<'a, T> IntoIterator for &'a Option<T> { // 1.4.0
  //The type of the elements being iterated over.
  type Item = &'a T;

  // Which kind of iterator are we turning this into?
  type IntoIter = Iter<'a, T>;

  // Creates an iterator from a value.
  fn into_iter(self) -> Iter<'a, T>;
}
```


### impl3: into_iter
Creates an iterator from a value. Since 1.4.0.

```rust
impl<'a, T> IntoIterator for &'a mut Option<T> { // 1.4.0
  // The type of the elements being iterated over.
  type Item = &'a mut T;

  // Which kind of iterator are we turning this into?
  type IntoIter = IterMut<'a, T>;

  // Creates an iterator from a value.
  fn into_iter(self) -> IterMut<'a, T>;
}
```


## FromIterator: from_iter

Takes each element in the `Iterator`: 
- if it is `None`, no further elements are taken, and `None` is returned.
- if no `None` occurs, a container with the values of each `Option` is returned.

```rust
impl<A, V> FromIterator<Option<A>> for Option<V> 
 where V: FromIterator<A>
{
  fn from_iter<I>(iter: I) -> Option<V> 
    where I: IntoIterator<Item = Option<A>>;
 }
```

Example: increment every integer in a vector, checking for overflow

```rust
use std::u16;
let v = vec![1, 2];
let res: Option<Vec<u16>> = v.iter()
                             .map(|&x: &u16|
                                  if x == u16::MAX { None }
                                  else { Some(x + 1) }
                            ).collect();
assert!(res == Some(vec![2, 3]));
```




## Try

- `Try` trait [docs](https://doc.rust-lang.org/stable/std/ops/trait.Try.html)
- This is a __nightly-only__ experimental API 
  ([try_trait #42327](https://github.com/rust-lang/rust/issues/42327))

```rust
impl<T> Try for Option<T> {
  // The type of this value when viewed as successful.
  type Ok = T;
  // The type of this value when viewed as failed.
  type Error = NoneError;

  fn into_result(self) -> Result<T, NoneError>;
  fn from_ok(v: T) -> Option<T>;
  fn from_error(NoneError) -> Option<T>;
}
```

### into_result
Applies the "?" operator. A return of `Ok(t)` means that the execution should continue normally, and the result of `?` is the value `t`. A return of `Err(e)` means that execution should branch to the innermost enclosing catch, or return from the function.

### from_ok
Wrap an `Ok` value to construct the composite result. 
For example, `Result::Ok(x)` and `Result::from_ok(x)` are equivalent.

### from_error
Wrap an `Err` value to construct the composite result. 
For example, `Result::Err(x)` and `Result::from_error(x)` are equivalent.
