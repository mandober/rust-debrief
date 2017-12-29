# Function `std::fs::read_dir` 1.0.0

```rust
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>
```

Returns an iterator over the entries within a directory.

The iterator will yield instances of `io::Result<DirEntry>`.
New errors may be encountered after an iterator is initially constructed.

## Platform-specific behavior
This function currently corresponds to the opendir function on Unix and the 
FindFirstFile function on Windows. Note that, this may change in the future.

## Errors
This function will return an error in the following situations, 
but is not limited to just these cases:
- The provided path doesn't exist.
- The process lacks permissions to view the contents.
- The path points at a non-directory file.


## Examples:

```rust
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
```
