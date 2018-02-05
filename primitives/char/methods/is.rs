// ! `is_*` methods

fn is_alphabetic(self) -> bool;
fn is_uppercase(self) -> bool;
fn is_lowercase(self) -> bool;
fn is_alphanumeric(self) -> bool;
fn is_numeric(self) -> bool;
fn is_digit(self, radix: u32) -> bool;
fn is_whitespace(self) -> bool;
fn is_control(self) -> bool;
fn is_xid_start(self) -> bool; // [LAB]
fn is_xid_continue(self) -> bool; // [LAB]




// ! is_digit
fn is_digit(self, radix: u32) -> bool;
/**
Checks if a char is a digit in the given radix.

A 'radix' here is sometimes also called a 'base'. A radix of two indicates a binary number, a radix of ten, decimal, and a radix of sixteen, hexadecimal, to give some common values. Arbitrary radices are supported.

Compared to `is_numeric()`, this function only
recognizes the characters 0-9, a-z and A-Z.

'Digit' is defined to be only the
following characters: 0-9 a-z A-Z

PANICS:
Panics if given a radix larger than 36.

EXAMPLES:
*/
assert!('1'.is_digit(10));
assert!('f'.is_digit(16));
assert!(!'f'.is_digit(10));

// Passing a large radix, causing a panic:
use std::thread;
let result = thread::spawn(|| {
    // this panics
    '1'.is_digit(37);
}).join();
assert!(result.is_err());




// ! is_alphabetic
fn is_alphabetic(self) -> bool;
/**
Returns true if this char is an
alphabetic code point, and false if not.

Examples: */
assert!('a'.is_alphabetic());
assert!('京'.is_alphabetic());

let c = '💝';
// love is many things, but it is not alphabetic
assert!(!c.is_alphabetic());




// ! is_lowercase
fn is_lowercase(self) -> bool;
/**
Returns true if this char is lowercase, and false otherwise.

'Lowercase' is defined according to the terms of the
Unicode Derived Core Property Lowercase.

Examples: */
assert!('a'.is_lowercase());
assert!('δ'.is_lowercase());
assert!(!'A'.is_lowercase());
assert!(!'Δ'.is_lowercase());

// The various Chinese scripts do not have case, and so:
assert!(!'中'.is_lowercase());




// ! is_uppercase
fn is_uppercase(self) -> bool;
/**
Returns true if this char is uppercase, and false otherwise.

'Uppercase' is defined according to the terms of the
Unicode Derived Core Property Uppercase.

Examples: */
assert!(!'a'.is_uppercase());
assert!(!'δ'.is_uppercase());
assert!('A'.is_uppercase());
assert!('Δ'.is_uppercase());

// The various Chinese scripts do not have case, and so:
assert!(!'中'.is_uppercase());



// ! is_whitespace
fn is_whitespace(self) -> bool;
/**
Returns true if this char is whitespace, and false otherwise.

'Whitespace' is defined according to the terms of the
Unicode Derived Core Property White_Space.

Examples: */
assert!(' '.is_whitespace());
// a non-breaking space
assert!('\u{A0}'.is_whitespace());
assert!(!'越'.is_whitespace());



// ! is_alphanumeric
fn is_alphanumeric(self) -> bool;
/**
Returns true if this char is alphanumeric, and false otherwise.

'Alphanumeric'-ness is defined in terms of the
Unicode General Categories 'Nd', 'Nl', 'No' and
the Derived Core Property 'Alphabetic'.

Examples: */
assert!('٣'.is_alphanumeric());
assert!('7'.is_alphanumeric());
assert!('৬'.is_alphanumeric());
assert!('K'.is_alphanumeric());
assert!('و'.is_alphanumeric());
assert!('藏'.is_alphanumeric());
assert!(!'¾'.is_alphanumeric());
assert!(!'①'.is_alphanumeric());



// ! is_control
fn is_control(self) -> bool;
/**
Returns true if this char is a control
code point, and false otherwise.

'Control code point' is defined in terms of the
Unicode General Category Cc.

Examples: */

// U+009C, STRING TERMINATOR
assert!(''.is_control());
assert!(!'q'.is_control());



// ! is_numeric
fn is_numeric(self) -> bool;
/**
Returns true if this char is numeric, and false otherwise.

'Numeric'-ness is defined in terms of the Unicode General Categories 'Nd', 'Nl', 'No'.

Examples: */
assert!('٣'.is_numeric());
assert!('7'.is_numeric());
assert!('৬'.is_numeric());
assert!(!'K'.is_numeric());
assert!(!'و'.is_numeric());
assert!(!'藏'.is_numeric());
assert!(!'¾'.is_numeric());
assert!(!'①'.is_numeric());


// ! is_xid_start
fn is_xid_start(self) -> bool;
/**
This is a nightly-only experimental API.
(rustc_private #27812)

Returns true if this char satisfies the 'XID_Start' Unicode property, and false otherwise.

'XID_Start' is a Unicode Derived Property specified in UAX #31, mostly similar to ID_Start but modified for closure under NFKx.
*/


// ! is_xid_continue
fn is_xid_continue(self) -> bool;
/**
This is a nightly-only experimental API.
(rustc_private #27812)

Returns true if this char satisfies the 'XID_Continue' Unicode property, and false otherwise.

'XID_Continue' is a Unicode Derived Property specified in UAX #31, mostly similar to 'ID_Continue' but modified for closure under NFKx.
*/