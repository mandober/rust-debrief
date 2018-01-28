// raw_pointers

/**
Pointer types

All pointers in Rust are explicit first-class values.
They can be copied, stored into data structs, and returned from functions.

There are 2 varieties of pointer in Rust:

References (&T)
  These point to memory owned by some other value. A reference type is written `&T`,
  or `&'a type` when you need to specify an explicit lifetime. Copying a reference is
  a "shallow" operation: it involves only copying the pointer itself. Releasing a
  reference has no effect on the value it points to, but a reference of a temporary
  value will keep it alive during the scope of the reference itself.

Raw pointers (*const T or *mut T)
  Raw pointers are pointers without safety or liveness guarantees. Raw pointers are
  written as `*const T` or `*mut T`, for example `*const i32` means a raw pointer to
  a 32-bit integer. Copying or dropping a raw pointer has no effect on the lifecycle
  of any other value. Dereferencing a raw pointer or converting it to any other pointer
  type is an unsafe operation. Raw pointers are generally discouraged in Rust code;
  they exist to support interoperability with foreign code, and writing performance
  critical or low-level functions.

*/

// The address of a value is given by the & operator
let mut game = "Space Invaders";
let health = 32;

// print these addresses by using the format string {:p} for pointers:
println!("address of health-value: {:p}", &health); // prints 0x23fba4
println!("address of game-value: {:p}", &game);     // prints 0x23fb90
println!("game-value: {}", game);       // prints "Space Invaders"
println!("game: {}", &game);            // prints "Space Invaders"

// &health is the address where value 32 is stored, and
// &game is the address where the Space Invaders' value is stored.

// We can make an alias, which is another reference
// that points to the same place in memory, like this:
let game2 = &game;
println!("{:p}", game2); // prints 0x23fb90

// To get the value that is being referred to rather than the game2
// reference itself, dereference it with the asterisk * operator like this:
println!("{}", *game2); // prints "Space Invaders"
println!("{}", game2);  // prints "Space Invaders"
// println! is doing automatic dereferencing


// *boxing values onto the heap:
// let x = box 5i32;
// error: box expression syntax is experimental; you can call `Box::new` instead.
let x = Box::new(5i32);
