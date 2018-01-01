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
- When we call the function, the code actually runs on the concrete 
  values that we pass in; generic types get locked into concrete types.
