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



## Trait object
Dynamically sized type that implements some trait. The original type is *erased* in favor of runtime reflection, with a *vtable* containing all the information necessary to use the type. The information that "completes" a trait object (trait object is a fat pointer) is a *pointer to its vtable*.

Trait objects, like `&TraitName` or `Box<TraitName>`, are normal values that store a value of any type that implements the given trait, where the precise type can only be known at runtime. A trait object can be obtained from a ref to a concrete type that impl the trait by casting it (`&x as &TraitName`) or by coercing it (using `&x` as an arg to a fn that takes `&TraitName`). These trait object coercions and casts also work for pointers like `&mut T` to `&mut TraitName` and `Box<T>` to `Box<TraitName>`, but that’s all at the moment. Coercions and casts are identical.

## vtable, virtual method table, VMT
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

## Turbofish `::<T>`
type arguments used in a call expression: `let v = Vec`::<i32>`::new()`

## Deref Coercion
A deref coercion happens when the ref type of the argument passed into the fn 
differs from the ref type of the parameter defined in that fn signature.
Examples:
- coercing `String` to `&str`
- ref to a pointer into ref to that pointer's contents


## reference
In computer science, a reference is a value that enables a program to indirectly
access a particular datum, such as a variable or a record, in the computer's memory or in some other storage device. The reference is said to refer to the datum, and accessing the datum is called *dereferencing* the reference.
A reference is distinct from the datum itself. Typically, for references to data stored in memory on a given system, a reference is implemented as the physical address of where the data is stored in memory or in the storage device. For this reason, a reference is often erroneously confused with a *pointer*, and is said to "point to" the data. However a reference may also be implemented in other ways, such as the offset between the data's address and some fixed "base" address, as an index into an array, or more abstractly as a handle. More broadly, in networking, references may be network addresses, such as URLs.

## smart pointer
In computer science, a smart pointer is an abstract data type that simulates a 
pointer while providing added features, such as automatic memory management or 
bounds checking. Such features are intended to reduce bugs caused by the misuse 
of pointers, while retaining efficiency. Smart pointers typically keep track of 
the memory they point to, and may also be used to manage other resources, such 
as network connections and file handles.
Smart pointers prevent most situations of memory leaks, caused by pointer 
mishandling (double free, dangling pointers, etc.), by making the memory 
deallocation automatic. More generally, they make object destruction automatic.

