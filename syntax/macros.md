# Macros

Assert
- `assert`          ensure that a boolean is true at runtime
- `assert_eq`       asserts 2 expr are equal, with `PartialEq`
- `assert_ne`       asserts 2 expr are not equal, with `PartialEq`
- `debug_assert`    ensure that a boolean is true at runtime
- `debug_assert_eq` asserts 2 expressions are equal
- `debug_assert_ne` asserts 2 expressions are not equal

Write
- `print`           printing to stout
- `println`         to stout, with newline i.e. LINE FEED (\n, U+000A)
- `eprint`          printing to sterr
- `eprintln`        printing to sterr, with a newline
- `write`           write formatted data into a buffer
- `writeln`         write formatted data into a buffer, newline appended
- `format`          create a String
- `format_args`     formatted string creation and output

Source
- `file`            expands to the file name from which it was invoked
- `module_path`     expands to a string of the current module path
- `column`          expands to the column number
- `line`            expands to the line number

Utility
- `unreachable`     indicating unreachable code
- `unimplemented`   marks unfinished code
- `vec`             creates a vector
- `try`             deprecated error handling (use `?` instead)
- `concat`          concatenates literals into a static string slice
- `stringify`       stringifies its argument
- `include`         parse a file as an expression or an item
- `include_bytes`   includes a file as a reference to a byte array
- `include_str`     includes a UTF8-encoded file as a string
- `env`             inspect an environment variable at compile time
- `option_env`      optionally inspect an env var at compile time
- `thread_local`    thread local storage key of `std::thread::LocalKey`type
- `concat_idents`   concatenate identifiers into one. [LAB]
- `select`          select an event from a number of receivers. [LAB]
- `cfg`             boolean evaluation of conf flags
- `compile_error`   unconditional compilation failure
- `panic`           the entry point for panic of Rust threads
