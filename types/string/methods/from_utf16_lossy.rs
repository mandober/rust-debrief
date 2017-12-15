fn from_utf16_lossy(v: &[u16]) -> String[src]
[−]

Decode a UTF-16 encoded slice v into a String, replacing invalid data with the replacement character (U+FFFD).
Examples

Basic usage:

// 𝄞mus<invalid>ic<invalid>
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0x0073, 0xDD1E, 0x0069, 0x0063,
          0xD834];

assert_eq!(String::from("𝄞mus\u{FFFD}ic\u{FFFD}"),
           String::from_utf16_lossy(v));