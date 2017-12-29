# `mem` module
https://doc.rust-lang.org/std/mem/index.html

Module `std::mem` 1.0.0

Basic functions for dealing with memory.

This module contains functions for querying the size and
alignment of types, initializing and manipulating memory.


# Structs
`Discriminant` Opaque type representing the discriminant of an `enum`.

# Unions
`ManuallyDrop` A wrapper to inhibit compiler from automatically calling destructor of `T`.

# Functions
`swap`         Swaps the values at two mut locations, without deinitializing either one.
`size_of`      Returns the size of a type in bytes.
`size_of_val`  Returns the size of the pointed-to value in bytes.
`align_of`     Returns the ABI-required minimum alignment of a type.
`align_of_val` Returns the ABI-required minimum alignment of the type of the value that val points to.
`discriminant` Returns a value uniquely identifying the enum variant in v.
`drop`         Disposes of a value.
`forget`       Leaks a value: takes ownership and "forgets" about the value without running its destructor.
`needs_drop`   Returns whether dropping values of type T matters.
`replace`      Replaces the value at a mut location with a new one, returning the old value, without deinitializing either one.

`zeroed`         ⚠ Creates a value whose bytes are all zero.
`transmute`      ⚠ Reinterprets the bits of a value of one type as another type.
`transmute_copy` ⚠ Interprets src as having type &U, and then reads src without moving the contained value.
`uninitialized`  ⚠ Bypasses Rust's normal memory-ini checks by pretending to produce a value of type T, while doing nothing at all.
`unreachable`    ⚠ [LAB] Tells LLVM this point in code is not reachable, enabling further optimizations.

`min_align_of`     [Deprecated] Returns the ABI-required minimum alignment of a type.
`min_align_of_val` [Deprecated] Returns the ABI-required minimum alignment of the type of the value that val points to.
