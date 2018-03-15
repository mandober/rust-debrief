# String slice

- string slice, `str`, usually seen in its borrowed form `&str`, is the most primitive string type.
- string literals, also string slices, have type `&'static str`.

- online docs: [`str` primitive][strpri] and [`str` module][strmod]
- It is a fat pointer: a pointer itself and a lenght



[strpri]: https://doc.rust-lang.org/std/primitive.str.html
[strmod]: https://doc.rust-lang.org/std/str/



## String slices
String slice, `str`, is the most primitive string type. It is an unsized type and as such it must be behind a pointer - that is why it is always in seen in its borrowed form as `&str`.


## String literals
String literals, which are also string slices, have `&'static str` type. They have a static lifetime, which means they are guaranteed to be valid for the duration of the entire program. This is not a problem for them as they are stored in the final binary; when the binary is executed, the string slices are stored in the `.data` section of the memory block.



It is also the type of *string literals*, `&'static str`.
String literals are string slices.

Strings slices are always valid UTF-8.

Strings slices have fixed size, they are a view into String.

This documentation describes a number of methods 
and trait implementations on the `str` type. 
For technical reasons, there is additional, separate 
documentation in the `std::str` module as well.




Casting str to pointer

```rust
let ptr = "Once upon a time".as_ptr();
println!("{:?}", ptr); // 0x7ff614f86958
```
