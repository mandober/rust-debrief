// *Casting
// Cast between types with as:
let x: i32 = 100;
let y: u32 = x as u32;

// Naturally, you can only cast between types that are safe to cast between.
// No casting [i16; 4] to char! (This is called a "non-scalar" cast)
// There are unsafe mechanisms to overcome this, if you know what you're doing.

// explicit casting with `as` keyword:
let points = 10i32;
let mut saved_points: u32 = 0;
saved_points = points as u32;
// if points (i32) was negative, the sign would be lost after casting to u 32
// when casting from a wider value like a float to an integer, the decimal part is truncated:
let f2 = 3.14;
saved_points = f2 as u32; // truncation to value 3 occurs here

