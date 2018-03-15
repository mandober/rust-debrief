# Generics

- Generics are abstract stand-ins for concrete types or other properties.
- We can express properties of generics, such as their behavior or how they 
  relate to other generics, without needing to know what will actually be in 
  their place.
- Generics encompass:
  - generic type parameters
  - trait bounds
  - lifetimes



## Generic type parameters
- Generic functions operate on abstract types instead of concrete types. 
- When we call the generic function, the code actually runs on the concrete 
  values that we pass in; generic types are locked into concrete types.
- Generics have both input and output types: type parameters stand in for **input** types and associated types stand in for **output** types. Input types are specified with generic type parameters and output types are specified using associated types.
