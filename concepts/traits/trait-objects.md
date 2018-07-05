# Trait object

- a trait object is a trait behind abstraction pointer (ref, box).




A trait object is an *opaque* value of another type that implements a set of traits. The set of traits is made up of an object save base trait plus any number of auto traits.

Trait objects implement the base trait, its auto traits, and any super traits of the base trait.

Trait objects are written as the optional keyword `dyn` followed by a set of trait bounds, with restrictions on the trait bounds:
- all traits, except the first trait, must be auto traits
- there may not be more than one lifetime
- opt-out bounds (e.g. `?Sized`) are not allowed
- paths to traits may be parenthesized


For example, given a trait `Trait`, the following are all trait objects:

```rust
Trait
dyn Trait
dyn Trait + Send
dyn Trait + Send + Sync
dyn Trait + 'static
dyn Trait + Send + 'static
dyn Trait +
dyn 'static + Trait.
dyn (Trait)
```

If the first bound of the trait object is a path that starts with `::`, then the `dyn` will be treated as a part of the path. The first path can be put in parenthesis to get around this. So, if you want a trait object with the trait `::your_module::Trait`, you should write it as `dyn (::your_module::Trait)`.

Note: For clarity, it is recommended to always use the `dyn` keyword on your trait objects unless your codebase supports compiling with Rust 1.26 or lower.

Two trait object types alias each other if the base traits alias each other and if the sets of auto traits are the same and the lifetime bounds are the same. For example, `dyn Trait + Send + UnwindSafe` is the same as `dyn Trait + Unwindsafe + Send`.

WARNING: With two trait object types, even when the complete set of traits is the same, if the base traits differ, the type is different. For example, `dyn Send + Sync` is a different type from `dyn Sync + Send`(see issue 33140).

WARNING: Including the same auto trait multiple times is allowed, and each instance is considered a unique type. As such, `dyn Trait + Send` is a distinct type to `dyn Trait + Send + Send` (See issue 47010).

Due to the opaqueness of which concrete type the value is of, trait objects are dynamically sized types. Like all DSTs, trait objects are used behind some ptr; for example `&dyn SomeTrait` or `Box<dyn SomeTrait>`. 

Each instance of a pointer to a trait object includes:
- a pointer to an instance of a type `T` that implements `SomeTrait`
- a virtual method table, often just called a _vtable_, which contains, for each method of `SomeTrait` that `T` implements, a pointer to `T`'s implementation (i.e. a function pointer).

The purpose of trait objects is to permit _late binding_ of methods. Calling a method on a trait object results in virtual dispatch at runtime: that is, a function pointer is loaded from the trait object vtable and invoked indirectly. The actual implementation for each vtable entry can vary on an object-by-object basis.


An example of a trait object:

```rust

trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<dyn Printable>);
}
```

In this example, the trait `Printable` occurs as a trait object in both the type signature of print, and the cast expression in main.


## Trait Object Lifetime Bounds
Since a trait object can contain references, the lifetimes of those references need to be expressed as part of the trait object. This lifetime is written as `Trait + 'a`. There are defaults that allow this lifetime to usually be inferred with a sensible choice.






---

Trait object is dynamically sized type that implements some trait. The original type is *erased* in favor of runtime reflection, with a *vtable* containing all the information necessary to use the type. The information that "completes" a trait object (trait object is a fat pointer) is a *pointer to its vtable*.

Trait objects, like `&TraitName` or `Box<TraitName>`, are normal values that store a value of any type that implements the given trait, where the precise type can only be known at runtime.

A trait object can be obtained from a ref to a concrete type that impl the trait by *casting* it (`&x as &TraitName`) or by *coercing* it (using `&x` as an arg to a fn that takes `&TraitName`). These trait object coercions and casts also work for mut refs, like `&mut T` to `&mut TraitName` and smart pointers, like `Box<T>` to `Box<TraitName>`. 


## Trait objects
Vector holds homogeneous elements. In order to hold heterogeneous elements, we can set up an enum and then the vector can hold enum's variants instances. This works for cases in which the kinds of things we want to be able to treat interchangeably are a fixed set of types that we know when our code gets compiled.

Trait objects are more like objects in other languages, in the sense that they combine the data made up of the pointer to a concrete object with the behavior of the methods defined in the trait.

However, trait objects are different from objects in other languages because we can’t add data to a trait object. Trait objects aren’t as generally useful as objects in other languages: their purpose is to allow abstraction across common behavior.

A trait defines behavior that we need in a given situation. We can then use a trait as a trait object in places where we would use a concrete type or a generic type. Rust’s type system will ensure that any value we substitute in for the trait object will implement the methods of the trait. Then we don’t need to know all the possible types at compile time, and we can treat all the instances the same way. 


## Trait objects perform dynamic dispatch
When we use trait objects, the compiler can’t perform monomorphization because we don’t know all the types that might be used with the code. Instead, Rust keeps track of the code that might be used when a method is called and figures out at runtime which code needs to be used for a particular method call. This is known as dynamic dispatch, and there’s a runtime cost when this lookup happens. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which prevents some optimizations.


## Object Safety
Not all traits can be made into trait objects; only object safe traits can.

A trait is object safe as long as both of these are true:
- The trait does not require `Self` to be `Sized`
- All of the trait’s methods are object safe

`Self` is an alias for the type that we’re implementing traits or methods on.

`Sized`
Sized is a marker trait that is automatically implemented on types that have a known size at compile time (such as i32 and references; types that do not have a known size include slices and trait objects).
*Generic type parameters have a default bound of `Self: Sized`*
Sized is an implicit trait bound on all generic type parameters by default. 
Most useful operations in Rust require a type to be Sized, so making Sized a default requirement on trait bounds means we don’t have to write T:Sized with most every use of generics. 

`?Sized`
However, if we want to use a trait on slices, we need to opt out of the Sized trait bound, and we can do that by specifying `T: ?Sized` as a trait bound.

`?Sized` traits
Traits opt out of the Sized trait bound, they have `T: ?Sized`as a trait bound.
*Traits have a default bound of `Self: ?Sized`*
which means they can be implemented on types that may or may not be Sized. 


**generic type params**:  
`Self: Sized`  default bound (type MUST be sized)
`Self: ?Sized` opting out of default (now, type may or may not be sized)

**traits**:  
`Self: ?Sized` default bound (type may or may not be sized).
`Self: Sized`  opting out of default (now, type MUST be sized)


