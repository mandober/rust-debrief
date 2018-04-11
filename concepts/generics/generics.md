# Generics

- Generic type parameters are abstract stand-ins for concrete types.
- Generics encompass:
  - generic type parameters
  - bounds
  - lifetimes
- When we call the generic function, the code actually runs on the concrete 
  values that we pass in; generic types are locked into concrete types.
- Generics have both input and output types: type parameters stand in for **input** types and associated types stand in for **output** types. Input types are specified with generic type parameters and output types are specified using associated types.



## Generics
Generic coding is a tool used to minimize the amount of code and its duplication by generalizing types and functionalities to broader cases.

We can express properties of generics, such as their behavior or how they relate to other generics, without needing to know what will actually be in their place.

Being generic requires taking great care to specify over which types a generic type is actually considered valid. Most common use of generics is for Generic Type Parameters (GTP).

"Generic" also refers to anything that accepts generic type parameters (generic function, generic trait, etc.).

Items that can be generic:
- types (generic type parameters)
- functions
- impl blocks
- traits
- enums
- structs
- type aliases



## Generic Type Parameter
A type parameter is specified as generic by declaring it before using it. It is declared inside the angle brackets: `<A, B, ...>`.

More commonly used name for GTP is `T`, even though the name can be any identifier, using "PascalCase" letter case style (like all the others non-primitive type names).

If a function accepts any type, it declares a GTP:

```rust
fn foo<T>(param: T) { println!("nevermind"); }
```

The problem is, there is nothing to do with value of such (any) type inside the finction, so in order to group types by common behaviours or capabilities we must constrain them by using bounds.

If a name is first declared in angle-brackets, that name is considered as generic type when used, even if there is a concrete type by the same name:

```rust
// concrete type T
struct T;

// GTP is declared as T in angle brackets
// so T is a generic type, no matter if the concrete
// type with the same name exists (struct T).
fn foo<T>(param: T) { /* ... */}

// but not here because there's no declaration:
fn foo(param: T) { /* ... */}
// this would error: "cannot find type `T` in this scope"
// had we not already defined T as the struct
```







## Bounds
If the value of the input type is to be printed, then a function must specify type constraints by declaring that it accepts any type as long as its value is printable (i.e. that the type implements `Display` trait). This is called a type bound:

```rust
fn foo<T: Display>(param: T) { println!("{}"); }
```



## Structs

```rust
// GTP must be declared first in angle brackets,
// AFTER the struct's name
struct Generic<T> { // declare GTP
  field: T          // use GTP
}

// impl for Generic struct with GTP set to the concrete type:
impl Generic<u8> {}

// impl for Generic struct that stays generic:
// GTP's name is the same one used in struct (T)
impl<T> Generic<T> {
  fn get(&self) -> &T {
    &self.field
  }
}
// but it doesn't have to be:
impl<S> Generic<S> {
  fn get(&self) -> &S {
    &self.field
  }
}
// although it better be to avoid confusion
```


## Enums

```rust
// GTP must be declared first in angle brackets,
// AFTER the enum's name
enum Maybe<T> {
  Some: T,
  Nothing,
}
```

## Functions

```rust
// GTP must be declared first in angle brackets,
// AFTER the function's name
fn foo<T: Display>(param: T) { println!("{}"); }
```

## Implementations

```rust
struct Generic<T>(T);

// GTP must be declared first in angle brackets,
// BEFORE the name of the type
impl<T> Generic<T> {}
```

## Traits

```rust
// GTP must be declared first in angle brackets,
// AFTER the trait's name
trait Documentable<T> {}
```

