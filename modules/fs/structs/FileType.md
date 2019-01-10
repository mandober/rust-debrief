# Struct `std::fs::FileType` 1.1.0

```rust
pub struct FileType(_);
```
A structure representing a type of file with accessors for each file type.
It is returned by `Metadata::file_type` method.


## Methods Index

`is_dir()`
`is_file()`
`is_symlink()`


## Methods

Test whether this file type represents a directory.

```rust
impl FileType {
    pub fn is_dir(&self) -> bool;
}

// EXAMPLE: 
use std::fs;
let metadata = fs::metadata("foo.txt")?;
let file_type = metadata.file_type();
assert_eq!(file_type.is_dir(), false);
```



Test whether this file type represents a regular file.

```rust
impl FileType {
    pub fn is_file(&self) -> bool;
}

// EXAMPLE: 
use std::fs;
let metadata = fs::metadata("foo.txt")?;
let file_type = metadata.file_type();
assert_eq!(file_type.is_file(), true);
```



Test whether this file type represents a symbolic link.

```rust
impl FileType {
    pub fn is_symlink(&self) -> bool
}
```
The underlying Metadata struct needs to be retrieved with `fs::symlink_metadata` 
function and not the `fs::metadata` function. The `fs::metadata` function follows 
symbolic links, so `is_symlink` would always return false for the target file.

```rust
use std::fs;
let metadata = fs::symlink_metadata("foo.txt")?;
let file_type = metadata.file_type();
assert_eq!(file_type.is_symlink(), false);
```


## Trait Implementations Index
Copy, Clone, Debug, PartialEq, Eq, Hash, 
FileTypeExt: `is_block_device`, `is_char_device`, `is_fifo`, `is_socket`



## Trait Implementations

```rust
impl Copy for FileType {
impl Clone for FileType {
    fn clone(&self) -> FileType;
    // Returns a copy of the value.

    fn clone_from(&mut self, source: &Self); // 1.0.0
    // Performs copy-assignment from source
}};


impl PartialEq for FileType {
    fn eq(&self, __arg_0: &FileType) -> bool;
    // tests for self and other values to be equal, and is used by ==

    fn ne(&self, __arg_0: &FileType) -> bool;
    // tests for !=
};


impl Eq for FileType {
impl Hash for FileType {
    fn hash<__H: Hasher>(&self, __arg_0: &mut __H);
    // Feeds this value into the given [Hasher]

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where H: Hasher; // 1.3.0
    // Feeds a slice of this type into the given [Hasher]
}};


impl Debug for FileType {
    fn fmt(&self, __arg_0: &mut Formatter) -> Result
    // Formats the value using the given formatter.
};



// Unix only:
impl FileTypeExt for FileType { // 1.5.0
    fn is_block_device(&self) -> bool;
    // [Unix only] Returns whether this file type is a block device.

    fn is_char_device(&self) -> bool;
    // [Unix only] Returns whether this file type is a char device.

    fn is_fifo(&self) -> bool;
    // [Unix only] Returns whether this file type is a fifo.

    fn is_socket(&self) -> bool;
    // [Unix only] Returns whether this file type is a socket.
}
```
