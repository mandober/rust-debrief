# enum: basics

- variants
- discriminant
- kinds
- sum type


Fields of structs, enums and tuples are directly put into the memory layout of the struct (without boxing them like in Java!).
Data inside structs, enums and tuples is stored directly inline inside the memory of the struct value. 


## Variants

An enum is a simultaneous definition of a nominal enumerated type as well as a set of constructors, that can be used to create or pattern-match values of the corresponding enumerated type.

An enum is declared using a keyword `enum` followed by a name; enum is a nominal type, it must have a unique name (an identifier, by convention in PascalCase). Then come a set of constructors, called variants. **Variants** enumerate all the possible values an enum can hold.

Each variant has an associated tag (integer), called **discriminant**, which identifies it. Discriminants are assigned to variants by compiler.

An enum stores a tag to indicate which of its variants the enum (as a whole) represents at the given moment (instance). Tags are associated with sequence of integers, starting at 0 with a step of 1.

An enum stores a tag to indicate which of its variants the enum represents at the given instance. Tags are associated with sequence of integers, starting at 0 with a step of 1.

```rust
enum Tag {
    A,
    B(bool),
    C(char)
}
```
Above, `A` variant has a tag 0, `B` is 1 and `C` is 2.


- Since any variant of an enum can become any other variant of that enum, the memory requirement of its variants is equal to the memory requirement of its largest variant.
- Every element needs as much space as the largest element because it has to be ready to become that element anytime.
- Enum is a sum type because it is the sum of the two sets:

```rust
enum Sum {
    a(bool),
    b(u8)
}
```

This is a set of all values which are booleans or integers. This is a sum of sets, `Sum = bool + u8` i.e. Sum=2+256=258. More accurately, it is a disjoint (discriminated) union: if the sets have overlap, the overlap is discriminated out.




An example of an `enum` item and its use:

```rust
enum Animal {
    Dog,
    Cat
}

let mut a: Animal = Animal::Dog;
a = Animal::Cat;
```

Enumeration constructors can have either named or unnamed fields:

```rust
enum Animal {
    Dog (String, f64),
    Cat { name: String, weight: f64 }
}

let mut a: Animal = Animal::Dog("Cocoa".to_string(), 37.2);
a = Animal::Cat { name: "Spotty".to_string(), weight: 2.7 };
```

In this example, Cat is a struct-like enum variant, whereas Dog is simply called an enum variant. Each enum instance has a discriminant which is an integer associated to it that is used to determine which variant it holds.


## null pointer optimization
every enum has to store a tag to specify which variant of the enum its bits represent. However, if we have a special kind of enum:

```rust
enum Foo {
  A,
  B(NonNullPtr),
}
```

the null pointer optimization kicks in, which eliminates the space needed for the tag. If the variant is A, the whole enum is set to all 0's. Otherwise, the variant is B. This works because B can never be all 0's, since it contains a non-zero pointer.




## C-like Enumerations

If there is no data attached to any of the variants of an enumeration it is called a c-like enumeration. If a discriminant isn't specified, they start at zero, and add one for each variant, in order. Each enum value is just its discriminant which you can specify explicitly:

```rust
enum Foo {
    Bar,            // 0
    Baz = 123,
    Quux,           // 124
}
```

The right hand side of the specification is interpreted as an isize value, but the compiler is allowed to use a smaller type in the actual memory layout. The repr attribute can be added in order to change the type of the right hand side and specify the memory layout.

You can also cast a c-like enum to get its discriminant:

```rust
let x = Foo::Baz as u32; // x is now 123u32
```

This only works as long as none of the variants have data attached. If it were Baz(i32), this is disallowed.


## sum type
Enum is, what is called in type theory, an algebraic data type, more precisely, an algebraic sum type, meaning that the number of different values it can have is the sum of the different values each of its variants can hold. Sum types are "OR" types because only one of their variants can be active (assigned to) at a time. More accurately, Enum is a disjoint (discriminated) union: if the sets have overlap, the overlap is discriminated out.
