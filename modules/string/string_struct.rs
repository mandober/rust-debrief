// ! String
/**
https://doc.rust-lang.org/std/string/struct.String.html

Struct std::string::String

pub struct String {
    // fields omitted
}

A UTF-8 encoded, growable string.

The String type is the most common string type
that has ownership over the contents of the string.

It has a close relationship with its borrowed counterpart, the primitive `str`.

EXAMPLES:
*/

// You can create a String from a literal string with String::from:
let hello = String::from("Hello, world!");

// You can append a char to a String with the push method,
// and append a &str with the push_str method:
let mut hello = String::from("Hello, ");
hello.push('w');
hello.push_str("orld!");


// If you have a vector of UTF-8 bytes, you can
// create a String from it with the from_utf8 method:

// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// We know these bytes are valid, so we'll use `unwrap()`.
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);

/**
UTF-8
=====
Strings are always valid UTF-8.
This has a few implications, the first of which is that if you need a non-UTF-8
string, consider OsString. It is similar, but without the UTF-8 constraint.
The second implication is that you cannot index into a String:
let s = "hello";
println!("The first letter of s is {}", s[0]); // ERROR!!!

Indexing is intended to be a constant-time operation, but UTF-8 encoding does
not allow us to do this. Furthermore, it's not clear what sort of thing the
index should return: a byte, a codepoint, or a grapheme cluster. The `bytes` and
`chars` methods return iterators over the first two, respectively.

Deref
=====
Strings implement Deref<Target=str>, and so inherit all of str's methods.
In addition, this means that you can pass a String to
a function which takes a &str by using an ampersand (&):
*/
fn takes_str(s: &str) { }
let s = String::from("Hello");
takes_str(&s);

/**
This will create a &str from the String and pass it in.
This conversion is very inexpensive, and so generally, functions will accept
&strs as arguments unless they need a String for some specific reason.

In certain cases Rust doesn't have enough information to make this conversion,
known as Deref coercion. In the following example a string slice `&'a str`
implements the trait TraitExample, and the function example_func takes anything
that implements the trait. In this case Rust would need to make two implicit
conversions, which Rust doesn't have the means to do.
For that reason, the following example will not compile.
*/
trait TraitExample {}

impl<'a> TraitExample for &'a str {}

fn example_func<A: TraitExample>(example_arg: A) {}

fn main() {
    let example_string = String::from("example_string");

    example_func(&example_string);
    // needs to be chaged to:
    example_func(example_string.as_str());
    // or:
    example_func(&*example_string);
}
/**
There are two options that would work instead.

The first would be to change the line `example_func(&example_string)`
to `example_func(example_string.as_str())`, using the method `as_str()`
to explicitly extract the string slice containing the string.

The second way changes `example_func(&example_string)`
to `example_func(&*example_string)`.
In this case we are dereferencing a String to a str,
then referencing the str back to &str.

The second way is more idiomatic, however both work to do the conversion
explicitly rather than relying on the implicit conversion.


Representation
==============

A String is made up of 3 components:
1) pointer to some bytes
   Pointer points to an internal buffer String uses to store its data.
2) length
   Length is the number of bytes currently stored in the buffer.
3) capacity
   Capacity is the size of the buffer in bytes.

As such, the length will always be less than or equal to the capacity.
This buffer is always stored on the heap.

You can look at these with the as_ptr, len, and capacity methods:
*/
use std::mem;
let story = String::from("Once upon a time...");
let ptr = story.as_ptr();
let len = story.len();
let capacity = story.capacity();

// story has nineteen bytes
assert_eq!(19, len);

// Now that we have our parts, we throw the story away.
mem::forget(story);

// We can re-build a String out of ptr, len, and capacity. This is all
// unsafe because we are responsible for making sure the components are valid:
let s = unsafe { String::from_raw_parts(ptr as *mut _, len, capacity) } ;

assert_eq!(String::from("Once upon a time..."), s);


/**
If a String has enough capacity, adding elements to it
will not re-allocate. For example, consider this program:
*/
let mut s = String::new();
println!("{}", s.capacity());

for _ in 0..5 {
    s.push_str("hello");
    println!("{}", s.capacity());
}
/**
This will output the following:
0
5
10
20
20
40

At first, we have no memory allocated at all, but as we append to the string,
it increases its capacity appropriately. If we instead use the with_capacity
method to allocate the correct capacity initially:
*/
let mut s = String::with_capacity(25);

println!("{}", s.capacity());

for _ in 0..5 {
    s.push_str("hello");
    println!("{}", s.capacity());
}
/**
We end up with a different output:
25
25
25
25
25
25

Here, there's no need to allocate more memory inside the loop.
*/