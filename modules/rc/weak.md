# Weak

- Struct `std::rc::Weak`, 1.4.0
- Non-owning `Weak` pointer to value created by `downgrade()` on `Rc<T>`
- managed value is accessed with `upgrade()` on `Weak`, which returns `Option<Rc<T>>`; `None` is returned if the value was already dropped.
- `Weak<T>` doesn't auto-deref to `T` (as value may have been destroyed).


## Weak
`Weak` is a version of `Rc` that holds a non-owning reference to the managed value. The value is accessed by calling upgrade on the `Weak` pointer, which returns an `Option<Rc<T>>`.

Since a `Weak` reference does not count towards ownership, it will not prevent the inner value from being dropped, and `Weak` itself makes no guarantees about the value still being present and may return None when upgraded.

A `Weak` pointer is useful for keeping a temporary reference to the value within `Rc` without extending its lifetime. It is also used to prevent circular references between `Rc` pointers, since mutual owning references would never allow either `Rc` to be dropped. For example, a tree could have strong `Rc` pointers from parent nodes to children, and `Weak` pointers from children back to their parents.

The typical way to obtain a `Weak` pointer is to call `Rc::downgrade`.

