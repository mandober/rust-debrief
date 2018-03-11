# Module `rc`

- Module `std::rc`, 1.0.0
- Single-threaded, non-atomic, reference-counting, smart pointer, `Rc<T>`.
- shared ownership of a value of type `T`, allocated on the heap.
- cloning `Rc` produces a new pointer to the value on the heap.
- when the last `Rc` pointer to the value is destroyed, so is the value.
- mut ref to value is disallowed, use `Rc<Cell<T>>`, `Rc<RefCell<T>>`.
- `Rc<T>` auto-derefs to `T`, so `T` methods can be called on `Rc<T>`.
- `Rc` methods are all associated fn (avoids clashes with `T` methods).
- Non-owning `Weak` pointer to value created by `downgrade()` on `Rc<T>`
- the managed value is accessed by calling `upgrade()` method on `Weak`, which returns `Option<Rc<T>>`; `None` is returned if the value was already dropped.
- `Weak<T>` doesn't auto-deref to `T` (as value may have been destroyed).




## Reference-counting smart pointer

Module `rc` contentains 2 structs:
- `Rc` single-threaded reference-counting pointer.
- `Weak` holds a non-owning ref to the managed value. The value is accessed by calling `upgrade` on the `Weak` pointer, which returns `Option<Rc<T>>`.

`Rc<T>` provides shared ownership of a value of type `T`, heap allocated.
- cloning an `Rc` produces a new pointer to the same value on the heap.
- When the last `Rc` pointer to the value is destroyed, so is the value.

Shared references disallow mutation by default, and `Rc` is no exception:
- you cannot obtain a mut ref to something inside an `Rc`.
- If you need mutability, put a `Cell` or `RefCell` inside the `Rc`

`Rc` uses non-atomic reference counting. 
- overhead is very low, but `Rc` cannot be sent between threads, no `Send`
- compile-time tracked lest it be sent between threads.
- for multi-threaded, atomic reference counting, use `sync::Arc`.

The `downgrade` method can be used to create a non-owning `Weak` pointer. `Weak` is upgradable to `Rc`, but this will return `None` if the value has already been dropped. A cycle between `Rc` pointers will never be deallocated. For this reason, `Weak` is used to break cycles. For example, a tree could have strong `Rc` pointers from parent nodes to children, and `Weak` pointers from children back to their parents.

`Rc<T>` automatically dereferences to `T` via `Deref` trait, so methods on `T` can be called on a value of type `Rc<T>`. To avoid name clashes with `T` methods, `Rc<T>` methods are associated fns. `Weak<T>` does not auto-dereference to `T`, because the value may have already been destroyed.

```rust
use std::rc::Rc;
let my_rc = Rc::new(());
Rc::downgrade(&my_rc);
```

Creating a new reference from an existing `Rc` pointer is done using the `Clone` trait implemented for `Rc<T>` and `Weak<T>`.

```rust
use std::rc::Rc;
let foo = Rc::new(vec![1.0, 2.0, 3.0]);
// These two syntaxes are equivalent:
// a and b point to the same memory location as foo.
let a = foo.clone();
let b = Rc::clone(&foo);
// all vars: foo, a, b point to the same memory location
```

The `Rc::clone(&from)` syntax is the most idiomatic because it conveys more explicitly the meaning of the code. In the example above, this syntax makes it easier to see that this code is creating a new reference rather than copying the whole content of foo.

## Examples

Consider a scenario where a set of Gadgets are owned by a given Owner. We want to have our Gadgets point to their Owner. We can't do this with unique ownership, because more than one gadget may belong to the same Owner. Rc allows us to share an Owner between multiple Gadgets, and have the Owner remain allocated as long as any Gadget points at it.

```rust
use std::rc::Rc;

struct Owner {
  name: String,
}

struct Gadget {
  id: i32,
  name: String,
  owner: Rc<Owner>,
}

fn main() {
  // Create a reference-counted Owner.
  let gadget_owner: Rc<Owner> = Rc::new(
    Owner {
      name: "Batman".to_string(),
    }
  );

  // Create Gadgets belonging to `gadget_owner`. Cloning the `Rc<Owner>`
  // value gives us a new pointer to the same `Owner` value, incrementing
  // the reference count in the process.
  let gadget1 = Gadget {
    id: 1,
    name: "future dream".to_string();
    owner: Rc::clone(&gadget_owner),
  };

  let gadget2 = Gadget {
    id: 2,
    name: "shopping scheme".to_string();
    owner: Rc::clone(&gadget_owner),
  };

  // Dispose of our local variable `gadget_owner`.
  drop(gadget_owner);

  // Despite dropping `gadget_owner`, we're still able to print out the name
  // of the `Owner` of the `Gadget`s. This is because we've only dropped a
  // single `Rc<Owner>`, not the `Owner` it points to. As long as there are
  // other `Rc<Owner>` values pointing at the same `Owner`, it will remain
  // allocated. The field projection `gadget1.owner.name` works because
  // `Rc<Owner>` automatically dereferences to `Owner`.
  println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
  println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

  // At the end of the function, `gadget1` and `gadget2` are destroyed, and
  // with them the last counted references to our `Owner`. Batman, like any
  // passer-by, gets destroyed as well.
}
```

If our requirements change, and we also need to be able to traverse from Owner to Gadget, we will run into problems. An Rc pointer from Owner to Gadget introduces a cycle between the values. This means that their reference counts can never reach 0, and the values will remain allocated forever: a memory leak. In order to get around this, we can use Weak pointers.

Rust actually makes it somewhat difficult to produce this loop in the first place. In order to end up with two values that point at each other, one of them needs to be mutable. This is difficult because Rc enforces memory safety by only giving out shared references to the value it wraps, and these don't allow direct mutation. We need to wrap the part of the value we wish to mutate in a RefCell, which provides interior mutability: a method to achieve mutability through a shared reference. RefCell enforces Rust's borrowing rules at runtime.

```rust
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
    // ...other fields
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // ...other fields
}

fn main() {
    // Create a reference-counted `Owner`. Note that we've put the `Owner`'s
    // vector of `Gadget`s inside a `RefCell` so that we can mutate it through
    // a shared reference.
    let gadget_owner: Rc<Owner> = Rc::new(
        Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(vec![]),
        }
    );

    // Create `Gadget`s belonging to `gadget_owner`, as before.
    let gadget1 = Rc::new(
        Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        }
    );
    let gadget2 = Rc::new(
        Gadget {
            id: 2,
            owner: Rc::clone(&gadget_owner),
        }
    );

    // Add the `Gadget`s to their `Owner`.
    {
        let mut gadgets = gadget_owner.gadgets.borrow_mut();
        gadgets.push(Rc::downgrade(&gadget1));
        gadgets.push(Rc::downgrade(&gadget2));

        // `RefCell` dynamic borrow ends here.
    }

    // Iterate over our `Gadget`s, printing their details out.
    for gadget_weak in gadget_owner.gadgets.borrow().iter() {

        // `gadget_weak` is a `Weak<Gadget>`. Since `Weak` pointers can't
        // guarantee the value is still allocated, we need to call
        // `upgrade`, which returns an `Option<Rc<Gadget>>`.
        //
        // In this case we know the value still exists, so we simply
        // `unwrap` the `Option`. In a more complicated program, you might
        // need graceful error handling for a `None` result.

        let gadget = gadget_weak.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    // At the end of the function, `gadget_owner`, `gadget1`, and `gadget2`
    // are destroyed. There are now no strong (`Rc`) pointers to the
    // gadgets, so they are destroyed. This zeroes the reference count on
    // Gadget Man, so he gets destroyed as well.
}
```
