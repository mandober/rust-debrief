//! char Methods: encode_*

fn encode_utf8(self, dst: &mut [u8]) -> &mut str;
fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];



// ! encode_utf8
fn encode_utf8(self, dst: &mut [u8]) -> &mut str;
/**
1.15.0

Encodes this character as UTF-8 into the provided byte buffer, and then returns the subslice of the buffer that contains the encoded character.
Panics

Panics if the buffer is not large enough. A buffer of length four is large enough to encode any char.

Examples: */

// In both of these examples,
// 'ß' takes two bytes to encode.
let mut b = [0; 2];
let result = 'ß'.encode_utf8(&mut b);
assert_eq!(result, "ß");
assert_eq!(result.len(), 2);

// A buffer that's too small:
use std::thread;
let result = thread::spawn(|| {
    let mut b = [0; 1];
    // this panics
   'ß'.encode_utf8(&mut b);
}).join();
assert!(result.is_err());




// ! encode_utf16
fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];
/**
1.15.0

Encodes this character as UTF-16 into the provided u16 buffer, and then returns the subslice of the buffer that contains the encoded character.

PANICS:
Panics if the buffer is not large enough. A buffer of length 2 is large enough to encode any char.

EXAMPLES:
In both of these examples, '𝕊' takes two u16s to encode.
*/
let mut b = [0; 2];
let result = '𝕊'.encode_utf16(&mut b);
assert_eq!(result.len(), 2);

// A buffer that's too small:
use std::thread;
let result = thread::spawn(|| {
    let mut b = [0; 1];
    // this panics
    '𝕊'.encode_utf16(&mut b);
}).join();
assert!(result.is_err());