# String slice

- string slice, `str`, a reference to the part of a string.
- primitive, nominal, unsized, pointer type
- online docs: [`str` primitive][strpri], [`str` module][strmod]
- the most primitive string type, a special reference to a `String`
- string slices are always valid UTF-8.
- it has no ownership over the data it points to, it always borrows it, `&str`
- 2 word pointer: pointer to the (part of the) string and the length.
- string slices have fixed size, they are a view into String.
- string literals are also string slices of `&'static str` type.


[strpri]: https://doc.rust-lang.org/std/primitive.str.html
[strmod]: https://doc.rust-lang.org/std/str/



## String slices
String slice is the most primitive string type. It is an unsized type, its size is not known at compile-time and as such it can only exist behind some kind of pointer; that's why it's always seen in its borrowed form as `&str`.

A value of type `str` is a Unicode string, represented as an `u8` array of bytes, `[u8; len]`, holding a sequence of UTF-8 code points. The `str` is not a first-class type, it is a dynamically sized type so it can only be instantiated behind some pointer type, such as a reference, `&str`.

The type `str` is special: being a primitive (implemented by the compiler), the compiler can optimize certain string operations. But it is not the first class type, so it's not possible to define variables of type `str` or pass `str` values directly to functions. To use it, it must be behind some pointer, so it always appears as `&str` i.e. in its borrowed form.

Being a pointer (reference) type, it has a lifetime and its lifetime has to be frequently annotated when the `&str` is used in fn, impl blocks, traits, structs, enums, constants, statics and type aliases.


```rust
// byte string literal:
let bytes: &[u8; 10] = b"some thing";
```



Casting str to a pointer

```rust
let ptr = "Once upon a time".as_ptr();
println!("{:?}", ptr); // 0x7ff614f86958
```


## String literals
String literals, which are also string slices, have `&'static str` type. They have a static lifetime, which means they are guaranteed to be valid for the duration of the entire program. This is not a problem for them as they are stored in the final binary; when the binary is executed, the string slices are stored in the `.data` section of the memory block.


```rust
// string literals = string slices
let hello = "Hello, world!";

// declare string literal
let _sl = "string literal";
// annotated string literal
let _asl: &str = "annotated string literal";
// fully annotated string literal
let _tasl: &'static str = "lifetime annotated string literal";


// with explicit type and lifetime annotation
let hello: &'static str = "Hello, world!";
```

They have the `'static` lifetime, because they are stored directly in the final binary, and so will be valid for the `'static` duration i.e. the entire runtime of the program. 'static is the longest lifetime. All string slices have `'stratic` lifetime.

They are `'static` because they're stored directly in the final binary, so they will be valid for the `'static` duration.


## Representation
A string slice is a two word fat pointer stored on the stack: first word is a pointer to some bytes on the heap and the second word represents the length in bytes. They can be examined with the `as_ptr` and `len` methods:

```rust
use std::slice;
use std::str;

// string literal
let str_slice = "résumé";
println!("str slice: {}", str_slice);

// get its length in bytes:
let len = str_slice.len();
println!("len: {}", len); // 8 (bytes)

// convert str to raw pointer
let ptr = str_slice.as_ptr();
println!("ptr: {:?}", ptr); // 0x7ff7d12fb330

let result = unsafe {
  // first, we build a slice of u8's
  let slice: &[u8] = slice::from_raw_parts(ptr, len);
  // then convert the slice into a string slice, which returns Result
  // with a &str if conversation is successful or Utf8Error error if not.
  let tmp: Result<&str, std::str::Utf8Error> = str::from_utf8(slice);
  tmp
};

match result {
  // Ok holds a `&str`
  Ok(s) => println!("rebuilt str: {}", s),
  // Err holds a `std::str::Utf8Error`
  Err(e) => println!("error: {}", e),
}
```
