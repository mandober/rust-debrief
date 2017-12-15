// ! `to_*`

fn to_lowercase(self) -> ToLowercase; // Returns an iterator
fn to_uppercase(self) -> ToUppercase; // Returns an iterator
fn to_digit(self, radix: u32) -> Option<u32>;



// ! to_lowercase
fn to_lowercase(self) -> ToLowercase;
/**
|> Returns an iterator
   that yields the lowercase equivalent
   of a char as one or more chars.

If a character does not have a lowercase equivalent,
the same character will be returned back by the iterator.

This performs complex unconditional mappings with no tailoring: it maps one Unicode character to its lowercase equivalent according to the Unicode database and the additional complex mappings.
*/

// As an iterator:
for c in 'İ'.to_lowercase() {
    print!("{}", c);
}
println!();

// Using println! directly:
println!("{}", 'İ'.to_lowercase());

// Both are equivalent to:
println!("i\u{307}");

// Using to_string:
assert_eq!('C'.to_lowercase().to_string(), "c");

// Sometimes the result is more than one character:
assert_eq!('İ'.to_lowercase().to_string(), "i\u{307}");

// Characters that do not have both uppercase and lowercase
// convert into themselves.
assert_eq!('山'.to_lowercase().to_string(), "山");



// ! to_uppercase
fn to_uppercase(self) -> ToUppercase;
/**
|> Returns an iterator
   that yields the uppercase equivalent
   of a char as one or more chars.

If a character does not have an uppercase equivalent,
the same character will be returned back by the iterator.

This performs complex unconditional mappings with no tailoring: it maps one Unicode character to its uppercase equivalent according to the Unicode database and the additional complex mappings. */

// As an iterator:
for c in 'ß'.to_uppercase() {
    print!("{}", c);
}
println!();

// Using println! directly:
println!("{}", 'ß'.to_uppercase());

// Both are equivalent to:
println!("SS");

// Using to_string:
assert_eq!('c'.to_uppercase().to_string(), "C");

// Sometimes the result is more than one character:
assert_eq!('ß'.to_uppercase().to_string(), "SS");

// Characters that do not have both uppercase and lowercase
// convert into themselves.
assert_eq!('山'.to_uppercase().to_string(), "山");

/** Note on locale:
In Turkish, the equivalent of 'i' in Latin has five forms instead of two:
  'Dotless': I / ı, sometimes written ï
  'Dotted': İ / i
Note that the lowercase dotted 'i' is the same as the Latin.
Therefore: */

let upper_i = 'i'.to_uppercase().to_string();
/** The value of upper_i here relies on the language of the text:
if we're in en-US, it should be "I", but if we're in tr_TR,
it should be "İ". to_uppercase() does not take this into account,
and so: */
let upper_i = 'i'.to_uppercase().to_string();
assert_eq!(upper_i, "I");
// holds across languages.



// ! to_digit
fn to_digit(self, radix: u32) -> Option<u32>;
/**
Converts a char to a digit in the given radix.

A 'radix' here is sometimes also called a 'base'. A radix of two indicates a binary number, a radix of ten, decimal, and a radix of sixteen, hexadecimal, to give some common values. Arbitrary radices are supported.

'Digit' is defined to be only the following characters: 0-9 a-z A-Z

Errors:
Returns None if the char does not
refer to a digit in the given radix.

Panics
Panics if given a radix larger than 36.
*/
assert_eq!('1'.to_digit(10), Some(1));
assert_eq!('f'.to_digit(16), Some(15));

// Passing a non-digit results in failure:
assert_eq!('f'.to_digit(10), None);
assert_eq!('z'.to_digit(16), None);

// Passing a large radix, causing a panic:
use std::thread;
let result = thread::spawn(|| {
    '1'.to_digit(37);
}).join();
assert!(result.is_err());