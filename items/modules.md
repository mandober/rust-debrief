# Modules

- module is a container for items; modules can nest arbitrarily.
- _module item_ is a module, surrounded in braces, named, prefixed with`mod`.
- module item introduces a new named module into the tree of modules making up a crate.
- topmost module of every crate is _anonymous module_; all other items within the crate have paths within the module tree of the crate.
- module resolution:
  - `use` resolves paths relative to the top module:   
    `use foo::bar::baz` resolves foo relative to the **top** mod, main.rs    
  - in code, paths are resolved relative to the current module:   
    `foo::bar::baz` resolves relative to the **current** mod   
- Imports
  - `extern crate` and `use` act as if they were defining the imported item in the current module, like a symbolic link.
  - `use foo::bar::baz` resolves the path relative to the root module
  - path `foo::bar::baz` resolves relative to the current module
Path overrides
  - path `::foo::bar::baz` always resolves relative to the root module
  - path `self::foo::bar::baz` always resolves relative to the current module
  - path `super::foo::bar::baz` always resolves relative to the parent module



An example of a module:

```rust
mod math {
    type Complex = (f64, f64);
    pub fn sin() -> f64 { }
}
```

- modules and types share the same namespace. Declaring a named type with the same name as a module in scope is forbidden: i.e. a type definition, trait, struct, enum, union, type parameter or crate can't shadow the name of a module in scope, or vice versa. Items brought into scope with `use` also have this restriction.
- module without a body is loaded from an external, eponymous (by default), file with `.rs` extension added. When a nested submodule is loaded from an external file, it is loaded from a subdirectory path that mirrors the module hierarchy.


```rust
// load module from "vec.rs"
mod vec;

mod thread {
    // load "local_data" module from "thread/local_data.rs"
    // or "thread/local_data/mod.rs".
    mod local_data;
}
```

The directories and files used for loading external file modules can be influenced with the path attribute.

```rust
#[path = "thread_files"]
mod thread {
    // Load the `local_data` module from `thread_files/tls.rs`
    #[path = "tls.rs"]
    mod local_data;
}
```


## Misc examples
from https://manishearth.github.io/blog/2017/05/14/mentally-modelling-modules/

Module resolution:
- in `use` paths are resolved from top module   
  `use foo::bar::baz` resolves foo relative to the top mod, main.rs    
  `use self::foo::bar::baz` resolves foo relative to the current mod
- in code, paths are resolved relative to the current module   
  `foo::bar::baz` within the code resolves foo relative to the current mod   
  `::foo::bar::baz` within the code, foo resolves relative to the top mod


Imports
- `extern crate` and `use` will act as if they were defining the imported item in the current module, like a symbolic link.
- `use foo::bar::baz` resolves the path relative to the root module
- path `foo::bar::baz` resolves relative to the current module
- path `::foo::bar::baz` always resolves relative to the root module
- path `self::foo::bar::baz` always resolves relative to the current module
- path `super::foo::bar::baz` always resolves relative to the parent module


## example 1:

```rust
// this is annonymous top module.
// it imports mem and now, from any child mod's POV,
// there’s an item called mem in the top module.
use std::mem;

// foo is a child of the annonymous top module
pub mod foo {
    // use resolves paths relative to the anonymous top mod,
    // and mem exists there, as it was imported there
    use mem::transmute; // ok

    // use is relative to the top mod, and std is not in the top mod
    // it is in the root (::std)
    use std::mem::transmute; // error

    pub mod bar {
        // child mod bar sees transmute as if it were belonging to its parent
        use foo::transmute; // ok
    }
}
```

## example 2:

```rust
pub mod foo {
    use bar;
    use bar::bar_inner;

    fn foo() {
        bar_inner(); // ok
        bar::bar_inner(); // ok
        baz::baz_inner(); // error

        ::baz::baz_inner(); // ok
        super::baz::baz_inner(); // ok

        ::bar::bar_inner(); // ok
        super::bar::bar_inner(); // ok
        self::bar::bar_inner(); // ok
    }
}

pub mod bar {
    pub fn bar_inner() {}
}

pub mod baz {
    pub fn baz_inner() {}
}
```

## example 3:

```rust
pub mod foo {
    use bar::baz;

    use baz::inner(); // error

    use self::baz::inner; // ok
    use bar::baz::inner // ok

    pub fn foo() {
        baz::inner(); // ok
    }
}

pub mod bar {
    pub mod baz {
        pub fn inner() {}
    }
}
```
