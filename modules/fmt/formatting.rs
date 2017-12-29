// *formatting 

println!("MAX_HEALTH is {:x} in hexadecimal", MAX_HEALTH); // 64
println!("MAX_HEALTH is {:b} in binary", MAX_HEALTH);  // 1100100
println!("pi is {:e} in floating point notation", PI); // 3.14e0

/*
    o for octal
    x for lower hexadecimal
    X for upper hexadecimal
    p for a pointer
    b for binary
    e for lower exponential notation
    E for upper exponential notation
    ? for debugging purposes
*/

let dec = 3.2f32;
// should be printed out as +003.20  
println!("{}", dec);  // 3.2
println!("{:+007.2}", dec);  // +003.20
// explanation:
// +00 = literal text
// 7 = total character width of output
// .2 = 2 digits after decimal point
