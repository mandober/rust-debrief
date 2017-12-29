Module `std::fs` 1.0.0

# Functions
`canonicalize`     Returns canonical path with all intermediate components normalized and symlinks resolved.
`copy`             Copies the contents of one file to another; also copy the perm bits of the src to dest file.
`create_dir`       Creates a new, empty directory at the provided path
`create_dir_all`   Recursively create a directory and all of its parent components if they are missing.
`hard_link`        Creates a new hard link on the filesystem.
`metadata`         Given a path, query the file system to get information about a file, directory, etc.
`read_dir`         Returns an iterator over the entries within a directory.
`read_link`        Reads a symbolic link, returning the file that the link points to.
`remove_dir`       Removes an existing, empty directory.
`remove_dir_all`   Removes a directory at this path, after removing all its contents. Use carefully!
`remove_file`      Removes a file from the filesystem.
`rename`           Rename a file or directory to a new name, replacing the original file if to already exists.
`set_permissions`  Changes the permissions found on a file or a directory.
`soft_link`        Creates a new symbolic link on the filesystem. [Deprecated]
`symlink_metadata` Query the metadata about a file without following symlinks.
