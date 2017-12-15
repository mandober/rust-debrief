// ! String methods:

fn as_bytes(&self) -> &[u8];
// Returns a byte slice of this String's contents.

fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>;
// Converts a vector of bytes to a String.
