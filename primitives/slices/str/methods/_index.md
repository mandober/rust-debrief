# String slice methods
https://doc.rust-lang.org/std/primitive.str.html

```rust
len
into_string
is_empty

chars
char_indices
is_char_boundary

as_bytes
as_ptr
bytes
lines

contains
starts_with
ends_with

get
get_mut
get_unchecked
get_unchecked_mut    // experimental

slice_unchecked      // experimental
slice_mut_unchecked  // experimental
encode_utf16

find
rfind
split
split_at
split_at_mut
split_whitespace
rsplit
split_terminator
rsplit_terminator
splitn
rsplitn

matches
rmatches
match_indices
rmatch_indices

trim
trim_left
trim_right
trim_matches
trim_left_matches
trim_right_matches

parse
into_boxed_bytes
replace
replacen
repeat

to_lowercase
to_uppercase

escape_debug       // experimental
escape_default     // experimental
escape_unicode     // experimental
```


```rust
fn len(&self) -> usize
// Returns the length of self. This length is in bytes, not chars or graphemes.
// In other words, it may not be what a human considers the length of the string.
let len = "foo".len();
assert_eq!(3, len);
let len = "ƒoo".len(); // fancy f!
assert_eq!(4, len);


fn is_empty(&self) -> bool
// Returns `true` if `self` has a length of zero bytes.
let s = "";
assert!(s.is_empty());
let s = "not empty";
assert!(!s.is_empty());


fn is_char_boundary(&self, index: usize) -> bool
// Checks that index-th byte lies at the start and/or end of a UTF-8 code point 
// sequence. The start and end of the string (when index == self.len()) are 
// considered to be boundaries. Returns false if index is greater than self.len().
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0)); // start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));
assert!(!s.is_char_boundary(2)); // second byte of `ö`
assert!(!s.is_char_boundary(8)); // third byte of `老`


fn as_bytes(&self) -> &[u8]
// Converts a string slice to a byte slice. To convert the byte
// slice back into a string slice, use the str::from_utf8 function.
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);


unsafe fn as_bytes_mut(&mut self) -> &mut
// Converts a mutable string slice to a mutable byte slice. To convert the mutable
// byte slice back into a mutable string slice, use the str::from_utf8_mut function.


fn as_ptr(&self) -> *const u8
// Converts a string slice to a raw pointer.
// As string slices are a slice of bytes, the raw pointer points to a u8. T
// his pointer will be pointing to the first byte of the string slice.


fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output>
  where I: SliceIndex<str>,
// Returns a subslice of str.
// This is the non-panicking alternative to indexing the str.
// Returns None whenever equivalent indexing operation would panic.
let mut v = String::from("🗻∈🌏");
assert_eq!(Some("🗻"), v.get(0..4));
// indices not on UTF-8 sequence boundaries
assert!(v.get_mut(1..).is_none());
assert!(v.get_mut(..8).is_none());
// out of bounds
assert!(v.get_mut(..42).is_none());


fn get_mut<I>(&mut self, i: I) -> Option<&mut <I as SliceIndex<str>>::Output>
  where I: SliceIndex<str>,
// Returns a mutable subslice of str.
// This is the non-panicking alternative to indexing the str.
// Returns None whenever equivalent indexing operation would panic.


unsafe fn get_unchecked<I>(&self, i: I) -> &<I as SliceIndex<str>>::Output
  where I: SliceIndex<str>,
// Returns a unchecked subslice of str.
// This is the unchecked alternative to indexing the str.


fn split_at(&self, mid: usize) -> (&str, &str)
// Divide one string slice into two at an index.
// The argument, mid, should be a byte offset from the start of the string. 
// It must also be on the boundary of a UTF-8 code point. The two slices returned 
// go from the start of the string slice to mid, and from mid to the end of the 
// string slice. To get mutable string slices instead, see the split_at_mut method.


fn chars(&self) -> Chars
// Returns an iterator over the chars of a string slice.
// As a string slice consists of valid UTF-8, we can iterate through a string 
// slice by char. This method returns such an iterator. It's important to remember 
// that char represents a Unicode Scalar Value, and may not match your idea of what 
// a 'character' is. Iteration over grapheme clusters may be what you actually want.


fn char_indices(&self) -> CharIndices
// Returns an iterator over the chars of a string slice, and their positions.
// As a string slice consists of valid UTF-8, we can iterate through a string 
// slice by char. This method returns an iterator of both these chars, as well as 
// their byte positions.
// The iterator yields tuples. The position is first, the char is second.


fn bytes(&self) -> Bytes
// An iterator over the bytes of a string slice.
// As a string slice consists of a sequence of bytes, we can iterate through a 
// string slice by byte. This method returns such an iterator.


fn split_whitespace(&self) -> SplitWhitespace
// Split a string slice by whitespace.
// The iterator returned will return string slices that are sub-slices of the 
// original string slice, separated by any amount of whitespace. Whitespace is 
// defined according to the terms of the Unicode Derived Core Property White_Space.


fn lines(&self) -> Lines
// An iterator over the lines of a string, as string slices.
// Lines are ended with either a newline (\n) or a carriage return with a 
// line feed (\r\n). The final line ending is optional.
let text = "foo\r\nbar\n\nbaz\n";
let mut lines = text.lines();
assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());
assert_eq!(None, lines.next());


fn contains<'a, P>(&'a self, pat: P) -> bool
  where P: Pattern<'a>
// Returns true if the given pattern matches a sub-slice of this string slice.
// Returns false if it does not.
let bananas = "bananas";
assert!(bananas.contains("nana"));


fn starts_with<'a, P>(&'a self, pat: P) -> bool
  where P: Pattern<'a>
// Returns true if the given pattern matches a prefix of this string slice.
// Returns false if it does not.
let bananas = "bananas";
assert!(bananas.starts_with("bana"));


fn ends_with<'a, P>(&'a self, pat: P) -> bool
  where P: Pattern<'a>,
        <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>
// Returns true if the given pattern matches a suffix of this string slice.
// Returns false if it does not.
let bananas = "bananas";
assert!(bananas.ends_with("anas"));


fn find<'a, P>(&'a self, pat: P) -> Option<usize>
  where P: Pattern<'a>
// Returns the byte index of the first character of this string slice that 
// matches the pattern.
// Returns None if the pattern doesn't match.
// The pattern can be: &str, char, closure (that determines if a character 
// matches).
let s = "Löwe 老虎 Léopard";
assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("Léopard"), Some(13));
assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
let x: &[_] = &['1', '2'];
assert_eq!(s.find(x), None);
```