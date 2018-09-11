# Module `cell`

- Module `std::cell`, 1.0.0
- Shareable mutable single-threaded containers, `Cell<T>` and `RefCell<T>`
- mutated through shared references; interior mutability.
- `Cell<T>` has interior mutability by moving values in and out of self.
- to move refs instead, use `RefCell<T>` acquiring write lock before mutating. 



<!-- TOC -->

- [Contents of the `cell` module](#contents-of-the-cell-module)
- [Shareable mutable containers](#shareable-mutable-containers)
- [Cell types](#cell-types)
- [Interior mutability](#interior-mutability)
  - [Interior mutability of something immutable](#interior-mutability-of-something-immutable)
- [Interior mutability and API](#interior-mutability-and-api)
  - [Mutating implementations of Clone](#mutating-implementations-of-clone)

<!-- /TOC -->


## Contents of the `cell` module
Structs:
- `Cell` mutable memory location.
- `RefCell` mutable memory location with dynamically checked borrow rules.
- `UnsafeCell` core primitive for interior mutability.
- `Ref` wrapper type for an immutably borrowed value from `RefCell<T>`.
- `RefMut` wrapper type for a mutably borrowed value from `RefCell<T>`.
- `BorrowError` error returned by the `RefCell::try_borrow`.
- `BorrowMutError` error returned by `RefCell::try_borrow_mut`.


## Shareable mutable containers
Rust memory safety is based on this rule enforced by the compiler: Given an object `T`, it is only possible to have one of the following:
- Having several immutable references (`&T`) to the object (aliasing).
- Having one mutable reference (`&mut T`) to the object (mutability).
However, sometimes it's neeed to have multi refs to object and mutate it.

Shareable mutable containers permit mutability in a controlled manner, even in the presence of aliasing. Both `Cell<T>` and `RefCell<T>` allows to do this in a single threaded way. However, neither `Cell<T>` nor `RefCell<T>` are thread safe (they do not implement `Sync`). If you need to do aliasing and mutation between multiple threads it is possible to use `Mutex`, `RwLock` or atomic types.

Values of the `Cell<T>` and `RefCell<T>` types may be mutated through shared references (i.e. the common `&T` type), whereas most Rust types can only be mutated through unique (`&mut T`) references. We say that `Cell<T>` and `RefCell<T>` provide **interior mutability**, in contrast with typical Rust types that exhibit __inherited mutability__.


## Cell types
Cell types come in two flavors: `Cell<T>` and `RefCell<T>`.

`Cell<T>` implements interior mutability by moving values in and out of the `Cell<T>`. To use references instead of values, one must use the `RefCell<T>` type, acquiring a write lock before mutating.

`Cell<T>` provides methods to retrieve and change the current interior value:
- For types that impl `Copy`, `get` retrieves the current interior value.
- For types that impl `Default`, `take` method replaces the current interior value with `Default::default()` and returns the replaced value.
- For all types, methods
  - `replace` current interior value and returns the replaced value
  - `into_inner` consumes the `Cell<T>` and returns the interior value
  - `set` replaces the interior value, dropping the replaced value.

`RefCell<T>` uses Rust's lifetimes to implement __dynamic borrowing__, a process whereby one can claim temporary, exclusive, mutable access to the inner value. Borrows for `RefCell<T>`s are tracked at runtime, unlike Rust's native reference types which are entirely tracked statically, at compile-time. Because `RefCell<T>` borrows are dynamic it is possible to attempt to borrow a value that is already mutably borrowed; when this happens it results in thread panic.


## Interior mutability
The more common inherited mutability, where one must have unique access to mutate a value, is one of the key language elements that enables Rust to reason strongly about pointer aliasing, statically preventing crash bugs. Because of that, inherited mutability is preferred, and interior mutability is something of a last resort. Since cell types enable mutation where it would otherwise be disallowed though, there are occasions when interior mutability might be appropriate, or even must be used, e.g.:
- Introducing mutability "inside" of something immutable
- Implementation details of logically-immutable methods.
- Mutating implementations of `Clone`.


### Interior mutability of something immutable
Many shared smart pointer types, including `Rc<T>` and `Arc<T>`, provide containers that can be cloned and shared between multiple parties. Because the contained values may be multiply-aliased, they can only be borrowed with `&`, not `&mut`. Without cells it would be impossible to mutate data inside of these smart pointers at all.

It's very common then to put a `RefCell<T>` inside shared pointer types to reintroduce mutability:

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
  shared_map.borrow_mut().insert("africa", 92388);
  shared_map.borrow_mut().insert("kyoto", 11837);
  shared_map.borrow_mut().insert("piccadilly", 11826);
  shared_map.borrow_mut().insert("marbles", 38);
}
```

Note that this example uses `Rc<T>` and not `Arc<T>`. `RefCell<T>` is for single-threaded scenarios. Consider using `RwLock<T>` or `Mutex<T>` if you need shared mutability in a multi-threaded situation.


## Interior mutability and API
Implementation details of logically-immutable methods:
Occasionally it may be desirable not to expose in an API that there is mutation happening "under the hood". This may be because logically the operation is immutable, but e.g. caching forces the implementation to perform mutation; or because you must employ mutation to implement a trait method that was originally defined to take `&self`.

```rust
use std::cell::RefCell;

struct Graph {
    edges: Vec<(i32, i32)>,
    span_tree_cache: RefCell<Option<Vec<(i32, i32)>>>
}

impl Graph {
    fn minimum_spanning_tree(&self) -> Vec<(i32, i32)> {
        // Create a new scope to contain the lifetime of the
        // dynamic borrow
        {
            // Take a reference to the inside of cache cell
            let mut cache = self.span_tree_cache.borrow_mut();
            if cache.is_some() {
                return cache.as_ref().unwrap().clone();
            }

            let span_tree = self.calc_span_tree();
            *cache = Some(span_tree);
        }

        // Recursive call to return the just-cached value.
        // Note that if we had not let the previous borrow
        // of the cache fall out of scope then the subsequent
        // recursive borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        self.minimum_spanning_tree()
    }
}
```

### Mutating implementations of Clone
This is simply a special, but common, case of the previous: hiding mutability for operations that appear to be immutable. The `clone` method is expected to not change the source value, and is declared to take `&self`, not `&mut self`. Therefore any mutation that happens in the `clone` method must use cell types. For example, `Rc<T>` maintains its reference counts within a `Cell<T>`.

```rust
#![feature(core_intrinsics)]
#![feature(shared)]
use std::cell::Cell;
use std::ptr::Shared;
use std::intrinsics::abort;

struct Rc<T: ?Sized> {
    ptr: Shared<RcBox<T>>
}

struct RcBox<T: ?Sized> {
    strong: Cell<usize>,
    refcount: Cell<usize>,
    value: T,
}

impl<T: ?Sized> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        self.inc_strong();
        Rc { ptr: self.ptr }
    }
}

trait RcBoxPtr<T: ?Sized> {
    fn inner(&self) -> &RcBox<T>;

    fn strong(&self) -> usize {
        self.inner().strong.get()
    }

    fn inc_strong(&self) {
        self.inner()
            .strong
            .set(self.strong()
                     .checked_add(1)
                     .unwrap_or_else(|| unsafe { abort() }));
    }
}

impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
   fn inner(&self) -> &RcBox<T> {
       unsafe { self.ptr.as_ref() }
   }
}
```
