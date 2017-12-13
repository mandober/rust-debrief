# Macros

`assert`          ensure that a boolean expression is true at runtime.
`assert_eq`       asserts 2 expr are equal to each other, with `PartialEq`.
`assert_ne`       asserts 2 expr are not equal to each other, with `PartialEq`.
`debug_assert`    ensure that a boolean expression is true at runtime.
`debug_assert_eq` asserts 2 expressions are equal to each other.
`debug_assert_ne` asserts 2 expressions are not equal to each other.
`print`           printing to stout.
`println`         to stout, with newline i.e. LINE FEED (\n, U+000A)
`eprint`          printing to sterr.
`eprintln`        printing to sterr, with a newline.
`write`           write formatted data into a buffer
`writeln`         write formatted data into a buffer, with a newline appended.
`format`          create String. See std::fmt for more information.
`format_args`     formatted string creation and output.
`concat`          concatenates literals into a static string slice.
`vec`             creates a Vec containing the arguments.
`include`         parse a file as an expression or an item.
`include_bytes`   includes a file as a reference to a byte array.
`include_str`     includes a UTF8-encoded file as a string.
`env`             inspect an environment variable at compile time.
`option_env`      optionally inspect an environment variable at compile time.
`cfg`             boolean evaluation of configuration flags.
`column`          expands to the column number on which it was invoked.
`compile_error`   unconditional compilation failure.
`file`            expands to the file name from which it was invoked.
`line`            expands to the line number on which it was invoked.
`module_path`     expands to a string of the current module path.
`panic`           the entry point for panic of Rust threads.
`stringify`       stringifies its argument.
`thread_local`    new thread local storage key of `std::thread::LocalKey` type.
`concat_idents`   concatenate identifiers into one. [LAB]
`select`          select an event from a number of receivers. [LAB]
`try`             matching Result together with converting downstream errors.
`unreachable`     indicating unreachable code.
`unimplemented`   mark unfinished code. It panics with "not yet implemented".
