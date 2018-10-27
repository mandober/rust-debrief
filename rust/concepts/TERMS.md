# terms

coercion
deref coercion
casting
dynamic dispatch
trait object
trait bounds
trait orphan rule
turbofish
reference
dereference
pointer
raw pointer
function pointer
type erasure
lifetime
generics
generic type
generic lifetime
associated type
associated function
discriminant
Interior mutability
vtable
Pointer
Dynamic Dispatch
Trait Orphan Rule
Trait Bounds
Turbofish
Deref Coercion

<!-- TOC -->

- [Trait object](#trait-object)
- [vtable](#vtable)
- [Dynamic dispatch](#dynamic-dispatch)
- [Trait orphan rule](#trait-orphan-rule)
- [Trait bound](#trait-bound)
- [Turbofish](#turbofish)
- [Deref Coercion](#deref-coercion)

<!-- /TOC -->

## Trait object
Dynamically sized type that implements some trait. The original type is *erased* in favor of runtime reflection, with a *vtable* containing all the information necessary to use the type. The information that "completes" a trait object (trait object is a fat pointer) is a *pointer to its vtable*.

Trait objects, like `&TraitName` or `Box<TraitName>`, are normal values that store a value of any type that implements the given trait, where the precise type can only be known at runtime. A trait object can be obtained from a ref to a concrete type that impl the trait by casting it (`&x as &TraitName`) or by coercing it (using `&x` as an arg to a fn that takes `&TraitName`). These trait object coercions and casts also work for pointers like `&mut T` to `&mut TraitName` and `Box<T>` to `Box<TraitName>`, but that’s all at the moment. Coercions and casts are identical.

## vtable
vtable, virtual method table, VMT
The methods of the trait can be called on a trait object via a special record 
of fn pointers traditionally called a `vtable`, which is created and managed 
by the compiler.

## Dynamic dispatch
Rust supports dynamic dispatch through a mechanism called *trait objects*.

## Trait orphan rule
Orphan rule is from type theory; briefly, it’s called *the orphan rule* because 
the parent type is not present. Without this rule, two crates could implement the same trait for the same type, and the two implementations would conflict: Rust wouldn’t know which implementation to use. Because Rust enforces the orphan rule, other people’s code can’t break your code and vice versa.

## Trait bound
we can use traits with generic type parameters: we can constrain generic types 
so that rather than being any type, the compiler will ensure that the type will 
be limited to those types that implement a particular trait and thus have the 
behavior that we need the types to have. This is called specifying trait bounds 
on a generic type.

## Turbofish
Turbofish `::<T>`
type arguments used in a call expression: `let v = Vec`::<i32>`::new()`

## Deref Coercion
A deref coercion happens when the ref type of the argument passed into the fn 
differs from the ref type of the parameter defined in that fn signature.
Examples:
- coercing `String` to `&str`
- ref to a pointer into ref to that pointer's contents
