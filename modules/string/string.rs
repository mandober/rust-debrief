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

*/

// You can create a String from a literal string with String::from:
let hello = String::from("Hello, world!");

// You can append a char to a String with the push method,
// and append a &str with the push_str method:
let mut hello = String::from("Hello, ");
hello.push('w');
hello.push_str("orld!");


// Two types of Rust strings: String and &str.
// String is a heap-allocated, growable vector of utf8 characters.
// &str is a type¹ that's used to slice into Strings.
// String literals like "foo" are of type &str.
let s: &str = "galaxy";
let s2: String = "galaxy".to_string();
let s3: String = String::from("galaxy");
let s4: &str = &s3;
// ¹str is an unsized type, which doesn't have a compile-time known size,
// and therefore cannot exist by itself.


// *String
// The String Type is a heap-allocated, growable vector of utf8 characters.

// string literals are immutable, hardcoded into the program, stack allocated
// to create a String from a string literal:
let s = String::from("hello");
// This kind of String can be mutated:
let mut s = String::from("hello");
// push_str() appends a literal string to a String
s.push_str(", world!");
println!("{}", s); // This will print `hello, world!`


//* OWNERSHIP
// s1 is the owner
let s1 = String::from("hello");
// s2 takes ownership and s1 BECOMES INVALID
let s2 = s1;
// s1 was MOVED into s2

// here s1 is CLONED (pointer on stack along with heap data is cloned)
let s1 = String::from("hello");
let s2 = s1.clone();

// value types (like int) are always cloned
let x = 5;
let y = x; // copied and x STAYS VALID

// Generally, if a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait. Copy (value) types: int, float, bool, tuples (but only if they hold Copy types)

// ====================================================
//! STRINGS

/* implemented as a collection of bytes plus some methods to provide useful functionality when those bytes are interpreted as text

Rust actually only has one string type in the core language itself:
str, the string slice, which is usually in its borrowed form, &str.

string slices are a reference to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the binary output of the program, and are therefore string slices.

The type called `String` is provided in Rust’s standard library rather than coded into the core language, and is a growable, mutable, owned, UTF-8 encoded string type.

strings in Rust usually mean both the `String` and
the string slice &str types, not just one of these.

Rust’s stdlib also includes a number of other string types, such as:
OsString / OsStr
CString / CStr
Similar to the *String / *Str naming, they often provide
an owned and borrowed variant, just like String / &str
*/

//! CREATE
// Many of the operations available with Vec are available with String, starting with the new function to create a string:
//? String::new()
let mut s = String::new();
// This creates a new empty string that we can then load data into.

//? to_string()
// method is available on any type that implements the `Display` trait, which string literals do:
let data = "initial contents";
let s = data.to_string();
// the method also works on a literal directly:
let s = "initial contents".to_string();
// This creates a string containing "initial contents".

//? String::from()
// creates a String from a string literal. This is equivalent to using to_string:
let s = String::from("initial contents");

// strings are UTF-8 encoded
let s1 = "Здравствуйте";
let s2 = "Dobrý den";
let s3 = "Olá";
let s4 = "السلام عليكم";
let s8 = "안녕하세요";
let s5 = "שָׁלוֹם";
let s6 = "नमस्ते";
let s7 = "こんにちは";
let s9 = "你好";


//! UPDATE

//? push_str(:&str)
// appends a string slice to a `String`
let mut s = String::from("foo");
s.push_str("bar");
// it takes str.slice so as to just borrow the value
let mut s1 = String::from("foo");
let s2 = String::from("bar");
s1.push_str(&s2);
// but we can still use s2

//? push(:char)
// has a single character as a parameter to be added to String
let mut s = String::from("lo");
s.push('l'); // s => lol

//? Concatenation with + operator
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
// The + operator uses the `add` method, with signature like this:
fn add(self, s: &str) -> String

//* tricky bits of the + operator
/* First of all, s2 has an &, meaning that we are adding a reference of the second string to the first string. This is because of the s parameter in the add function: we can only add a &str to a String, we can’t add two String values together. But the type of &s2 is &String, not &str, as specified in the second parameter to add!? We are able to use &s2 in the call to add because //! &String ARGUMENT CAN BE COERCED INTO &str
When the add function is called, Rust uses //! DEREF COERCION
which is alike to turning &s2 into &s2[..] for use in the add function. Because add does not take ownership of the parameter, s2 will still be a valid String after this operation.

Second, we can see in the signature that add takes ownership of self (because self does not have an &). This means s1 in the above example will be moved into the add call and no longer be valid after that.
So while `let s3 = s1 + &s2` only looks like it will copy both strings and create a new one, this statement actually:
- takes ownership of s1,
- appends a copy of the contents of s2, then
- returns ownership of the result.
It looks like it’s making a lot of copies, but it's not;
the implementation is more efficient than copying. */
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3; // tic-tac-toe

//? format!()
// string combining with format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe
// The format! macro works in the same way as println!, but instead of printing the output to the screen, it returns a String with the contents. This version is much easier to read, and also DOES NOT TAKE OWNERSHIP OF ANY OF ITS PARAMETERS.


//! indexing
// strings don’t support indexing with brackets operator cuz index into the string's bytes will not always correlate to a valid Unicode scalar value.

//? Internal Representation of String
// A String is a wrapper over a Vec<u8>
let len = String::from("Hola").len(); // 4 (4 bytes)
let len = String::from("Здравствуйте").len(); // 24
// because it takes 24 bytes to encode it in utf8


//! Slicing Strings
// create a string slice containing particular bytes:
let hello = "Здравствуйте";
let s = &hello[0..4];
// Here, s will be a &str that contains the first four bytes of the string; each of these characters was two bytes, so that means that s will be “Зд”.
// what if we did `&hello[0..1]` => panic at runtime in the same way that accessing an invalid index in a vector does

//! Iterating Over Strings

//? chars()
// to perform operations on individual Unicode scalar values
for c in "नमस्ते".chars() {
    println!("{}", c);
} // न  म  स  ्  त  े

//? bytes()
// The bytes method returns each raw byte
for b in "नमस्ते".bytes() {
    println!("{}", b);
} // 224 164 168 ...
