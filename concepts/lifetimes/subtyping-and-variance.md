# Subtyping and variance

- subtyping on lifetimes is in terms of _outlives_ relation:
- `'a : 'b` means `'a` outlives `'b`
- bigger scope is a subtype of smaller: pass `'static` when `'a` is expected.

- Higher-ranked lifetimes (HRL) are subtypes of every concrete lifetime:   
  taking arbitrary lifetime is strictly more general than taking a specific one.

- Variance is a property that type ctors have with respect to their params.
- Type constructor is a generic type with unbound params:
  - `Vec` takes `T` and returns `Vec<T>`
  - `&` and `&mut` take 2 inputs: a lifetime and a type to point to
- Type constructor's variance is how the subtyping of its inputs affects the   subtyping of its outputs. 



Variance
- `F` is variant over `T`, if `T`, being a subtype of `U`, implies
  `F<T>` is a subtype of `F<U>` (subtyping "passes through")

Invariance
- `F` is invariant over `T` otherwise (no subtyping relation can be derived)



## Subtyping

Subtyping derives entirely from lifetimes. Since lifetimes are scopes, we can partially order them based on the "outlives" relation. We can even express this as a generic bound.

Subtyping on lifetimes is in terms of that relation:   
if `'a: 'b` i.e. if "a outlives b" ("a contains b")   
then `'a` is a subtype of `'b`.

The bigger scope is a subtype of the smaller scope:   
you can provide `'static` when at least `'a` is expected.

Higher-ranked lifetimes (HRL) are subtypes of every concrete lifetime.

This is because taking an arbitrary lifetime is strictly more general than taking a specific one.


## Variance

Variance is a property that type constructors have with respect to their args.
A type constructor in Rust is a generic type with unbound args.

For instance `Vec` is a type constructor that takes `T` and returns `Vec<T>`.
`&` and `&mut` are type constructors that take 2 inputs: a lifetime, and a type to point to.

A type constructor's variance is how subtyping of its inputs affects subtyping of its outputs. 

`F` is _variant_ over `T` if `T`, being a subtype of `U`, implies `F<T>` is a subtype of `F<U>` (subtyping "passes through").

`F` is _invariant_ over `T` otherwise (no subtyping relation can be derived)

`fn(T)` is _contravariant_ in `T`, which is used to match the methods in trait implementation to the methods in trait definition.

Traits don't have inferred variance: `Fn(T)` is _invariant_ in `T`.



## Important variance cases

1. `&'a T` is variant over `'a` and `T` (as is `*const T` by metaphor)
2. `&'a mut T` is variant over `'a` but invariant over `T`
3. `Box<T>`, `Vec<T>`, etc. are variant over `T`
4. `UnsafeCell<T>`, `Cell<T>`, `RefCell<T>`, `Mutex<T>` and all other interior
    mutability types are invariant over `T` (as is `*mut T` by metaphor)
5. `Fn(T) -> U` is invariant over `T`, but variant over `U`


details:


### 1. `&'a T` is variant over `'a` and `T`

`&'a T` is variant over `'a` and `T` (as is `*const T` by metaphor):

* `&'a T` is variant over `'a`
  we can pass longer where shorter lifetime is expected
  e.g. `'static` when at least `'a` is expected.

* `&'a T`: variant over `T`
  we can pass `&&'static str` where `&&'a str` is expected. The __additional levels of indirection__ should not prevent us from being able to pass longer where shorter lifetime is expected.


Shared ref to `T` with a lifetime `'a` is variant over both, lifetime and type
(as long as "type" means more indirection levels of original type).

`&'a T` is variant over `'a` and `T`
1. `&'a T` is variant over `'a` because  `&'static T`   for  `&'a T`
2. `&'a T` is variant over `T`  because `&&'static str` for `&&'a str`



`&&'static str` for `&&'a str`
`& &'static T`   for `&&'a T`

U = `&'static T`

T = 



given         | expected   | variance
--------------|------------|---------------
&'a T is variant over 'a and T:
* &'a T variant over 'a

  'static T   |   'a T     | &'a T variant over 'a
  
&&'static str | &&'a str   | &'a T variant over T

  & 'a      T    is variant over       T
& & 'static str  is variant over  &&'a str
& & 'a T         is variant over  & & 'a T




### 2. `&'a mut T`: variant over `'a`, invariant over `T`

To see why` &mut` should be invariant over `T`, consider the following code:

```rust
fn overwrite<T: Copy>(input: &mut T, new: &mut T) {
    *input = *new;
}

fn main() {
    let mut forever: &'static str = "hello";
    {
      let stringy = String::from("world");
      overwrite(&mut forever, &mut &*stringy);
    }
    // Oops, printing free'd memory
    println!("{}", forever);
}
```

The signature of `overwrite` is clearly valid: it takes mutable references to two values of the same type, and overwrites one with the other.

If `&mut T` was variant over `T`, then `&mut &'static str` would be a subtype of `&mut &'a str`, since `&'static str` is a subtype of `&'a str`.

Therefore the lifetime of `forever` would successfully be "shrunk" down to the shorter lifetime of string, and overwrite would be called successfully. 

`stringy` would subsequently be dropped, and `forever` would point to freed memory when we print it!

Therefore `&mut` should be invariant.

This is the general theme of variance vs invariance: if variance would allow you to store a short-lived value into a longer-lived slot, then you must be invariant.

However it is sound for `&'a mut T` to be variant over `'a.` 

The key difference between `'a` and `T` is that `'a` is a property of the reference itself, while `T` is something the reference is borrowing.

If you change `T`'s type, then the source still remembers the original type. However if you change the lifetime's type, no one but the reference knows this information, so it's fine.

Put another way: `&'a mut T` owns `'a`, but only borrows `T`.
