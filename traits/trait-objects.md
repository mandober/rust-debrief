# Trait object

Trait object is a trait behind some sort of pointer, like reference or Box<T>.

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


