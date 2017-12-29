# Module `std::fs` 1.0.0

Filesystem manipulation operations.

Contains basic methods to manipulate the contents of the local filesystem.
All methods in this module represent cross-platform filesystem operations.
Extra platform-specific functionality can be found in the extension traits of 
`std::os::$platform`.


## Structs
`File`        reference to an open file on the filesystem.
`FileType`    struct repr. a file type with accessors for each file type, returned by `Metadata::file_type` method.
`Metadata`    Metadata information about a file.
`OpenOptions` Options and flags which can be used to configure how a file is opened.
`Permissions` Representation of the various permissions on a file.
`DirBuilder`  A builder used to create directories in various manners.
`ReadDir`     Iterator over the entries in a directory.
`DirEntry`    Entries returned by the `ReadDir` iterator.


## Functions
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
