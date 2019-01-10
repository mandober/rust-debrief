// raw_pointers


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
