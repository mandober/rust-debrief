// ! str
/**
- The `str` type, also called a string slice, is the most primitive string type
- usually seen in its borrowed form: `&str`
- It is also the type of string literals: `&'static str`
- Strings slices are always valid UTF-8.
*/

// string literals = string slices
let hello = "Hello, world!";
// with explicit type and lifetime annotation
let hello: &'static str = "Hello, world!";

/**
They are `'static` because they are stored directly in the final binary,
and so will be valid for the `'static` duration i.e. the entire runtime of the program.
'static is the longest lifetime. All string slices have `'stratic` lifetime.

&str is made up of 2 components:
  1. pointer to some bytes
  2. length

You can look at these with the as_ptr and len methods:
*/
use std::slice;
use std::str;

let story = "Once upon a time...";

let ptr = story.as_ptr();
let len = story.len();

// story has nineteen bytes
assert_eq!(19, len);

// We can re-build a str out of ptr and len. This is all unsafe because
// we are responsible for making sure the two components are valid:
let s = unsafe {
    // First, we build a &[u8]...
    let slice = slice::from_raw_parts(ptr, len);
    // ... and then convert that slice into a string slice
    str::from_utf8(slice)
};
assert_eq!(s, Ok(story));
